use std::sync::{
    Arc, Weak,
    atomic::Ordering,
    mpsc::{Sender, channel},
};
use super::{
    runtime::{VolumeRuntimeMessage, run_volume_runtime},
    state::LinuxVolumeState,
};
use crate::{
    SystemControlError,
    feature_state::FeatureState,
    volume::{VolumeFeatureState, VolumeRequestError},
};

#[derive(Clone)]
pub(crate) struct PlatformVolumeHandle {
    backend: Arc<LinuxVolumeBackend>,
}

pub(crate) struct PlatformVolumeObservation {
    observer_id: u64,
    inner: Weak<LinuxVolumeState>,
}

struct LinuxVolumeBackend {
    inner: Arc<LinuxVolumeState>,
    command_sender: Sender<VolumeRuntimeMessage>,
}

impl PlatformVolumeHandle {
    pub(crate) fn new() -> Result<Self, SystemControlError> {
        let inner = Arc::new(LinuxVolumeState::new(FeatureState::Loading));
        let (command_sender, command_receiver) = channel();
        let worker_inner = Arc::clone(&inner);
        let runtime_sender = command_sender.clone();
        std::thread::Builder::new()
            .name("system-control-linux-volume".to_string())
            .spawn(move || run_volume_runtime(worker_inner, command_receiver, runtime_sender))
            .map_err(|error| SystemControlError::RuntimeThreadSpawnFailed {
                message: error.to_string(),
            })?;

        Ok(Self {
            backend: Arc::new(LinuxVolumeBackend {
                inner,
                command_sender,
            }),
        })
    }

    pub(crate) fn current_state(&self) -> VolumeFeatureState {
        self.backend.inner.current_state()
    }

    pub(crate) fn observe<F>(&self, observer: F) -> PlatformVolumeObservation
    where
        F: Fn(VolumeFeatureState) + Send + 'static,
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
            .send(VolumeRuntimeMessage::NotifyObserver { observer_id });

        PlatformVolumeObservation {
            observer_id,
            inner: Arc::downgrade(&self.backend.inner),
        }
    }

    pub(crate) fn set_output_volume_percentage(
        &self,
        output_percentage: u8,
    ) -> Result<(), VolumeRequestError> {
        self.backend
            .command_sender
            .send(VolumeRuntimeMessage::SetOutputPercentage { output_percentage })
            .map_err(|_| VolumeRequestError::RuntimeUnavailable)
    }
}

impl Drop for PlatformVolumeObservation {
    fn drop(&mut self) {
        if let Some(inner) = self.inner.upgrade() {
            inner.remove_observer(self.observer_id);
        }
    }
}
