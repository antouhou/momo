use super::{
    device::{load_device, sort_devices},
    store::BackendState,
};
use crate::{
    bluetooth::{
        BluetoothAdapterState, BluetoothCapabilities, BluetoothConnectionState, BluetoothDevice,
        BluetoothDeviceId, BluetoothDiscoveryState, BluetoothOperationId, BluetoothOperationKind,
        BluetoothPendingOperation, BluetoothPowerState, BluetoothState, BluetoothUserVisibleError,
    },
    feature_state::FeatureState,
};
use bluer::Adapter;
use std::sync::Arc;

pub(super) async fn load_current_state(adapter: &Adapter) -> bluer::Result<BluetoothState> {
    let adapter_identifier = adapter.name().to_string();
    let adapter_name = Some(adapter.name().to_string());
    let is_powered = adapter.is_powered().await?;
    let is_discovering = adapter.is_discovering().await?;

    let mut devices = Vec::new();
    for address in adapter.device_addresses().await? {
        if let Some(device) = load_device(adapter, address.to_string()).await {
            devices.push(device);
        }
    }
    sort_devices(&mut devices);

    Ok(BluetoothState {
        adapter: BluetoothAdapterState {
            adapter_identifier,
            adapter_name,
            power_state: if is_powered {
                BluetoothPowerState::On
            } else {
                BluetoothPowerState::Off
            },
            discovery_state: if is_discovering {
                BluetoothDiscoveryState::Scanning
            } else {
                BluetoothDiscoveryState::Idle
            },
            capabilities: BluetoothCapabilities {
                can_change_power: true,
                can_start_discovery: true,
                can_connect_devices: true,
            },
        },
        devices,
        pending_operations: Vec::new(),
        last_error: None,
        revision: 0,
    })
}

pub(super) async fn refresh_adapter_state(
    inner: &Arc<BackendState>,
    adapter: &Adapter,
    connection_id: u64,
    completed_operation_id: Option<BluetoothOperationId>,
    last_error: Option<BluetoothUserVisibleError>,
) {
    match load_current_state(adapter).await {
        Ok(mut next_state) => {
            if !inner.is_active_connection(connection_id) {
                return;
            }
            let current_state = inner.current_state();
            let FeatureState::Ready(current_ready_state) = current_state else {
                return;
            };

            finalize_ready_state(
                &mut next_state,
                &current_ready_state,
                completed_operation_id,
                last_error,
            );
            inner.publish(FeatureState::Ready(next_state));
        }
        Err(error) => {
            finish_operation_with_error(
                inner,
                connection_id,
                completed_operation_id,
                error.to_string(),
            );
        }
    }
}

pub(super) async fn refresh_device(
    adapter: &Adapter,
    inner: &Arc<BackendState>,
    connection_id: u64,
    device_identifier: BluetoothDeviceId,
    completed_operation_id: Option<BluetoothOperationId>,
    next_error: Option<BluetoothUserVisibleError>,
) -> bool {
    let updated_device = load_device(adapter, device_identifier.0.clone()).await;
    if !inner.is_active_connection(connection_id) {
        return false;
    }
    let current_state = inner.current_state();
    let FeatureState::Ready(mut ready_state) = current_state else {
        return false;
    };

    let is_device_present = apply_device_refresh_result(
        &mut ready_state,
        &device_identifier,
        updated_device,
        completed_operation_id,
    );

    let current_ready_state = ready_state.clone();
    finalize_ready_state(
        &mut ready_state,
        &current_ready_state,
        completed_operation_id,
        next_error,
    );
    inner.publish(FeatureState::Ready(ready_state));

    is_device_present
}

pub(super) async fn refresh_device_from_identifier(
    inner: &Arc<BackendState>,
    adapter: &Adapter,
    connection_id: u64,
    device_identifier: BluetoothDeviceId,
) {
    refresh_device(adapter, inner, connection_id, device_identifier, None, None).await;
}

fn apply_device_refresh_result(
    ready_state: &mut BluetoothState,
    device_identifier: &BluetoothDeviceId,
    updated_device: Option<BluetoothDevice>,
    completed_operation_id: Option<BluetoothOperationId>,
) -> bool {
    let completed_operation_kind = completed_operation_id.and_then(|operation_id| {
        ready_state
            .pending_operations
            .iter()
            .find(|operation| operation.operation_id == operation_id)
            .map(|operation| operation.kind.clone())
    });

    if let Some(updated_device) = updated_device {
        upsert_device(&mut ready_state.devices, updated_device);
        return true;
    }

    if matches!(
        completed_operation_kind,
        Some(BluetoothOperationKind::DisconnectDevice {
            device_identifier: ref pending_device_identifier,
        }) if pending_device_identifier == device_identifier
    ) && let Some(device) = ready_state
        .devices
        .iter_mut()
        .find(|device| &device.device_identifier == device_identifier)
    {
        device.connection_state = BluetoothConnectionState::Disconnected;
        device.signal_strength_dbm = None;
    }

    false
}

