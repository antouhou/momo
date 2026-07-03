use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::sync::{Arc, Mutex, Weak};

use crate::SystemControlError;
use crate::bluetooth::FeatureState;
use crate::volume::{VolumeFeatureState, VolumeRequestError, VolumeState};

const STUB_VOLUME_PERCENTAGE: u8 = 40;

type VolumeObserverCallback = Box<dyn Fn(VolumeFeatureState) + Send + 'static>;

#[derive(Clone)]
pub(crate) struct PlatformVolumeHandle {
    backend: Arc<PlatformVolumeBackend>,
}

pub(crate) struct PlatformVolumeObservation {
    observer_id: u64,
    inner: Weak<PlatformVolumeState>,
}

struct PlatformVolumeBackend {
    inner: Arc<PlatformVolumeState>,
    command_sender: Sender<VolumeRuntimeMessage>,
}

struct PlatformVolumeState {
    current_state: Mutex<VolumeFeatureState>,
    observers: Mutex<BTreeMap<u64, Arc<Mutex<VolumeObserverCallback>>>>,
    next_observer_id: AtomicU64,
}

enum VolumeRuntimeMessage {
    NotifyObserver { observer_id: u64 },
    SetOutputPercentage { output_percentage: u8 },
}

impl PlatformVolumeHandle {
    pub(crate) fn new() -> Result<Self, SystemControlError> {
        let inner = Arc::new(PlatformVolumeState {
            current_state: Mutex::new(FeatureState::Ready(VolumeState::new(
                STUB_VOLUME_PERCENTAGE,
                false,
            ))),
            observers: Mutex::new(BTreeMap::new()),
            next_observer_id: AtomicU64::new(1),
        });
        let (command_sender, command_receiver) = channel();
        let worker_inner = Arc::clone(&inner);
        std::thread::Builder::new()
            .name("system-control-stub-volume".to_string())
            .spawn(move || run_volume_runtime(worker_inner, command_receiver))
            .map_err(|error| SystemControlError::RuntimeThreadSpawnFailed {
                message: error.to_string(),
            })?;

        Ok(Self {
            backend: Arc::new(PlatformVolumeBackend {
                inner,
                command_sender,
            }),
        })
    }

    pub(crate) fn current_state(&self) -> VolumeFeatureState {
        self.backend
            .inner
            .current_state
            .lock()
            .expect("volume stub state poisoned")
            .clone()
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
        let observer = Arc::new(Mutex::new(Box::new(observer) as VolumeObserverCallback));
        self.backend
            .inner
            .observers
            .lock()
            .expect("volume stub observers poisoned")
            .insert(observer_id, observer);
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

impl PlatformVolumeState {
    fn current_state(&self) -> VolumeFeatureState {
        self.current_state
            .lock()
            .expect("volume stub state poisoned")
            .clone()
    }

    fn set_current_state(&self, next_state: VolumeFeatureState) {
        *self
            .current_state
            .lock()
            .expect("volume stub state poisoned") = next_state;
    }

    fn observer(&self, observer_id: u64) -> Option<Arc<Mutex<VolumeObserverCallback>>> {
        self.observers
            .lock()
            .expect("volume stub observers poisoned")
            .get(&observer_id)
            .cloned()
    }

    fn notify(&self, next_state: VolumeFeatureState) {
        let observers = self
            .observers
            .lock()
            .expect("volume stub observers poisoned")
            .values()
            .cloned()
            .collect::<Vec<_>>();

        for observer in observers {
            if let Ok(observer) = observer.lock() {
                observer(next_state.clone());
            }
        }
    }
}

fn run_volume_runtime(inner: Arc<PlatformVolumeState>, receiver: Receiver<VolumeRuntimeMessage>) {
    while let Ok(message) = receiver.recv() {
        match message {
            VolumeRuntimeMessage::NotifyObserver { observer_id } => {
                if let Some(observer) = inner.observer(observer_id)
                    && let Ok(observer) = observer.lock()
                {
                    observer(inner.current_state());
                }
            }
            VolumeRuntimeMessage::SetOutputPercentage { output_percentage } => {
                let next_state = FeatureState::Ready(VolumeState::new(output_percentage, false));
                inner.set_current_state(next_state.clone());
                inner.notify(next_state);
            }
        }
    }
}

impl Drop for PlatformVolumeObservation {
    fn drop(&mut self) {
        if let Some(inner) = self.inner.upgrade()
            && let Ok(mut observers) = inner.observers.lock()
        {
            observers.remove(&self.observer_id);
        }
    }
}
