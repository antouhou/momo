use crate::SystemControlError;
use crate::feature_state::FeatureState;
use crate::users::{UserListFeatureState, UserUnsupportedReason};

#[derive(Clone)]
pub(crate) struct PlatformUserHandle;

impl PlatformUserHandle {
    pub(crate) fn new() -> Result<Self, SystemControlError> {
        Ok(Self)
    }

    pub(crate) fn list_users(&self) -> UserListFeatureState {
        FeatureState::Unsupported(UserUnsupportedReason::PlatformNotSupported)
    }
}
