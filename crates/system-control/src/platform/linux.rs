use crate::SystemControlError;
use crate::bluetooth::{
    BluetoothAdapterState, BluetoothCapabilities, BluetoothConnectionState, BluetoothDevice,
    BluetoothDeviceCategory, BluetoothDeviceId, BluetoothDiscoveryState, BluetoothFeatureState,
    BluetoothOperationId, BluetoothOperationKind, BluetoothOperationReceipt,
    BluetoothPendingOperation, BluetoothPowerState, BluetoothRequestError, BluetoothState,
    BluetoothUnavailableReason, BluetoothUnsupportedReason, BluetoothUserVisibleError,
    FeatureState,
};
use bluer::{Adapter, AdapterEvent, Device, DeviceEvent};
use futures_util::StreamExt;
use std::collections::BTreeMap;
use std::str::FromStr;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, Weak};
use std::thread::JoinHandle;
use tokio::runtime::Builder;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel};
use tracing::{debug, warn};

type ObserverCallback = Box<dyn Fn(BluetoothFeatureState) + Send + 'static>;

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
}

enum RuntimeMessage {
    Command(BluetoothCommand),
    AdapterEvent(AdapterEvent),
    AdapterEventsEnded,
    DeviceEvent {
        device_identifier: BluetoothDeviceId,
    },
    DeviceEventsEnded {
        device_identifier: BluetoothDeviceId,
    },
    DiscoveryEvent(AdapterEvent),
    DiscoveryEnded,
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
        let runtime_inner = inner.clone();
        let runtime_sender = command_sender.clone();
        let runtime_thread = std::thread::Builder::new()
            .name("system-control-linux-bluetooth".to_string())
            .spawn(move || {
                let runtime = Builder::new_current_thread()
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
        }
    }

