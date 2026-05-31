mod battery;
mod bluetooth;
mod platform;
mod volume;

use thiserror::Error;

pub use battery::{
    BatteryChargingState, BatteryFeatureState, BatteryHandle, BatteryObservation, BatteryState,
    BatteryUnavailableReason, BatteryUnsupportedReason,
};
pub use bluetooth::{
    BluetoothAdapterState, BluetoothCapabilities, BluetoothConnectionState, BluetoothDevice,
    BluetoothDeviceCategory, BluetoothDeviceId, BluetoothDiscoveryState, BluetoothFeatureState,
    BluetoothHandle, BluetoothObservation, BluetoothOperationId, BluetoothOperationKind,
    BluetoothOperationReceipt, BluetoothPendingOperation, BluetoothPowerState,
    BluetoothRequestError, BluetoothState, BluetoothUnavailableReason, BluetoothUnsupportedReason,
    BluetoothUserVisibleError, FeatureState,
};
pub use volume::{
    VolumeFeatureState, VolumeHandle, VolumeObservation, VolumeRequestError, VolumeState,
    VolumeUnavailableReason, VolumeUnsupportedReason,
};

#[derive(Clone)]
pub struct SystemControl {
    battery: BatteryHandle,
    bluetooth: BluetoothHandle,
    volume: VolumeHandle,
}

impl SystemControl {
    pub fn new() -> Result<Self, SystemControlError> {
        Ok(Self {
            battery: BatteryHandle::new()?,
            bluetooth: BluetoothHandle::new()?,
            volume: VolumeHandle::new()?,
        })
    }

    pub fn battery(&self) -> BatteryHandle {
        self.battery.clone()
    }

    pub fn bluetooth(&self) -> BluetoothHandle {
        self.bluetooth.clone()
    }

    pub fn volume(&self) -> VolumeHandle {
        self.volume.clone()
    }
}

#[derive(Debug, Error)]
pub enum SystemControlError {
    #[error("failed to spawn system control runtime thread: {message}")]
    RuntimeThreadSpawnFailed { message: String },
}
