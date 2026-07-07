use super::{
    command::{RuntimeMessage, handle_command},
    events::{handle_adapter_event, spawn_adapter_watcher, spawn_device_watcher},
    state::{load_current_state, refresh_device_from_identifier},
    store::BackendState,
};
use crate::{
    bluetooth::{
        BluetoothDeviceId, BluetoothState, BluetoothUnavailableReason, BluetoothUnsupportedReason,
    },
    feature_state::FeatureState,
};
use bluer::{Adapter, Session};
use std::{collections::BTreeMap, sync::Arc, time::Duration};
use tokio::sync::{
    Mutex as AsyncMutex,
    mpsc::{UnboundedReceiver, UnboundedSender},
};
use tracing::{debug, warn};

const BLUETOOTH_RECONNECT_DELAY: Duration = Duration::from_secs(2);

pub(super) struct RuntimeTaskState {
    pub(super) device_watchers:
        AsyncMutex<BTreeMap<BluetoothDeviceId, tokio::task::JoinHandle<()>>>,
    pub(super) discovery_task: AsyncMutex<Option<tokio::task::JoinHandle<()>>>,
}

struct ConnectedBluetoothRuntime {
    _session: Session,
    adapter: Arc<Adapter>,
    runtime_state: Arc<RuntimeTaskState>,
}

pub(super) enum BluetoothConnectionError {
    NoAdapter,
    BackendUnavailable { message: String },
}

enum ConnectedRuntimeExit {
    Reconnect,
    Shutdown,
}

pub(super) async fn run_linux_bluetooth_runtime(
    inner: Arc<BackendState>,
    runtime_sender: UnboundedSender<RuntimeMessage>,
    mut command_receiver: UnboundedReceiver<RuntimeMessage>,
) {
    let mut connection_id = 1;

    loop {
        inner.clear_active_connection();

        match connect_bluetooth_runtime(&inner, &runtime_sender, connection_id).await {
            Ok(runtime) => {
                match run_connected_bluetooth_runtime(
                    &inner,
                    &runtime_sender,
                    &mut command_receiver,
                    connection_id,
                    runtime,
                )
                .await
                {
                    ConnectedRuntimeExit::Reconnect => {
                        connection_id = connection_id.wrapping_add(1);
                    }
                    ConnectedRuntimeExit::Shutdown => break,
                }
            }
            Err(error) => {
                publish_bluetooth_connection_error(&inner, error);
                connection_id = connection_id.wrapping_add(1);
            }
        }

        if !wait_for_bluetooth_reconnect_delay(&mut command_receiver).await {
            break;
        }
    }
}

async fn connect_bluetooth_runtime(
    inner: &Arc<BackendState>,
    runtime_sender: &UnboundedSender<RuntimeMessage>,
    connection_id: u64,
) -> Result<ConnectedBluetoothRuntime, BluetoothConnectionError> {
    let session = match bluer::Session::new().await {
        Ok(session) => session,
        Err(error) => {
            return Err(BluetoothConnectionError::BackendUnavailable {
                message: error.to_string(),
            });
        }
    };

    let adapter = match session.default_adapter().await {
        Ok(adapter) => adapter,
        Err(error) => {
            if error.kind == bluer::ErrorKind::NotFound {
                return Err(BluetoothConnectionError::NoAdapter);
            } else {
                return Err(BluetoothConnectionError::BackendUnavailable {
                    message: error.to_string(),
                });
            }
        }
    };

    let adapter = Arc::new(adapter);
    let runtime_state = Arc::new(RuntimeTaskState {
        device_watchers: AsyncMutex::new(BTreeMap::new()),
        discovery_task: AsyncMutex::new(None),
    });

    let initial_state = match load_current_state(adapter.as_ref()).await {
        Ok(state) => state,
        Err(error) => {
            return Err(BluetoothConnectionError::BackendUnavailable {
                message: error.to_string(),
            });
        }
    };
    spawn_adapter_watcher(adapter.as_ref(), runtime_sender, connection_id).await?;

    let mut device_watchers = runtime_state.device_watchers.lock().await;
    for device in &initial_state.devices {
        spawn_device_watcher(
            adapter.as_ref(),
            runtime_sender,
            &mut device_watchers,
            connection_id,
            device.device_identifier.clone(),
        );
    }
    drop(device_watchers);
    inner.activate_connection(connection_id);
    inner.publish(FeatureState::Ready(BluetoothState {
        revision: 1,
        ..initial_state
    }));

    Ok(ConnectedBluetoothRuntime {
        _session: session,
        adapter,
        runtime_state,
    })
}

