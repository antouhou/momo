use crate::{ShellConfiguration, ShellMode, ShellViewModel};
use momo_compositor::{
    BackendMetadata, CompositorBackend, CompositorError, CompositorSession, CompositorSnapshot,
    CompositorStartupConfiguration, ShortcutId, ShortcutRegistration, ShortcutTrigger,
};

#[cfg(test)]
mod tests;

pub struct StartedShellApp {
    pub view_model: ShellViewModel,
    pub compositor_session: Option<CompositorSession>,
}

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

    pub fn start(self) -> Result<StartedShellApp, CompositorError> {
        if self.configuration.mode == ShellMode::Standalone {
            return Ok(StartedShellApp {
                view_model: self.initial_view_model(),
                compositor_session: None,
            });
        }

        let mode = self.configuration.mode;
        let compositor_session = self.backend.start(CompositorStartupConfiguration {
            shortcuts: vec![ShortcutRegistration {
                id: ShortcutId::new(0),
                trigger: ShortcutTrigger::super_key(),
            }],
        })?;
        let view_model = shell_view_model(
            mode,
            compositor_session.metadata(),
            compositor_session.snapshot(),
        );
        Ok(StartedShellApp {
            view_model,
            compositor_session: Some(compositor_session),
        })
    }

    pub fn initial_view_model(&self) -> ShellViewModel {
        shell_view_model(
            self.configuration.mode,
            self.backend.metadata(),
            &CompositorSnapshot::default(),
        )
    }
}

fn shell_view_model(
    mode: ShellMode,
    metadata: BackendMetadata,
    snapshot: &CompositorSnapshot,
) -> ShellViewModel {
    ShellViewModel {
        mode,
        output_count: snapshot.outputs.len(),
        view_count: snapshot.views.len(),
        compositor_name: metadata.name,
    }
}
