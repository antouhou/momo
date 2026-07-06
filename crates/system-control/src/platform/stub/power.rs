use crate::SystemControlError;
use crate::power::{PowerAction, PowerRequestError};

#[derive(Clone)]
pub(crate) struct PlatformPowerHandle;

impl PlatformPowerHandle {
    pub(crate) fn new() -> Result<Self, SystemControlError> {
        Ok(Self)
    }

    pub(crate) fn request(&self, _action: PowerAction) -> Result<(), PowerRequestError> {
        Ok(())
    }
}