    fn add_observer(&self, observer_id: u64, observer: ObserverCallback) {
        let observer = Arc::new(Mutex::new(observer));
        self.observers
            .lock()
            .expect("poisoned mutex")
            .insert(observer_id, observer.clone());

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

async fn run_linux_bluetooth_runtime(
    inner: Arc<BackendState>,
    runtime_sender: UnboundedSender<RuntimeMessage>,
    mut command_receiver: UnboundedReceiver<RuntimeMessage>,
) {
    let session = match bluer::Session::new().await {
        Ok(session) => session,
        Err(error) => {
            inner.publish(FeatureState::Unavailable(
                BluetoothUnavailableReason::BackendUnavailable {
                    message: error.to_string(),
                },
            ));
            return;
        }
    };

    let adapter = match session.default_adapter().await {
        Ok(adapter) => adapter,
        Err(error) => {
            if error.kind == bluer::ErrorKind::NotFound {
                inner.publish(FeatureState::Unsupported(
                    BluetoothUnsupportedReason::NoBluetoothAdapterPresent,
                ));
            } else {
                inner.publish(FeatureState::Unavailable(
                    BluetoothUnavailableReason::BackendUnavailable {
                        message: error.to_string(),
                    },
                ));
            }
            return;
        }
    };

    let mut device_watchers = BTreeMap::<BluetoothDeviceId, tokio::task::JoinHandle<()>>::new();
    let mut discovery_task: Option<tokio::task::JoinHandle<()>> = None;

    let initial_state = match load_full_state(&adapter, 0, Vec::new(), None).await {
        Ok(state) => state,
        Err(error) => {
            inner.publish(FeatureState::Unavailable(
                BluetoothUnavailableReason::BackendUnavailable {
                    message: error.to_string(),
                },
            ));
            return;
        }
    };
    for device in &initial_state.devices {
        spawn_device_watcher(
            &adapter,
            &runtime_sender,
            &mut device_watchers,
            device.device_identifier.clone(),
        )
        .await;
    }
    inner.publish(FeatureState::Ready(initial_state));

    spawn_adapter_watcher(&adapter, &runtime_sender).await;

    while let Some(message) = command_receiver.recv().await {
        match message {
            RuntimeMessage::Command(command) => {
                handle_command(
                    &inner,
                    &adapter,
                    &runtime_sender,
                    &mut device_watchers,
                    &mut discovery_task,
                    command,
                )
                .await;
            }
            RuntimeMessage::AdapterEvent(event) | RuntimeMessage::DiscoveryEvent(event) => {
                handle_adapter_event(
                    &inner,
                    &adapter,
                    &runtime_sender,
                    &mut device_watchers,
                    event,
                )
                .await;
            }
            RuntimeMessage::DeviceEvent { device_identifier } => {
                refresh_device_from_identifier(&inner, &adapter, device_identifier).await;
            }
            RuntimeMessage::DeviceEventsEnded { device_identifier } => {
                device_watchers.remove(&device_identifier);
            }
            RuntimeMessage::AdapterEventsEnded => {
                warn!("bluetooth adapter event stream ended");
                inner.publish(FeatureState::Unavailable(
                    BluetoothUnavailableReason::BackendUnavailable {
                        message: "Bluetooth adapter event stream ended".to_string(),
                    },
                ));
            }
            RuntimeMessage::DiscoveryEnded => {
                debug!("bluetooth discovery task ended");
            }
            RuntimeMessage::Shutdown => {
                if let Some(task) = discovery_task.take() {
                    task.abort();
                }
                for watcher in device_watchers.into_values() {
                    watcher.abort();
                }
                break;
            }
        }
    }
}

async fn handle_command(
    inner: &Arc<BackendState>,
    adapter: &Adapter,
    runtime_sender: &UnboundedSender<RuntimeMessage>,
    device_watchers: &mut BTreeMap<BluetoothDeviceId, tokio::task::JoinHandle<()>>,
    discovery_task: &mut Option<tokio::task::JoinHandle<()>>,
    command: BluetoothCommand,
) {
    apply_pending_operation(inner, command.operation_id(), command.pending_operation());

    match command {
        BluetoothCommand::SetPowerEnabled {
            operation_id,
            is_enabled,
        } => match adapter.set_powered(is_enabled).await {
            Ok(()) => {
                if !is_enabled {
                    if let Some(task) = discovery_task.take() {
                        task.abort();
                    }
                }
                refresh_adapter_state(inner, adapter, Some(operation_id), None).await;
            }
            Err(error) => {
                finish_operation_with_error(inner, Some(operation_id), error.to_string());
            }
        },
        BluetoothCommand::StartDiscovery { operation_id } => {
            if discovery_task.is_none() {
                match adapter.discover_devices().await {
                    Ok(mut discovery_stream) => {
                        let sender = runtime_sender.clone();
                        *discovery_task = Some(tokio::spawn(async move {
                            while let Some(event) = discovery_stream.next().await {
                                if sender.send(RuntimeMessage::DiscoveryEvent(event)).is_err() {
                                    return;
                                }
                            }
                            let _ = sender.send(RuntimeMessage::DiscoveryEnded);
                        }));
                        refresh_adapter_state(inner, adapter, Some(operation_id), None).await;
                    }
                    Err(error) => {
                        finish_operation_with_error(inner, Some(operation_id), error.to_string());
                    }
                }
            } else {
                refresh_adapter_state(inner, adapter, Some(operation_id), None).await;
            }
        }
        BluetoothCommand::StopDiscovery { operation_id } => {
            if let Some(task) = discovery_task.take() {
                task.abort();
            }
            refresh_adapter_state(inner, adapter, Some(operation_id), None).await;
        }
        BluetoothCommand::ConnectDevice {
            operation_id,
            device_identifier,
        } => match device_from_identifier(adapter, &device_identifier) {
            Ok(device) => match device.connect().await {
                Ok(()) => {
                    spawn_device_watcher(
                        adapter,
                        runtime_sender,
                        device_watchers,
                        device_identifier.clone(),
                    )
                    .await;
                    refresh_device(adapter, inner, device_identifier, Some(operation_id), None)
                        .await;
                }
                Err(error) => {
                    finish_operation_with_error(inner, Some(operation_id), error.to_string());
                }
            },
            Err(error) => {
                finish_operation_with_error(inner, Some(operation_id), error);
            }
        },
        BluetoothCommand::DisconnectDevice {
            operation_id,
            device_identifier,
        } => match device_from_identifier(adapter, &device_identifier) {
            Ok(device) => match device.disconnect().await {
                Ok(()) => {
                    refresh_device(adapter, inner, device_identifier, Some(operation_id), None)
                        .await;
                }
                Err(error) => {
                    finish_operation_with_error(inner, Some(operation_id), error.to_string());
                }
            },
            Err(error) => {
                finish_operation_with_error(inner, Some(operation_id), error);
            }
        },
    }
}

async fn handle_adapter_event(
    inner: &Arc<BackendState>,
    adapter: &Adapter,
    runtime_sender: &UnboundedSender<RuntimeMessage>,
    device_watchers: &mut BTreeMap<BluetoothDeviceId, tokio::task::JoinHandle<()>>,
    event: AdapterEvent,
) {
    match event {
        AdapterEvent::PropertyChanged(_) => {
            refresh_adapter_state(inner, adapter, None, None).await;
        }
        AdapterEvent::DeviceAdded(address) => {
            let device_identifier = BluetoothDeviceId(address.to_string());
            spawn_device_watcher(
                adapter,
                runtime_sender,
                device_watchers,
                device_identifier.clone(),
            )
            .await;
            refresh_device(adapter, inner, device_identifier, None, None).await;
        }
        AdapterEvent::DeviceRemoved(address) => {
            let device_identifier = BluetoothDeviceId(address.to_string());
            if let Some(task) = device_watchers.remove(&device_identifier) {
                task.abort();
            }
            remove_device(inner, device_identifier);
        }
    }
}

async fn spawn_adapter_watcher(
    adapter: &Adapter,
    runtime_sender: &UnboundedSender<RuntimeMessage>,
) {
    match adapter.events().await {
        Ok(mut event_stream) => {
            let sender = runtime_sender.clone();
            tokio::spawn(async move {
                while let Some(event) = event_stream.next().await {
                    if sender.send(RuntimeMessage::AdapterEvent(event)).is_err() {
                        return;
                    }
                }
                let _ = sender.send(RuntimeMessage::AdapterEventsEnded);
            });
        }
        Err(error) => {
            warn!("failed to subscribe to bluetooth adapter events: {error}");
        }
    }
}

async fn spawn_device_watcher(
    adapter: &Adapter,
    runtime_sender: &UnboundedSender<RuntimeMessage>,
    device_watchers: &mut BTreeMap<BluetoothDeviceId, tokio::task::JoinHandle<()>>,
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
            device_identifier: watcher_identifier,
        });
    });
    device_watchers.insert(device_identifier, watcher_task);
}

