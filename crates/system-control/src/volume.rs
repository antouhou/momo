use thiserror::Error;
use crate::{SystemControlError, feature_state::FeatureState, platform};

pub type VolumeFeatureState =
    FeatureState<VolumeState, VolumeUnsupportedReason, VolumeUnavailableReason>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VolumeState {
    pub output_percentage: u8,
    pub is_muted: bool,
}

impl VolumeState {
    pub fn new(output_percentage: u8, is_muted: bool) -> Self {
        Self {
            output_percentage: output_percentage.min(100),
            is_muted,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VolumeUnsupportedReason {
    PlatformNotSupported,
}

#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum VolumeUnavailableReason {
    #[error("Volume backend is unavailable: {message}")]
    BackendUnavailable { message: String },
}

#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum VolumeRequestError {
    #[error("Volume feature is not ready")]
    FeatureNotReady,
    #[error("Volume runtime is unavailable")]
    RuntimeUnavailable,
}

#[derive(Clone)]
pub struct VolumeHandle {
    platform_handle: platform::PlatformVolumeHandle,
}

impl VolumeHandle {
    pub(crate) fn new() -> Result<Self, SystemControlError> {
        Ok(Self {
            platform_handle: platform::PlatformVolumeHandle::new()?,
        })
    }

    pub fn current_state(&self) -> VolumeFeatureState {
        self.platform_handle.current_state()
    }

    pub fn observe<F>(&self, observer: F) -> VolumeObservation
    where
        F: Fn(VolumeFeatureState) + Send + 'static,
    {
        VolumeObservation {
            platform_observation: self.platform_handle.observe(observer),
        }
    }

    pub fn set_output_volume_percentage(
        &self,
        output_percentage: u8,
    ) -> Result<(), VolumeRequestError> {
        self.platform_handle
            .set_output_volume_percentage(output_percentage)
    }
}

#[must_use]
pub struct VolumeObservation {
    platform_observation: platform::PlatformVolumeObservation,
}

impl Drop for VolumeObservation {
    fn drop(&mut self) {
        let _ = &self.platform_observation;
    }
}
