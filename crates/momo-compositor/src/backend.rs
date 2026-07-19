use crate::{CapabilitySet, CompositorEvent, CompositorSnapshot, ShortcutRegistration};
use std::{
    sync::mpsc::{Receiver, Sender},
    thread::JoinHandle,
};
use thiserror::Error;
use tokio::sync::oneshot;

struct EventLoopShutdownSender {
    sender: Option<oneshot::Sender<()>>,
}

pub struct EventLoopShutdownReceiver {
    receiver: oneshot::Receiver<()>,
}

pub struct CompositorSession {
    metadata: BackendMetadata,
    capabilities: CapabilitySet,
    snapshot: CompositorSnapshot,
    event_receiver: Option<Receiver<CompositorEvent>>,
    shutdown_sender: EventLoopShutdownSender,
    thread_handle: Option<JoinHandle<()>>,
}

fn event_loop_shutdown_channel() -> (EventLoopShutdownSender, EventLoopShutdownReceiver) {
    let (sender, receiver) = oneshot::channel();
    (
        EventLoopShutdownSender {
            sender: Some(sender),
        },
        EventLoopShutdownReceiver { receiver },
    )
}

impl EventLoopShutdownSender {
    fn signal(&mut self) {
        if let Some(sender) = self.sender.take() {
            let _ = sender.send(());
        }
    }
}

impl EventLoopShutdownReceiver {
    pub async fn wait(&mut self) {
        let _ = (&mut self.receiver).await;
    }

    pub fn blocking_wait(self) {
        let _ = self.receiver.blocking_recv();
    }
}

impl CompositorSession {
    pub fn spawn(
        metadata: BackendMetadata,
        capabilities: CapabilitySet,
        snapshot: CompositorSnapshot,
        event_loop: impl FnOnce(
            Sender<CompositorEvent>,
            EventLoopShutdownReceiver,
        ) -> Result<(), CompositorError>
        + Send
        + 'static,
    ) -> Result<Self, CompositorError> {
        let (event_sender, event_receiver) = std::sync::mpsc::channel();
        let (shutdown_sender, shutdown_receiver) = event_loop_shutdown_channel();
        let thread_handle = std::thread::Builder::new()
            .name("momo-compositor-runtime".to_string())
            .spawn(move || {
                if let Err(error) = event_loop(event_sender.clone(), shutdown_receiver) {
                    tracing::error!(?error, "compositor runtime stopped after an error");
                    let _ = event_sender.send(CompositorEvent::Disconnected);
                }
            })
            .map_err(|error| {
                CompositorError::new(format!("failed to spawn compositor runtime: {error}"))
            })?;
        Ok(Self {
            metadata,
            capabilities,
            snapshot,
            event_receiver: Some(event_receiver),
            shutdown_sender,
            thread_handle: Some(thread_handle),
        })
    }

    pub fn metadata(&self) -> BackendMetadata {
        self.metadata
    }

    pub fn capabilities(&self) -> &CapabilitySet {
        &self.capabilities
    }

    pub fn snapshot(&self) -> &CompositorSnapshot {
        &self.snapshot
    }

    pub fn take_event_receiver(&mut self) -> Option<Receiver<CompositorEvent>> {
        self.event_receiver.take()
    }

    pub fn stop(&mut self) {
        self.shutdown_sender.signal();
        if let Some(thread_handle) = self.thread_handle.take()
            && thread_handle.join().is_err()
        {
            tracing::error!("compositor runtime thread panicked during shutdown");
        }
    }
}

impl Drop for CompositorSession {
    fn drop(&mut self) {
        self.stop();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BackendMetadata {
    pub name: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CompositorStartupConfiguration {
    pub shortcuts: Vec<ShortcutRegistration>,
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

    fn start(
        self,
        configuration: CompositorStartupConfiguration,
    ) -> Result<CompositorSession, CompositorError>;
}
