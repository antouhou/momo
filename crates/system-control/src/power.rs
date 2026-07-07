use crate::SystemControlError;
use crate::platform;
use thiserror::Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PowerAction {
    Shutdown,
    Suspend,
    Reboot,
}

#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum PowerRequestError {
    #[error("Power runtime is unavailable")]
    RuntimeUnavailable,
}

#[derive(Clone)]
pub struct PowerHandle {
    platform_handle: platform::PlatformPowerHandle,
}

impl PowerHandle {
    pub(crate) fn new() -> Result<Self, SystemControlError> {
        Ok(Self {
            platform_handle: platform::PlatformPowerHandle::new()?,
        })
    }

    pub fn request(&self, action: PowerAction) -> Result<(), PowerRequestError> {
        self.platform_handle.request(action)
    }

    pub fn shutdown(&self) -> Result<(), PowerRequestError> {
        self.request(PowerAction::Shutdown)
    }

    pub fn suspend(&self) -> Result<(), PowerRequestError> {
        self.request(PowerAction::Suspend)
    }

    pub fn reboot(&self) -> Result<(), PowerRequestError> {
        self.request(PowerAction::Reboot)
    }
}
