use std::ffi::OsString;
use std::sync::Arc;
use std::sync::mpsc::{Sender, channel};

use super::loginctl::run_loginctl_command;
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
            .spawn(move || {
                while let Ok(action) = command_receiver.recv() {
                    run_session_action(action);
                }
            })
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

fn run_session_action(action: SessionAction) {
    match action {
        SessionAction::LogOut => {
            if let Err(error) = log_out() {
                tracing::warn!("failed to log out: {error}");
            }
        }
    }
}

fn log_out() -> Result<(), String> {
    if let Some(session_id) = non_empty_env("XDG_SESSION_ID") {
        return run_loginctl_command([OsString::from("terminate-session"), session_id]);
    }

    let user = non_empty_env("USER")
        .or_else(|| non_empty_env("LOGNAME"))
        .ok_or_else(|| "missing XDG_SESSION_ID, USER, and LOGNAME".to_string())?;
    run_loginctl_command([OsString::from("terminate-user"), user])
}

fn non_empty_env(key: &str) -> Option<OsString> {
    let value = std::env::var_os(key)?;
    (!value.is_empty()).then_some(value)
}