async fn load_full_state(
    adapter: &Adapter,
    previous_revision: u64,
    pending_operations: Vec<BluetoothPendingOperation>,
    last_error: Option<BluetoothUserVisibleError>,
) -> bluer::Result<BluetoothState> {
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

    let mut state = BluetoothState {
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
        pending_operations,
        last_error,
        revision: previous_revision.saturating_add(1),
    };
    apply_pending_states(&mut state);

    Ok(state)
}

async fn refresh_adapter_state(
    inner: &Arc<BackendState>,
    adapter: &Adapter,
    completed_operation_id: Option<BluetoothOperationId>,
    last_error: Option<BluetoothUserVisibleError>,
) {
    let current_state = inner.current_state();
    let FeatureState::Ready(current_ready_state) = current_state else {
        return;
    };

    let pending_operations = current_ready_state
        .pending_operations
        .into_iter()
        .filter(|operation| Some(operation.operation_id) != completed_operation_id)
        .collect::<Vec<_>>();

    match load_full_state(
        adapter,
        current_ready_state.revision,
        pending_operations,
        last_error.or(current_ready_state.last_error),
    )
    .await
    {
        Ok(next_state) => inner.publish(FeatureState::Ready(next_state)),
        Err(error) => {
            finish_operation_with_error(inner, completed_operation_id, error.to_string());
        }
    }
}

