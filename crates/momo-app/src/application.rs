use std::{
    sync::mpsc::{Receiver, Sender},
    thread::JoinHandle,
};

use momo_compositor::{
    CompositorAction, CompositorBackend, CompositorCommand, CompositorError, CompositorEvent,
    CompositorSnapshot, ConnectionConfiguration,
};

use crate::{ShellConfiguration, ShellMode, ShellViewModel};

#[cfg(test)]
mod tests;

pub struct StartedShellApp {
    pub view_model: ShellViewModel,
    pub runtime: Option<CompositorRuntime>,
}

pub struct CompositorRuntime {
    event_receiver: Option<Receiver<CompositorEvent>>,
    shutdown_sender: Sender<()>,
    thread_handle: Option<JoinHandle<()>>,
}

impl CompositorRuntime {
    pub fn take_event_receiver(&mut self) -> Option<Receiver<CompositorEvent>> {
        self.event_receiver.take()
    }

    pub fn stop(&mut self) {
        let _ = self.shutdown_sender.send(());
        if let Some(thread_handle) = self.thread_handle.take()
            && thread_handle.join().is_err()
        {
            tracing::error!("compositor runtime thread panicked during shutdown");
        }
    }
}

impl Drop for CompositorRuntime {
    fn drop(&mut self) {
        self.stop();
    }
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

    pub fn connect_backend(&mut self) -> Result<(), CompositorError> {
        self.backend.connect(&ConnectionConfiguration::default())
    }

    pub fn start(mut self) -> Result<StartedShellApp, CompositorError>
    where
        Backend: Send + 'static,
    {
        if self.configuration.mode == ShellMode::Standalone {
            return Ok(StartedShellApp {
                view_model: self.initial_view_model(),
                runtime: None,
            });
        }

        self.connect_backend()?;
        let view_model = self.initial_view_model();
        self.backend.dispatch(CompositorCommand::RegisterAction(
            CompositorAction::ToggleLauncher,
        ))?;
        let runtime = start_compositor_runtime(self.backend)?;
        Ok(StartedShellApp {
            view_model,
            runtime: Some(runtime),
        })
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

fn start_compositor_runtime(
    mut backend: impl CompositorBackend + Send + 'static,
) -> Result<CompositorRuntime, CompositorError> {
    let (event_sender, event_receiver) = std::sync::mpsc::channel();
    let (shutdown_sender, shutdown_receiver) = std::sync::mpsc::channel();
    let thread_handle = std::thread::Builder::new()
        .name("momo-compositor-runtime".to_string())
        .spawn(move || {
            while shutdown_receiver.try_recv().is_err() {
                match backend.poll_events() {
                    Ok(events) => {
                        for event in events {
                            if event_sender.send(event).is_err() {
                                return;
                            }
                        }
                    }
                    Err(error) => {
                        tracing::error!(?error, "compositor runtime stopped after an error");
                        let _ = event_sender.send(CompositorEvent::Disconnected);
                        return;
                    }
                }
            }
        })
        .map_err(|error| {
            CompositorError::new(format!("failed to spawn compositor runtime: {error}"))
        })?;
    Ok(CompositorRuntime {
        event_receiver: Some(event_receiver),
        shutdown_sender,
        thread_handle: Some(thread_handle),
    })
}
