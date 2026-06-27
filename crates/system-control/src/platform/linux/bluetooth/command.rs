use std::sync::Arc;

use bluer::{Adapter, AdapterEvent};
use futures_util::StreamExt;
use tokio::sync::mpsc::UnboundedSender;

use super::device::device_from_identifier;
use super::events::spawn_device_watcher;
use super::runtime::RuntimeTaskState;
use super::state::{
    apply_pending_operation, finish_operation_with_error, refresh_adapter_state, refresh_device,
};
use super::store::BackendState;
use crate::bluetooth::{BluetoothDeviceId, BluetoothOperationId, BluetoothOperationKind};

pub(super) enum RuntimeMessage {
    Command(BluetoothCommand),
    AdapterEvent {
        connection_id: u64,
        event: AdapterEvent,
    },
    AdapterEventsEnded {
        connection_id: u64,
    },
    DeviceEvent {
        connection_id: u64,
        device_identifier: BluetoothDeviceId,
    },
    DeviceEventsEnded {
        connection_id: u64,
        device_identifier: BluetoothDeviceId,
    },
    DiscoveryEvent {
        connection_id: u64,
        event: AdapterEvent,
    },
    DiscoveryEnded {
        connection_id: u64,
    },
    Shutdown,
}

pub(super) enum BluetoothCommand {
    SetPowerEnabled {
        operation_id: BluetoothOperationId,
        is_enabled: bool,
    },
    StartDiscovery {
        operation_id: BluetoothOperationId,
    },
    StopDiscovery {
        operation_id: BluetoothOperationId,
    },
    ConnectDevice {
        operation_id: BluetoothOperationId,
        device_identifier: BluetoothDeviceId,
    },
    DisconnectDevice {
        operation_id: BluetoothOperationId,
        device_identifier: BluetoothDeviceId,
    },
}

impl BluetoothCommand {
    pub(super) fn operation_id(&self) -> BluetoothOperationId {
        match self {
            Self::SetPowerEnabled { operation_id, .. }
            | Self::StartDiscovery { operation_id }
            | Self::StopDiscovery { operation_id }
            | Self::ConnectDevice { operation_id, .. }
            | Self::DisconnectDevice { operation_id, .. } => *operation_id,
        }
    }
}

pub(super) async fn handle_command(
    inner: &Arc<BackendState>,
    adapter: &Adapter,
    runtime_sender: &UnboundedSender<RuntimeMessage>,
    runtime_state: &Arc<RuntimeTaskState>,
    connection_id: u64,
    command: BluetoothCommand,
) {
    apply_pending_operation(
        inner,
        connection_id,
        command.operation_id(),
        command.pending_operation(),
    );

    match command {
        BluetoothCommand::SetPowerEnabled {
            operation_id,
            is_enabled,
        } => match adapter.set_powered(is_enabled).await {
            Ok(()) => {
                if !is_enabled && let Some(task) = runtime_state.discovery_task.lock().await.take()
                {
                    task.abort();
                }
                refresh_adapter_state(inner, adapter, connection_id, Some(operation_id), None)
                    .await;
            }
            Err(error) => {
                finish_operation_with_error(
                    inner,
                    connection_id,
                    Some(operation_id),
                    error.to_string(),
                );
            }
        },
        BluetoothCommand::StartDiscovery { operation_id } => {
            let mut discovery_task = runtime_state.discovery_task.lock().await;
            if discovery_task.is_none() {
                match adapter.discover_devices_with_changes().await {
                    Ok(mut discovery_stream) => {
                        let sender = runtime_sender.clone();
                        *discovery_task = Some(tokio::spawn(async move {
                            while let Some(event) = discovery_stream.next().await {
                                if sender
                                    .send(RuntimeMessage::DiscoveryEvent {
                                        connection_id,
                                        event,
                                    })
                                    .is_err()
                                {
                                    return;
                                }
                            }
                            let _ = sender.send(RuntimeMessage::DiscoveryEnded { connection_id });
                        }));
                        drop(discovery_task);
                        refresh_adapter_state(
                            inner,
                            adapter,
                            connection_id,
                            Some(operation_id),
                            None,
                        )
                        .await;
                    }
                    Err(error) => {
                        drop(discovery_task);
                        finish_operation_with_error(
                            inner,
                            connection_id,
                            Some(operation_id),
                            error.to_string(),
                        );
                    }
                }
            } else {
                drop(discovery_task);
                refresh_adapter_state(inner, adapter, connection_id, Some(operation_id), None)
                    .await;
            }
        }
        BluetoothCommand::StopDiscovery { operation_id } => {
            if let Some(task) = runtime_state.discovery_task.lock().await.take() {
                task.abort();
            }
            refresh_adapter_state(inner, adapter, connection_id, Some(operation_id), None).await;
        }
        BluetoothCommand::ConnectDevice {
            operation_id,
            device_identifier,
        } => match device_from_identifier(adapter, &device_identifier) {
            Ok(device) => match device.connect().await {
                Ok(()) => {
                    let mut device_watchers = runtime_state.device_watchers.lock().await;
                    spawn_device_watcher(
                        adapter,
                        runtime_sender,
                        &mut device_watchers,
                        connection_id,
                        device_identifier.clone(),
                    );
                    refresh_device(
                        adapter,
                        inner,
                        connection_id,
                        device_identifier,
                        Some(operation_id),
                        None,
                    )
                    .await;
                }
                Err(error) => {
                    finish_operation_with_error(
                        inner,
                        connection_id,
                        Some(operation_id),
                        error.to_string(),
                    );
                }
            },
            Err(error) => {
                finish_operation_with_error(
                    inner,
                    connection_id,
                    Some(operation_id),
                    error.to_string(),
                );
            }
        },
        BluetoothCommand::DisconnectDevice {
            operation_id,
            device_identifier,
        } => match device_from_identifier(adapter, &device_identifier) {
            Ok(device) => match device.disconnect().await {
                Ok(()) => {
                    refresh_device(
                        adapter,
                        inner,
                        connection_id,
                        device_identifier,
                        Some(operation_id),
                        None,
                    )
                    .await;
                }
                Err(error) => {
                    finish_operation_with_error(
                        inner,
                        connection_id,
                        Some(operation_id),
                        error.to_string(),
                    );
                }
            },
            Err(error) => {
                finish_operation_with_error(
                    inner,
                    connection_id,
                    Some(operation_id),
                    error.to_string(),
                );
            }
        },
    }
}

trait IntoPendingOperation {
    fn pending_operation(&self) -> BluetoothOperationKind;
}

impl IntoPendingOperation for BluetoothCommand {
    fn pending_operation(&self) -> BluetoothOperationKind {
        match self {
            Self::SetPowerEnabled { is_enabled, .. } => BluetoothOperationKind::SetPowerEnabled {
                is_enabled: *is_enabled,
            },
            Self::StartDiscovery { .. } => BluetoothOperationKind::StartDiscovery,
            Self::StopDiscovery { .. } => BluetoothOperationKind::StopDiscovery,
            Self::ConnectDevice {
                device_identifier, ..
            } => BluetoothOperationKind::ConnectDevice {
                device_identifier: device_identifier.clone(),
            },
            Self::DisconnectDevice {
                device_identifier, ..
            } => BluetoothOperationKind::DisconnectDevice {
                device_identifier: device_identifier.clone(),
            },
        }
    }
}
