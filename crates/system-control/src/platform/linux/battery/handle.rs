use std::sync::atomic::Ordering;
use std::sync::mpsc::{Sender, channel};
use std::sync::{Arc, Weak};

use super::runtime::{BatteryRuntimeMessage, run_battery_runtime};
use super::state::LinuxBatteryState;
use crate::SystemControlError;
use crate::battery::BatteryFeatureState;
use crate::bluetooth::FeatureState;

#[derive(Clone)]
pub(crate) struct PlatformBatteryHandle {
    backend: Arc<LinuxBatteryBackend>,
}

pub(crate) struct PlatformBatteryObservation {
    observer_id: u64,
    inner: Weak<LinuxBatteryState>,
}

struct LinuxBatteryBackend {
    inner: Arc<LinuxBatteryState>,
    command_sender: Sender<BatteryRuntimeMessage>,
}

impl PlatformBatteryHandle {
    pub(crate) fn new() -> Result<Self, SystemControlError> {
        let inner = Arc::new(LinuxBatteryState::new(FeatureState::Loading));
        let (command_sender, command_receiver) = channel();
        let worker_inner = Arc::clone(&inner);
        std::thread::Builder::new()
            .name("system-control-linux-battery".to_string())
            .spawn(move || run_battery_runtime(worker_inner, command_receiver))
            .map_err(|error| SystemControlError::RuntimeThreadSpawnFailed {
                message: error.to_string(),
            })?;

        Ok(Self {
            backend: Arc::new(LinuxBatteryBackend {
                inner,
                command_sender,
            }),
        })
    }

    pub(crate) fn current_state(&self) -> BatteryFeatureState {
        self.backend.inner.current_state()
    }

    pub(crate) fn observe<F>(&self, observer: F) -> PlatformBatteryObservation
    where
        F: Fn(BatteryFeatureState) + Send + 'static,
    {
        let observer_id = self
            .backend
            .inner
            .next_observer_id
            .fetch_add(1, Ordering::Relaxed);
        self.backend
            .inner
            .add_observer(observer_id, Box::new(observer));
        let _ = self
            .backend
            .command_sender
            .send(BatteryRuntimeMessage::NotifyObserver { observer_id });

        PlatformBatteryObservation {
            observer_id,
            inner: Arc::downgrade(&self.backend.inner),
        }
    }
}

impl Drop for PlatformBatteryObservation {
    fn drop(&mut self) {
        if let Some(inner) = self.inner.upgrade() {
            inner.remove_observer(self.observer_id);
        }
    }
}
