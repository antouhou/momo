use thiserror::Error;
use crate::{SystemControlError, feature_state::FeatureState, platform};

pub type UserListFeatureState =
    FeatureState<Vec<SystemUser>, UserUnsupportedReason, UserUnavailableReason>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SystemUser {
    pub identifier: String,
    pub uid: u32,
    pub username: String,
    pub display_name: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UserUnsupportedReason {
    PlatformNotSupported,
}

#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum UserUnavailableReason {
    #[error("User backend is unavailable: {message}")]
    BackendUnavailable { message: String },
}

#[derive(Clone)]
pub struct UserHandle {
    platform_handle: platform::PlatformUserHandle,
}

impl UserHandle {
    pub(crate) fn new() -> Result<Self, SystemControlError> {
        Ok(Self {
            platform_handle: platform::PlatformUserHandle::new()?,
        })
    }

    pub fn list_users(&self) -> UserListFeatureState {
        self.platform_handle.list_users()
    }
}
