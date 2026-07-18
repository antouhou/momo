mod backend;
mod model;

pub use backend::{BackendMetadata, CompositorBackend, CompositorError, ConnectionConfiguration};
pub use model::{
    CapabilitySet, CompositorAction, CompositorCommand, CompositorEvent, CompositorSnapshot,
    Output, ViewSummary, Workspace,
};
