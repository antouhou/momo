mod battery;
mod bluetooth;
mod loginctl;
mod power;
mod session;
mod systemctl;
mod users;
mod volume;

pub(crate) use battery::{PlatformBatteryHandle, PlatformBatteryObservation};
pub(crate) use bluetooth::{PlatformBluetoothHandle, PlatformBluetoothObservation};
pub(crate) use power::PlatformPowerHandle;
pub(crate) use session::PlatformSessionHandle;
pub(crate) use users::PlatformUserHandle;
pub(crate) use volume::{PlatformVolumeHandle, PlatformVolumeObservation};
