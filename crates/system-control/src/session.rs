use crate::{SystemControlError, platform};
use thiserror::Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SessionAction {
    LogOut,
}

#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum SessionRequestError {
    #[error("Session runtime is unavailable")]
    RuntimeUnavailable,
}

#[derive(Clone)]
pub struct SessionHandle {
    platform_handle: platform::PlatformSessionHandle,
}

impl SessionHandle {
    pub(crate) fn new() -> Result<Self, SystemControlError> {
        Ok(Self {
            platform_handle: platform::PlatformSessionHandle::new()?,
        })
    }

    pub fn request(&self, action: SessionAction) -> Result<(), SessionRequestError> {
        self.platform_handle.request(action)
    }

    pub fn log_out(&self) -> Result<(), SessionRequestError> {
        self.request(SessionAction::LogOut)
    }
}
