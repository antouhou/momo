use momo_compositor::{
    CompositorBackend, CompositorError, CompositorSnapshot, ConnectionConfiguration,
};

use crate::{ShellConfiguration, ShellViewModel};

pub struct ShellApp<Backend>
where
    Backend: CompositorBackend,
{
    configuration: ShellConfiguration,
    backend: Backend,
}

impl<Backend> ShellApp<Backend>
where
    Backend: CompositorBackend,
{
    pub fn new(configuration: ShellConfiguration, backend: Backend) -> Self {
        Self {
            configuration,
            backend,
        }
    }

    pub fn connect_backend(&mut self) -> Result<(), CompositorError> {
        self.backend.connect(&ConnectionConfiguration::default())
    }

    pub fn initial_view_model(&self) -> ShellViewModel {
        let snapshot = self
            .backend
            .snapshot()
            .unwrap_or_else(|_| CompositorSnapshot {
                outputs: Vec::new(),
                views: Vec::new(),
            });

        ShellViewModel {
            mode: self.configuration.mode,
            output_count: snapshot.outputs.len(),
            view_count: snapshot.views.len(),
            compositor_name: self.backend.metadata().name,
        }
    }
}
