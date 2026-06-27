mod battery;
mod bluetooth;
mod volume;

pub(crate) use battery::{PlatformBatteryHandle, PlatformBatteryObservation};
pub(crate) use bluetooth::{PlatformBluetoothHandle, PlatformBluetoothObservation};
pub(crate) use volume::{PlatformVolumeHandle, PlatformVolumeObservation};
