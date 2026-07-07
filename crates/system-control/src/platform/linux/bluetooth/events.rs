use super::{
    command::RuntimeMessage,
    device::device_from_identifier,
    runtime::{BluetoothConnectionError, RuntimeTaskState},
    state::{refresh_adapter_state, refresh_device},
    store::BackendState,
};
use crate::bluetooth::BluetoothDeviceId;
use bluer::{Adapter, AdapterEvent, DeviceEvent};
use futures_util::StreamExt;
use std::{collections::BTreeMap, sync::Arc};
use tokio::sync::mpsc::UnboundedSender;
use tracing::warn;

pub(super) async fn handle_adapter_event(
    inner: &Arc<BackendState>,
    adapter: &Adapter,
    runtime_sender: &UnboundedSender<RuntimeMessage>,
    runtime_state: &Arc<RuntimeTaskState>,
    connection_id: u64,
    event: AdapterEvent,
) {
    match event {
        AdapterEvent::PropertyChanged(_) => {
            refresh_adapter_state(inner, adapter, connection_id, None, None).await;
        }
        AdapterEvent::DeviceAdded(address) => {
            let device_identifier = BluetoothDeviceId(address.to_string());
            let mut device_watchers = runtime_state.device_watchers.lock().await;
            spawn_device_watcher(
                adapter,
                runtime_sender,
                &mut device_watchers,
                connection_id,
                device_identifier.clone(),
            );
            drop(device_watchers);
            refresh_device(adapter, inner, connection_id, device_identifier, None, None).await;
        }
        AdapterEvent::DeviceRemoved(address) => {
            let device_identifier = BluetoothDeviceId(address.to_string());
            if let Some(task) = runtime_state
                .device_watchers
                .lock()
                .await
                .remove(&device_identifier)
            {
                task.abort();
            }
            let is_device_present = refresh_device(
                adapter,
                inner,
                connection_id,
                device_identifier.clone(),
                None,
                None,
            )
            .await;
            if is_device_present {
                let mut device_watchers = runtime_state.device_watchers.lock().await;
                spawn_device_watcher(
                    adapter,
                    runtime_sender,
                    &mut device_watchers,
                    connection_id,
                    device_identifier,
                );
            }
        }
    }
}

pub(super) async fn spawn_adapter_watcher(
    adapter: &Adapter,
    runtime_sender: &UnboundedSender<RuntimeMessage>,
    connection_id: u64,
) -> Result<(), BluetoothConnectionError> {
    match adapter.events().await {
        Ok(mut event_stream) => {
            let sender = runtime_sender.clone();
            tokio::spawn(async move {
                while let Some(event) = event_stream.next().await {
                    if sender
                        .send(RuntimeMessage::AdapterEvent {
                            connection_id,
                            event,
                        })
                        .is_err()
                    {
                        return;
                    }
                }
                let _ = sender.send(RuntimeMessage::AdapterEventsEnded { connection_id });
            });
            Ok(())
        }
        Err(error) => Err(BluetoothConnectionError::BackendUnavailable {
            message: format!("failed to subscribe to bluetooth adapter events: {error}"),
        }),
    }
}

pub(super) fn spawn_device_watcher(
    adapter: &Adapter,
    runtime_sender: &UnboundedSender<RuntimeMessage>,
    device_watchers: &mut BTreeMap<BluetoothDeviceId, tokio::task::JoinHandle<()>>,
    connection_id: u64,
    device_identifier: BluetoothDeviceId,
) {
    if device_watchers.contains_key(&device_identifier) {
        return;
    }

    let device = match device_from_identifier(adapter, &device_identifier) {
        Ok(device) => device,
        Err(error) => {
            warn!("failed to create Bluetooth device watcher: {error}");
            return;
        }
    };

    let sender = runtime_sender.clone();
    let watcher_identifier = device_identifier.clone();
    let watcher_task = tokio::spawn(async move {
        match device.events().await {
            Ok(mut event_stream) => {
                while let Some(event) = event_stream.next().await {
                    match event {
                        DeviceEvent::PropertyChanged(_) => {
                            if sender
                                .send(RuntimeMessage::DeviceEvent {
                                    connection_id,
                                    device_identifier: watcher_identifier.clone(),
                                })
                                .is_err()
                            {
                                return;
                            }
                        }
                    }
                }
            }
            Err(error) => {
                warn!("failed to subscribe to Bluetooth device events: {error}");
            }
        }

        let _ = sender.send(RuntimeMessage::DeviceEventsEnded {
            connection_id,
            device_identifier: watcher_identifier,
        });
    });
    device_watchers.insert(device_identifier, watcher_task);
}
