mod bluetooth;
mod platform;

use thiserror::Error;

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

#[derive(Debug, Error)]
pub enum SystemControlError {
    #[error("failed to spawn system control runtime thread: {message}")]
    RuntimeThreadSpawnFailed { message: String },
}
