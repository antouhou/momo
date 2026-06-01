#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
mod linux_system;
#[cfg(not(target_os = "linux"))]
mod stub;
#[cfg(not(target_os = "linux"))]
mod system_stub;

#[cfg(target_os = "linux")]
pub(crate) use linux::{PlatformBluetoothHandle, PlatformBluetoothObservation};
#[cfg(target_os = "linux")]
pub(crate) use linux_system::{
    PlatformBatteryHandle, PlatformBatteryObservation, PlatformVolumeHandle,
    PlatformVolumeObservation,
};
#[cfg(not(target_os = "linux"))]
pub(crate) use stub::{PlatformBluetoothHandle, PlatformBluetoothObservation};
#[cfg(not(target_os = "linux"))]
pub(crate) use system_stub::{
    PlatformBatteryHandle, PlatformBatteryObservation, PlatformVolumeHandle,
    PlatformVolumeObservation,
};
