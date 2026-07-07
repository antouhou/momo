use crate::{CapabilitySet, CompositorCommand, CompositorEvent, CompositorSnapshot};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BackendMetadata {
    pub name: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ConnectionConfiguration {
    pub socket_path: Option<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("{message}")]
pub struct CompositorError {
    message: String,
}

impl CompositorError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

pub trait CompositorBackend {
    fn metadata(&self) -> BackendMetadata;

    fn capabilities(&self) -> CapabilitySet;

    fn connect(&mut self, configuration: &ConnectionConfiguration) -> Result<(), CompositorError>;

    fn snapshot(&self) -> Result<CompositorSnapshot, CompositorError>;

    fn dispatch(&mut self, command: CompositorCommand) -> Result<(), CompositorError>;

    fn poll_events(&mut self) -> Result<Vec<CompositorEvent>, CompositorError>;
}