fn finalize_ready_state(
    next_state: &mut BluetoothState,
    current_ready_state: &BluetoothState,
    completed_operation_id: Option<BluetoothOperationId>,
    next_error: Option<BluetoothUserVisibleError>,
) {
    next_state.pending_operations = current_ready_state
        .pending_operations
        .iter()
        .filter(|operation| Some(operation.operation_id) != completed_operation_id)
        .cloned()
        .collect();
    next_state.last_error = next_error.or(current_ready_state.last_error.clone());
    next_state.revision = current_ready_state.revision.saturating_add(1);
    apply_pending_states(next_state);
}

pub(super) fn apply_pending_operation(
    inner: &Arc<BackendState>,
    connection_id: u64,
    operation_id: BluetoothOperationId,
    operation_kind: BluetoothOperationKind,
) {
    if !inner.is_active_connection(connection_id) {
        return;
    }

    let current_state = inner.current_state();
    let FeatureState::Ready(mut ready_state) = current_state else {
        return;
    };

    ready_state
        .pending_operations
        .push(BluetoothPendingOperation {
            operation_id,
            kind: operation_kind,
        });
    ready_state.last_error = None;
    ready_state.revision = ready_state.revision.saturating_add(1);
    apply_pending_states(&mut ready_state);
    inner.publish(FeatureState::Ready(ready_state));
}

fn apply_pending_states(state: &mut BluetoothState) {
    for operation in &state.pending_operations {
        match &operation.kind {
            BluetoothOperationKind::SetPowerEnabled { is_enabled } => {
                state.adapter.power_state = if *is_enabled {
                    BluetoothPowerState::TurningOn {
                        operation_id: operation.operation_id,
                    }
                } else {
                    BluetoothPowerState::TurningOff {
                        operation_id: operation.operation_id,
                    }
                };
            }
            BluetoothOperationKind::StartDiscovery => {
                state.adapter.discovery_state = BluetoothDiscoveryState::Starting {
                    operation_id: operation.operation_id,
                };
            }
            BluetoothOperationKind::StopDiscovery => {
                state.adapter.discovery_state = BluetoothDiscoveryState::Stopping {
                    operation_id: operation.operation_id,
                };
            }
            BluetoothOperationKind::ConnectDevice { device_identifier } => {
                if let Some(device) = state
                    .devices
                    .iter_mut()
                    .find(|device| &device.device_identifier == device_identifier)
                {
                    device.connection_state = BluetoothConnectionState::Connecting {
                        operation_id: operation.operation_id,
                    };
                }
            }
            BluetoothOperationKind::DisconnectDevice { device_identifier } => {
                if let Some(device) = state
                    .devices
                    .iter_mut()
                    .find(|device| &device.device_identifier == device_identifier)
                {
                    device.connection_state = BluetoothConnectionState::Disconnecting {
                        operation_id: operation.operation_id,
                    };
                }
            }
        }
    }
}

pub(super) fn finish_operation_with_error(
    inner: &Arc<BackendState>,
    connection_id: u64,
    operation_id: Option<BluetoothOperationId>,
    message: String,
) {
    if !inner.is_active_connection(connection_id) {
        return;
    }

    let current_state = inner.current_state();
    let FeatureState::Ready(mut ready_state) = current_state else {
        return;
    };

    if let Some(operation_id) = operation_id {
        ready_state
            .pending_operations
            .retain(|operation| operation.operation_id != operation_id);
    }
    ready_state.last_error = Some(BluetoothUserVisibleError {
        operation_id,
        message,
    });
    ready_state.revision = ready_state.revision.saturating_add(1);
    apply_pending_states(&mut ready_state);
    inner.publish(FeatureState::Ready(ready_state));
}

fn upsert_device(devices: &mut Vec<BluetoothDevice>, updated_device: BluetoothDevice) {
    if let Some(existing_device) = devices
        .iter_mut()
        .find(|device| device.device_identifier == updated_device.device_identifier)
    {
        *existing_device = updated_device;
    } else {
        devices.push(updated_device);
    }
    sort_devices(devices);
}