async fn refresh_device(
    adapter: &Adapter,
    inner: &Arc<BackendState>,
    device_identifier: BluetoothDeviceId,
    completed_operation_id: Option<BluetoothOperationId>,
    next_error: Option<BluetoothUserVisibleError>,
) {
    let current_state = inner.current_state();
    let FeatureState::Ready(mut ready_state) = current_state else {
        return;
    };

    if let Some(updated_device) = load_device(adapter, device_identifier.0.clone()).await {
        upsert_device(&mut ready_state.devices, updated_device);
    }

    ready_state
        .pending_operations
        .retain(|operation| Some(operation.operation_id) != completed_operation_id);
    ready_state.last_error = next_error.or(ready_state.last_error);
    ready_state.revision = ready_state.revision.saturating_add(1);
    apply_pending_states(&mut ready_state);
    inner.publish(FeatureState::Ready(ready_state));
}

async fn refresh_device_from_identifier(
    inner: &Arc<BackendState>,
    adapter: &Adapter,
    device_identifier: BluetoothDeviceId,
) {
    refresh_device(adapter, inner, device_identifier, None, None).await;
}

async fn load_device(adapter: &Adapter, device_identifier: String) -> Option<BluetoothDevice> {
    let device_identifier = BluetoothDeviceId(device_identifier);
    let device = device_from_identifier(adapter, &device_identifier).ok()?;
    let display_name = device
        .alias()
        .await
        .ok()
        .flatten()
        .unwrap_or_else(|| device_identifier.0.clone());
    let is_paired = device.is_paired().await.ok()?;
    let is_trusted = device.is_trusted().await.ok()?;
    let is_connected = device.is_connected().await.ok()?;
    let signal_strength_dbm = device.rssi().await.ok().flatten();

    Some(BluetoothDevice {
        device_identifier,
        display_name,
        category: classify_device(&device).await,
        is_paired,
        is_trusted,
        connection_state: if is_connected {
            BluetoothConnectionState::Connected
        } else {
            BluetoothConnectionState::Disconnected
        },
        signal_strength_dbm,
        battery_percentage: device.battery_percentage().await.ok().flatten(),
    })
}

async fn classify_device(device: &Device) -> BluetoothDeviceCategory {
    match device.icon().await.ok().flatten().as_deref() {
        Some("audio-card") | Some("audio-headphones") | Some("audio-headset") => {
            BluetoothDeviceCategory::Audio
        }
        Some("computer") | Some("input-gaming") => BluetoothDeviceCategory::Computer,
        Some("input-keyboard") | Some("input-mouse") | Some("input-tablet") => {
            BluetoothDeviceCategory::Input
        }
        Some("phone") | Some("smartphone") => BluetoothDeviceCategory::Phone,
        Some("printer") | Some("camera-photo") => BluetoothDeviceCategory::Peripheral,
        _ => BluetoothDeviceCategory::Unknown,
    }
}

fn device_from_identifier(
    adapter: &Adapter,
    device_identifier: &BluetoothDeviceId,
) -> Result<Device, String> {
    let address =
        bluer::Address::from_str(&device_identifier.0).map_err(|error| error.to_string())?;
    adapter.device(address).map_err(|error| error.to_string())
}

fn apply_pending_operation(
    inner: &Arc<BackendState>,
    operation_id: BluetoothOperationId,
    operation_kind: BluetoothOperationKind,
) {
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
    operation_id: Option<BluetoothOperationId>,
    message: String,
) {
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

fn remove_device(inner: &Arc<BackendState>, device_identifier: BluetoothDeviceId) {
    let current_state = inner.current_state();
    let FeatureState::Ready(mut ready_state) = current_state else {
        return;
    };

    ready_state
        .devices
        .retain(|device| device.device_identifier != device_identifier);
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

fn sort_devices(devices: &mut [BluetoothDevice]) {
    devices.sort_by(|left, right| {
        left.display_name
            .cmp(&right.display_name)
            .then(left.device_identifier.cmp(&right.device_identifier))
    });
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
