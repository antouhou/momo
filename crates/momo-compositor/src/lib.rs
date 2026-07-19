mod backend;
mod model;

pub use backend::{
    BackendMetadata, CompositorBackend, CompositorError, CompositorSession,
    CompositorStartupConfiguration, EventLoopShutdownReceiver,
};
pub use daikore::integration::input::Key;
pub use model::{
    CapabilitySet, CompositorCommand, CompositorEvent, CompositorSnapshot, Output, ShortcutId,
    ShortcutRegistration, ShortcutTrigger, ViewSummary, Workspace,
};
