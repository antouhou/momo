use crate::SystemControlError;
use crate::feature_state::FeatureState;
use crate::platform;
use thiserror::Error;

pub type BluetoothFeatureState =
    FeatureState<BluetoothState, BluetoothUnsupportedReason, BluetoothUnavailableReason>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BluetoothState {
    pub adapter: BluetoothAdapterState,
    pub devices: Vec<BluetoothDevice>,
    pub pending_operations: Vec<BluetoothPendingOperation>,
    pub last_error: Option<BluetoothUserVisibleError>,
    pub revision: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BluetoothAdapterState {
    pub adapter_identifier: String,
    pub adapter_name: Option<String>,
    pub power_state: BluetoothPowerState,
    pub discovery_state: BluetoothDiscoveryState,
    pub capabilities: BluetoothCapabilities,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BluetoothCapabilities {
    pub can_change_power: bool,
    pub can_start_discovery: bool,
    pub can_connect_devices: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BluetoothDevice {
    pub device_identifier: BluetoothDeviceId,
    pub display_name: String,
    pub category: BluetoothDeviceCategory,
    pub is_paired: bool,
    pub is_trusted: bool,
    pub connection_state: BluetoothConnectionState,
    pub signal_strength_dbm: Option<i16>,
    pub battery_percentage: Option<u8>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BluetoothDeviceCategory {
    Audio,
    Camera,
    CarAudio,
    Computer,
    Display,
    GameController,
    Headphones,
    Headset,
    Health,
    Input,
    Keyboard,
    MediaPlayer,
    Microphone,
    Mouse,
    Network,
    Peripheral,
    Phone,
    Printer,
    Scanner,
    Sensor,
    Speaker,
    Tablet,
    Wearable,
    Unknown,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BluetoothDeviceId(pub String);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BluetoothOperationId(pub u64);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BluetoothPendingOperation {
    pub operation_id: BluetoothOperationId,
    pub kind: BluetoothOperationKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BluetoothOperationKind {
    SetPowerEnabled {
        is_enabled: bool,
    },
    StartDiscovery,
    StopDiscovery,
    ConnectDevice {
        device_identifier: BluetoothDeviceId,
    },
    DisconnectDevice {
        device_identifier: BluetoothDeviceId,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BluetoothPowerState {
    Off,
    TurningOn { operation_id: BluetoothOperationId },
    On,
    TurningOff { operation_id: BluetoothOperationId },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BluetoothDiscoveryState {
    Idle,
    Starting { operation_id: BluetoothOperationId },
    Scanning,
    Stopping { operation_id: BluetoothOperationId },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BluetoothConnectionState {
    Disconnected,
    Connecting { operation_id: BluetoothOperationId },
    Connected,
    Disconnecting { operation_id: BluetoothOperationId },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BluetoothUnsupportedReason {
    PlatformNotSupported,
    NoBluetoothAdapterPresent,
}

#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum BluetoothUnavailableReason {
    #[error("Bluetooth backend is unavailable: {message}")]
    BackendUnavailable { message: String },
}

#[derive(Clone, Debug, Error, PartialEq, Eq)]
#[error("{message}")]
pub struct BluetoothUserVisibleError {
    pub operation_id: Option<BluetoothOperationId>,
    pub message: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BluetoothOperationReceipt {
    pub operation_id: BluetoothOperationId,
}

#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum BluetoothRequestError {
    #[error("Bluetooth feature is not ready")]
    FeatureNotReady,
    #[error("Bluetooth device was not found")]
    DeviceNotFound,
    #[error("Bluetooth runtime is unavailable")]
    RuntimeUnavailable,
}

#[derive(Clone)]
pub struct BluetoothHandle {
    platform_handle: platform::PlatformBluetoothHandle,
}

impl BluetoothHandle {
    pub(crate) fn new() -> Result<Self, SystemControlError> {
        Ok(Self {
            platform_handle: platform::PlatformBluetoothHandle::new()?,
        })
    }

    pub fn current_state(&self) -> BluetoothFeatureState {
        self.platform_handle.current_state()
    }

    pub fn observe<F>(&self, observer: F) -> BluetoothObservation
    where
        F: Fn(BluetoothFeatureState) + Send + 'static,
    {
        BluetoothObservation {
            platform_observation: self.platform_handle.observe(observer),
        }
    }

    pub fn set_power_enabled(
        &self,
        is_enabled: bool,
    ) -> Result<BluetoothOperationReceipt, BluetoothRequestError> {
        self.platform_handle.set_power_enabled(is_enabled)
    }

    pub fn start_discovery(&self) -> Result<BluetoothOperationReceipt, BluetoothRequestError> {
        self.platform_handle.start_discovery()
    }

    pub fn stop_discovery(&self) -> Result<BluetoothOperationReceipt, BluetoothRequestError> {
        self.platform_handle.stop_discovery()
    }

    pub fn connect_device(
        &self,
        device_identifier: BluetoothDeviceId,
    ) -> Result<BluetoothOperationReceipt, BluetoothRequestError> {
        self.platform_handle.connect_device(device_identifier)
    }

    pub fn disconnect_device(
        &self,
        device_identifier: BluetoothDeviceId,
    ) -> Result<BluetoothOperationReceipt, BluetoothRequestError> {
        self.platform_handle.disconnect_device(device_identifier)
    }
}

#[must_use]
pub struct BluetoothObservation {
    platform_observation: platform::PlatformBluetoothObservation,
}

impl Drop for BluetoothObservation {
    fn drop(&mut self) {
        let _ = &self.platform_observation;
    }
}
