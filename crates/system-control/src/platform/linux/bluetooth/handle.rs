use super::{
    command::{BluetoothCommand, RuntimeMessage},
    runtime::run_linux_bluetooth_runtime,
    store::BackendState,
};
use crate::{
    SystemControlError,
    bluetooth::{
        BluetoothDeviceId, BluetoothFeatureState, BluetoothOperationId, BluetoothOperationReceipt,
        BluetoothRequestError,
    },
    feature_state::FeatureState,
};
use std::{
    sync::{
        Arc, Mutex, Weak,
        atomic::{AtomicU64, Ordering},
    },
    thread::JoinHandle,
};
use tokio::{
    runtime::Builder,
    sync::mpsc::{UnboundedSender, unbounded_channel},
};

#[derive(Clone)]
pub(crate) struct PlatformBluetoothHandle {
    backend: Arc<LinuxBluetoothBackend>,
}

pub(crate) struct PlatformBluetoothObservation {
    observer_id: u64,
    inner: Weak<BackendState>,
}

struct LinuxBluetoothBackend {
    inner: Arc<BackendState>,
    command_sender: UnboundedSender<RuntimeMessage>,
    next_operation_id: AtomicU64,
    runtime_thread: Mutex<Option<JoinHandle<()>>>,
}

impl PlatformBluetoothHandle {
    pub(crate) fn new() -> Result<Self, SystemControlError> {
        let inner = Arc::new(BackendState::new());
        let (command_sender, command_receiver) = unbounded_channel();
        let runtime_inner = Arc::clone(&inner);
        let runtime_sender = command_sender.clone();
        let runtime_thread = std::thread::Builder::new()
            .name("system-control-linux-bluetooth".to_string())
            .spawn(move || {
                let runtime = Builder::new_multi_thread()
                    .worker_threads(2)
                    .enable_all()
                    .build()
                    .expect("failed to build tokio runtime");
                runtime.block_on(async move {
                    run_linux_bluetooth_runtime(runtime_inner, runtime_sender, command_receiver)
                        .await;
                });
            })
            .map_err(|error| SystemControlError::RuntimeThreadSpawnFailed {
                message: error.to_string(),
            })?;

        Ok(Self {
            backend: Arc::new(LinuxBluetoothBackend {
                inner,
                command_sender,
                next_operation_id: AtomicU64::new(1),
                runtime_thread: Mutex::new(Some(runtime_thread)),
            }),
        })
    }

    pub(crate) fn current_state(&self) -> BluetoothFeatureState {
        self.backend.inner.current_state()
    }

    pub(crate) fn observe<F>(&self, observer: F) -> PlatformBluetoothObservation
    where
        F: Fn(BluetoothFeatureState) + Send + 'static,
    {
        let observer_id = self.backend.inner.next_observer_id();
        self.backend
            .inner
            .add_observer(observer_id, Box::new(observer));
        PlatformBluetoothObservation {
            observer_id,
            inner: Arc::downgrade(&self.backend.inner),
        }
    }

    pub(crate) fn set_power_enabled(
        &self,
        is_enabled: bool,
    ) -> Result<BluetoothOperationReceipt, BluetoothRequestError> {
        self.send_command(BluetoothCommand::SetPowerEnabled {
            operation_id: self.next_operation_id(),
            is_enabled,
        })
    }

    pub(crate) fn start_discovery(
        &self,
    ) -> Result<BluetoothOperationReceipt, BluetoothRequestError> {
        self.send_command(BluetoothCommand::StartDiscovery {
            operation_id: self.next_operation_id(),
        })
    }

    pub(crate) fn stop_discovery(
        &self,
    ) -> Result<BluetoothOperationReceipt, BluetoothRequestError> {
        self.send_command(BluetoothCommand::StopDiscovery {
            operation_id: self.next_operation_id(),
        })
    }

    pub(crate) fn connect_device(
        &self,
        device_identifier: BluetoothDeviceId,
    ) -> Result<BluetoothOperationReceipt, BluetoothRequestError> {
        let operation_id = self.next_operation_id();
        self.ensure_ready_device_exists(&device_identifier)?;
        self.send_command(BluetoothCommand::ConnectDevice {
            operation_id,
            device_identifier,
        })
    }

    pub(crate) fn disconnect_device(
        &self,
        device_identifier: BluetoothDeviceId,
    ) -> Result<BluetoothOperationReceipt, BluetoothRequestError> {
        let operation_id = self.next_operation_id();
        self.ensure_ready_device_exists(&device_identifier)?;
        self.send_command(BluetoothCommand::DisconnectDevice {
            operation_id,
            device_identifier,
        })
    }

    fn ensure_ready_device_exists(
        &self,
        device_identifier: &BluetoothDeviceId,
    ) -> Result<(), BluetoothRequestError> {
        match self.current_state() {
            FeatureState::Ready(state) => {
                if state
                    .devices
                    .iter()
                    .any(|device| &device.device_identifier == device_identifier)
                {
                    Ok(())
                } else {
                    Err(BluetoothRequestError::DeviceNotFound)
                }
            }
            _ => Err(BluetoothRequestError::FeatureNotReady),
        }
    }

    fn next_operation_id(&self) -> BluetoothOperationId {
        BluetoothOperationId(
            self.backend
                .next_operation_id
                .fetch_add(1, Ordering::Relaxed),
        )
    }

    fn send_command(
        &self,
        command: BluetoothCommand,
    ) -> Result<BluetoothOperationReceipt, BluetoothRequestError> {
        if !matches!(self.current_state(), FeatureState::Ready(_)) {
            return Err(BluetoothRequestError::FeatureNotReady);
        }

        let operation_id = command.operation_id();
        self.backend
            .command_sender
            .send(RuntimeMessage::Command(command))
            .map_err(|_| BluetoothRequestError::RuntimeUnavailable)?;

        Ok(BluetoothOperationReceipt { operation_id })
    }
}

impl Drop for PlatformBluetoothObservation {
    fn drop(&mut self) {
        if let Some(inner) = self.inner.upgrade() {
            inner.remove_observer(self.observer_id);
        }
    }
}

impl Drop for LinuxBluetoothBackend {
    fn drop(&mut self) {
        let _ = self.command_sender.send(RuntimeMessage::Shutdown);
        if let Some(runtime_thread) = self.runtime_thread.lock().expect("poisoned mutex").take() {
            let _ = runtime_thread.join();
        }
    }
}
