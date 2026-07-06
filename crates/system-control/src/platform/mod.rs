#[cfg(target_os = "linux")]
mod linux;
#[cfg(not(target_os = "linux"))]
mod stub;

#[cfg(target_os = "linux")]
pub(crate) use linux::{
    PlatformBatteryHandle, PlatformBatteryObservation, PlatformBluetoothHandle,
    PlatformBluetoothObservation, PlatformPowerHandle, PlatformSessionHandle, PlatformUserHandle,
    PlatformVolumeHandle, PlatformVolumeObservation,
};
#[cfg(not(target_os = "linux"))]
pub(crate) use stub::{
    PlatformBatteryHandle, PlatformBatteryObservation, PlatformBluetoothHandle,
    PlatformBluetoothObservation, PlatformPowerHandle, PlatformSessionHandle, PlatformUserHandle,
    PlatformVolumeHandle, PlatformVolumeObservation,
};
