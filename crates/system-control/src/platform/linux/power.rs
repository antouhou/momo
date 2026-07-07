use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender, channel};

use super::login1::Login1Client;
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
            .spawn(move || run_power_runtime(command_receiver))
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

fn run_power_runtime(command_receiver: Receiver<PowerAction>) {
    let login1 = match Login1Client::new() {
        Ok(login1) => login1,
        Err(error) => {
            tracing::warn!("failed to initialize login1 power runtime: {error}");
            return;
        }
    };

    while let Ok(action) = command_receiver.recv() {
        run_power_action(&login1, action);
    }
}

fn run_power_action(login1: &Login1Client, action: PowerAction) {
    if let Err(error) = login1.request_power_action(action) {
        tracing::warn!("failed to run power action {action:?}: {error}");
    }
}
