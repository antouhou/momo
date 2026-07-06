use std::sync::Arc;
use std::sync::mpsc::{Sender, channel};

use super::systemctl::run_systemctl_command;
use crate::SystemControlError;
use crate::power::{PowerAction, PowerRequestError};

#[derive(Clone)]
pub(crate) struct PlatformPowerHandle {
    backend: Arc<PlatformPowerBackend>,
}

struct PlatformPowerBackend {
    command_sender: Sender<PowerAction>,
}

impl PlatformPowerHandle {
    pub(crate) fn new() -> Result<Self, SystemControlError> {
        let (command_sender, command_receiver) = channel();
        std::thread::Builder::new()
            .name("system-control-linux-power".to_string())
            .spawn(move || {
                while let Ok(action) = command_receiver.recv() {
                    run_power_action(action);
                }
            })
            .map_err(|error| SystemControlError::RuntimeThreadSpawnFailed {
                message: error.to_string(),
            })?;

        Ok(Self {
            backend: Arc::new(PlatformPowerBackend { command_sender }),
        })
    }

    pub(crate) fn request(&self, action: PowerAction) -> Result<(), PowerRequestError> {
        self.backend
            .command_sender
            .send(action)
            .map_err(|_| PowerRequestError::RuntimeUnavailable)
    }
}

fn run_power_action(action: PowerAction) {
    let command = command_for_power_action(action);
    if let Err(error) = run_systemctl_command(["--no-block", command]) {
        tracing::warn!("failed to run power action {action:?}: {error}");
    }
}

fn command_for_power_action(action: PowerAction) -> &'static str {
    match action {
        PowerAction::Shutdown => "poweroff",
        PowerAction::Suspend => "suspend",
        PowerAction::Reboot => "reboot",
    }
}
