mod bluetooth;
mod platform;

pub use bluetooth::{
    BluetoothAdapterState, BluetoothCapabilities, BluetoothConnectionState, BluetoothDevice,
    BluetoothDeviceCategory, BluetoothDeviceId, BluetoothDiscoveryState, BluetoothFeatureState,
    BluetoothHandle, BluetoothObservation, BluetoothOperationId, BluetoothOperationKind,
    BluetoothOperationReceipt, BluetoothPendingOperation, BluetoothPowerState,
    BluetoothRequestError, BluetoothState, BluetoothUnavailableReason, BluetoothUnsupportedReason,
    BluetoothUserVisibleError, FeatureState,
};

#[derive(Clone)]
pub struct SystemControl {
    bluetooth: BluetoothHandle,
}

impl SystemControl {
    pub fn new() -> Result<Self, SystemControlError> {
        Ok(Self {
            bluetooth: BluetoothHandle::new()?,
        })
    }

    pub fn bluetooth(&self) -> BluetoothHandle {
        self.bluetooth.clone()
    }
}

#[derive(Debug)]
pub enum SystemControlError {
    RuntimeThreadSpawnFailed { message: String },
}

impl std::fmt::Display for SystemControlError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RuntimeThreadSpawnFailed { message } => {
                write!(
                    formatter,
                    "failed to spawn system control runtime thread: {message}"
                )
            }
        }
    }
}

impl std::error::Error for SystemControlError {}
