use crate::{
    SystemControlError,
    session::{SessionAction, SessionRequestError},
};

#[derive(Clone)]
pub(crate) struct PlatformSessionHandle;

impl PlatformSessionHandle {
    pub(crate) fn new() -> Result<Self, SystemControlError> {
        Ok(Self)
    }

    pub(crate) fn request(&self, _action: SessionAction) -> Result<(), SessionRequestError> {
        Ok(())
    }
}
