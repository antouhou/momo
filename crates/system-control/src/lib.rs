mod battery;
mod bluetooth;
mod feature_state;
mod platform;
mod power;
mod session;
mod users;
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
    BluetoothUserVisibleError,
};
pub use feature_state::FeatureState;
pub use power::{PowerAction, PowerHandle, PowerRequestError};
pub use session::{SessionAction, SessionHandle, SessionRequestError};
pub use users::{
    SystemUser, UserHandle, UserListFeatureState, UserUnavailableReason, UserUnsupportedReason,
};
pub use volume::{
    VolumeFeatureState, VolumeHandle, VolumeObservation, VolumeRequestError, VolumeState,
    VolumeUnavailableReason, VolumeUnsupportedReason,
};

#[derive(Clone)]
pub struct SystemControl {
    battery: BatteryHandle,
    bluetooth: BluetoothHandle,
    power: PowerHandle,
    session: SessionHandle,
    users: UserHandle,
    volume: VolumeHandle,
}

impl SystemControl {
    pub fn new() -> Result<Self, SystemControlError> {
        Ok(Self {
            battery: BatteryHandle::new()?,
            bluetooth: BluetoothHandle::new()?,
            power: PowerHandle::new()?,
            session: SessionHandle::new()?,
            users: UserHandle::new()?,
            volume: VolumeHandle::new()?,
        })
    }

    pub fn battery(&self) -> BatteryHandle {
        self.battery.clone()
    }

    pub fn bluetooth(&self) -> BluetoothHandle {
        self.bluetooth.clone()
    }

    pub fn power(&self) -> PowerHandle {
        self.power.clone()
    }

    pub fn session(&self) -> SessionHandle {
        self.session.clone()
    }

    pub fn users(&self) -> UserHandle {
        self.users.clone()
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
