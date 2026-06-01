use crate::SystemControlError;
use crate::bluetooth::FeatureState;
use crate::platform;
use thiserror::Error;

pub type BatteryFeatureState =
    FeatureState<BatteryState, BatteryUnsupportedReason, BatteryUnavailableReason>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BatteryState {
    pub percentage: u8,
    pub charging_state: BatteryChargingState,
}

impl BatteryState {
    pub fn new(percentage: u8, charging_state: BatteryChargingState) -> Self {
        Self {
            percentage: percentage.min(100),
            charging_state,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BatteryChargingState {
    Charging,
    Discharging,
    Full,
    NotCharging,
    Unknown,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BatteryUnsupportedReason {
    PlatformNotSupported,
    NoBatteryPresent,
}

#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum BatteryUnavailableReason {
    #[error("Battery backend is unavailable: {message}")]
    BackendUnavailable { message: String },
}

#[derive(Clone)]
pub struct BatteryHandle {
    platform_handle: platform::PlatformBatteryHandle,
}

impl BatteryHandle {
    pub(crate) fn new() -> Result<Self, SystemControlError> {
        Ok(Self {
            platform_handle: platform::PlatformBatteryHandle::new()?,
        })
    }

    pub fn current_state(&self) -> BatteryFeatureState {
        self.platform_handle.current_state()
    }

    pub fn observe<F>(&self, observer: F) -> BatteryObservation
    where
        F: Fn(BatteryFeatureState) + Send + 'static,
    {
        BatteryObservation {
            platform_observation: self.platform_handle.observe(observer),
        }
    }
}

#[must_use]
pub struct BatteryObservation {
    platform_observation: platform::PlatformBatteryObservation,
}

impl Drop for BatteryObservation {
    fn drop(&mut self) {
        let _ = &self.platform_observation;
    }
}
