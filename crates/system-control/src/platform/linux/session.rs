use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender, channel};

use super::login1::Login1Client;
use crate::SystemControlError;
use crate::session::{SessionAction, SessionRequestError};

#[derive(Clone)]
pub(crate) struct PlatformSessionHandle {
    backend: Arc<PlatformSessionBackend>,
}

struct PlatformSessionBackend {
    command_sender: Sender<SessionAction>,
}

impl PlatformSessionHandle {
    pub(crate) fn new() -> Result<Self, SystemControlError> {
        let (command_sender, command_receiver) = channel();
        std::thread::Builder::new()
            .name("system-control-linux-session".to_string())
            .spawn(move || run_session_runtime(command_receiver))
            .map_err(|error| SystemControlError::RuntimeThreadSpawnFailed {
                message: error.to_string(),
            })?;

        Ok(Self {
            backend: Arc::new(PlatformSessionBackend { command_sender }),
        })
    }

    pub(crate) fn request(&self, action: SessionAction) -> Result<(), SessionRequestError> {
        self.backend
            .command_sender
            .send(action)
            .map_err(|_| SessionRequestError::RuntimeUnavailable)
    }
}

fn run_session_runtime(command_receiver: Receiver<SessionAction>) {
    let login1 = match Login1Client::new() {
        Ok(login1) => login1,
        Err(error) => {
            tracing::warn!("failed to initialize login1 session runtime: {error}");
            return;
        }
    };

    while let Ok(action) = command_receiver.recv() {
        run_session_action(&login1, action);
    }
}

fn run_session_action(login1: &Login1Client, action: SessionAction) {
    if let Err(error) = login1.request_session_action(action) {
        tracing::warn!("failed to run session action {action:?}: {error}");
    }
}
