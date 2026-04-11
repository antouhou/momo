mod backend;
mod model;

pub use backend::{BackendMetadata, CompositorBackend, CompositorError, ConnectionConfiguration};
pub use model::{
    CapabilitySet, CompositorCommand, CompositorEvent, CompositorSnapshot, Output, ViewSummary,
    Workspace,
};