async fn run_connected_bluetooth_runtime(
    inner: &Arc<BackendState>,
    runtime_sender: &UnboundedSender<RuntimeMessage>,
    command_receiver: &mut UnboundedReceiver<RuntimeMessage>,
    connection_id: u64,
    runtime: ConnectedBluetoothRuntime,
) -> ConnectedRuntimeExit {
    while let Some(message) = command_receiver.recv().await {
        match message {
            RuntimeMessage::Command(command) => {
                let inner = Arc::clone(inner);
                let adapter = Arc::clone(&runtime.adapter);
                let runtime_sender = runtime_sender.clone();
                let runtime_state = Arc::clone(&runtime.runtime_state);
                tokio::spawn(async move {
                    handle_command(
                        &inner,
                        adapter.as_ref(),
                        &runtime_sender,
                        &runtime_state,
                        connection_id,
                        command,
                    )
                    .await;
                });
            }
            RuntimeMessage::AdapterEvent {
                connection_id: event_connection_id,
                event,
            }
            | RuntimeMessage::DiscoveryEvent {
                connection_id: event_connection_id,
                event,
            } => {
                if event_connection_id != connection_id {
                    continue;
                }
                let inner = Arc::clone(inner);
                let adapter = Arc::clone(&runtime.adapter);
                let runtime_sender = runtime_sender.clone();
                let runtime_state = Arc::clone(&runtime.runtime_state);
                tokio::spawn(async move {
                    handle_adapter_event(
                        &inner,
                        adapter.as_ref(),
                        &runtime_sender,
                        &runtime_state,
                        connection_id,
                        event,
                    )
                    .await;
                });
            }
            RuntimeMessage::DeviceEvent {
                connection_id: event_connection_id,
                device_identifier,
            } => {
                if event_connection_id != connection_id {
                    continue;
                }
                let inner = Arc::clone(inner);
                let adapter = Arc::clone(&runtime.adapter);
                tokio::spawn(async move {
                    refresh_device_from_identifier(
                        &inner,
                        adapter.as_ref(),
                        connection_id,
                        device_identifier,
                    )
                    .await;
                });
            }
            RuntimeMessage::DeviceEventsEnded {
                connection_id: event_connection_id,
                device_identifier,
            } => {
                if event_connection_id != connection_id {
                    continue;
                }
                runtime
                    .runtime_state
                    .device_watchers
                    .lock()
                    .await
                    .remove(&device_identifier);
            }
            RuntimeMessage::AdapterEventsEnded {
                connection_id: ended_connection_id,
            } => {
                if ended_connection_id != connection_id {
                    continue;
                }
                warn!("bluetooth adapter event stream ended; reconnecting");
                stop_connected_runtime_tasks(&runtime).await;
                inner.deactivate_connection(connection_id);
                inner.publish(FeatureState::Loading);
                return ConnectedRuntimeExit::Reconnect;
            }
            RuntimeMessage::DiscoveryEnded {
                connection_id: ended_connection_id,
            } => {
                if ended_connection_id != connection_id {
                    continue;
                }
                let mut discovery_task = runtime.runtime_state.discovery_task.lock().await;
                *discovery_task = None;
                debug!("bluetooth discovery task ended");
            }
            RuntimeMessage::Shutdown => {
                stop_connected_runtime_tasks(&runtime).await;
                inner.deactivate_connection(connection_id);
                return ConnectedRuntimeExit::Shutdown;
            }
        }
    }

    stop_connected_runtime_tasks(&runtime).await;
    inner.deactivate_connection(connection_id);
    ConnectedRuntimeExit::Shutdown
}

fn publish_bluetooth_connection_error(inner: &Arc<BackendState>, error: BluetoothConnectionError) {
    match error {
        BluetoothConnectionError::NoAdapter => {
            inner.publish(FeatureState::Unsupported(
                BluetoothUnsupportedReason::NoBluetoothAdapterPresent,
            ));
        }
        BluetoothConnectionError::BackendUnavailable { message } => {
            inner.publish(FeatureState::Unavailable(
                BluetoothUnavailableReason::BackendUnavailable { message },
            ));
        }
    }
}

async fn wait_for_bluetooth_reconnect_delay(
    command_receiver: &mut UnboundedReceiver<RuntimeMessage>,
) -> bool {
    let reconnect_delay = tokio::time::sleep(BLUETOOTH_RECONNECT_DELAY);
    tokio::pin!(reconnect_delay);

    loop {
        tokio::select! {
            () = &mut reconnect_delay => return true,
            message = command_receiver.recv() => match message {
                Some(RuntimeMessage::Shutdown) | None => return false,
                Some(_) => {}
            },
        }
    }
}

async fn stop_connected_runtime_tasks(runtime: &ConnectedBluetoothRuntime) {
    if let Some(task) = runtime.runtime_state.discovery_task.lock().await.take() {
        task.abort();
    }
    let device_watchers = std::mem::take(&mut *runtime.runtime_state.device_watchers.lock().await);
    for watcher in device_watchers.into_values() {
        watcher.abort();
    }
}
