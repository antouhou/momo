mod device;

use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, Weak};
use std::thread::JoinHandle;
use std::time::Duration;

use bluer::{Adapter, AdapterEvent, DeviceEvent, Session};
use futures_util::StreamExt;
use tokio::runtime::Builder;
use tokio::sync::Mutex as AsyncMutex;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel};
use tracing::{debug, warn};

use self::device::{device_from_identifier, load_device, sort_devices};
use crate::SystemControlError;
use crate::bluetooth::{
    BluetoothAdapterState, BluetoothCapabilities, BluetoothConnectionState, BluetoothDevice,
    BluetoothDeviceId, BluetoothDiscoveryState, BluetoothFeatureState, BluetoothOperationId,
    BluetoothOperationKind, BluetoothOperationReceipt, BluetoothPendingOperation,
    BluetoothPowerState, BluetoothRequestError, BluetoothState, BluetoothUnavailableReason,
    BluetoothUnsupportedReason, BluetoothUserVisibleError, FeatureState,
};

type ObserverCallback = Box<dyn Fn(BluetoothFeatureState) + Send + 'static>;
const BLUETOOTH_RECONNECT_DELAY: Duration = Duration::from_secs(2);

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

struct BackendState {
    current_state: Mutex<BluetoothFeatureState>,
    observers: Mutex<BTreeMap<u64, Arc<Mutex<ObserverCallback>>>>,
    next_observer_id: AtomicU64,
    active_connection_id: AtomicU64,
}

struct RuntimeTaskState {
    device_watchers: AsyncMutex<BTreeMap<BluetoothDeviceId, tokio::task::JoinHandle<()>>>,
    discovery_task: AsyncMutex<Option<tokio::task::JoinHandle<()>>>,
}

enum RuntimeMessage {
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

enum BluetoothCommand {
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
        let observer_id = self
            .backend
            .inner
            .next_observer_id
            .fetch_add(1, Ordering::Relaxed);
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

impl BackendState {
    fn new() -> Self {
        Self {
            current_state: Mutex::new(FeatureState::Loading),
            observers: Mutex::new(BTreeMap::new()),
            next_observer_id: AtomicU64::new(1),
            active_connection_id: AtomicU64::new(0),
        }
    }

    fn add_observer(&self, observer_id: u64, observer: ObserverCallback) {
        let observer = Arc::new(Mutex::new(observer));
        self.observers
            .lock()
            .expect("poisoned mutex")
            .insert(observer_id, Arc::clone(&observer));

        let current_state = self.current_state();
        observer.lock().expect("poisoned mutex").as_ref()(current_state);
    }

    fn remove_observer(&self, observer_id: u64) {
        self.observers
            .lock()
            .expect("poisoned mutex")
            .remove(&observer_id);
    }

    fn current_state(&self) -> BluetoothFeatureState {
        self.current_state.lock().expect("poisoned mutex").clone()
    }

    fn publish(&self, next_state: BluetoothFeatureState) {
        {
            let mut state = self.current_state.lock().expect("poisoned mutex");
            *state = next_state.clone();
        }

        let observers = self
            .observers
            .lock()
            .expect("poisoned mutex")
            .values()
            .cloned()
            .collect::<Vec<_>>();

        for observer in observers {
            observer.lock().expect("poisoned mutex").as_ref()(next_state.clone());
        }
    }

    fn activate_connection(&self, connection_id: u64) {
        self.active_connection_id
            .store(connection_id, Ordering::Relaxed);
    }

    fn deactivate_connection(&self, connection_id: u64) {
        let _ = self.active_connection_id.compare_exchange(
            connection_id,
            0,
            Ordering::Relaxed,
            Ordering::Relaxed,
        );
    }

    fn clear_active_connection(&self) {
        self.active_connection_id.store(0, Ordering::Relaxed);
    }

    fn is_active_connection(&self, connection_id: u64) -> bool {
        self.active_connection_id.load(Ordering::Relaxed) == connection_id
    }
}

impl BluetoothCommand {
    fn operation_id(&self) -> BluetoothOperationId {
        match self {
            Self::SetPowerEnabled { operation_id, .. }
            | Self::StartDiscovery { operation_id }
            | Self::StopDiscovery { operation_id }
            | Self::ConnectDevice { operation_id, .. }
            | Self::DisconnectDevice { operation_id, .. } => *operation_id,
        }
    }
}

struct ConnectedBluetoothRuntime {
    _session: Session,
    adapter: Arc<Adapter>,
    runtime_state: Arc<RuntimeTaskState>,
}

enum BluetoothConnectionError {
    NoAdapter,
    BackendUnavailable { message: String },
}

enum ConnectedRuntimeExit {
    Reconnect,
    Shutdown,
}

async fn run_linux_bluetooth_runtime(
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

async fn handle_command(
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

async fn handle_adapter_event(
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

async fn spawn_adapter_watcher(
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

fn spawn_device_watcher(
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

async fn load_current_state(adapter: &Adapter) -> bluer::Result<BluetoothState> {
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

async fn refresh_adapter_state(
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

async fn refresh_device(
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

async fn refresh_device_from_identifier(
    inner: &Arc<BackendState>,
    adapter: &Adapter,
    connection_id: u64,
    device_identifier: BluetoothDeviceId,
) {
    refresh_device(adapter, inner, connection_id, device_identifier, None, None).await;
}

fn apply_pending_operation(
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

fn finish_operation_with_error(
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
