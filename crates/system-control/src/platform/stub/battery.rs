use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::sync::{Arc, Mutex, Weak};

use crate::SystemControlError;
use crate::battery::{BatteryChargingState, BatteryFeatureState, BatteryState};
use crate::bluetooth::FeatureState;

const STUB_BATTERY_PERCENTAGE: u8 = 96;

type BatteryObserverCallback = Box<dyn Fn(BatteryFeatureState) + Send + 'static>;

#[derive(Clone)]
pub(crate) struct PlatformBatteryHandle {
    backend: Arc<PlatformBatteryBackend>,
}

pub(crate) struct PlatformBatteryObservation {
    observer_id: u64,
    inner: Weak<PlatformBatteryState>,
}

struct PlatformBatteryBackend {
    inner: Arc<PlatformBatteryState>,
    command_sender: Sender<BatteryRuntimeMessage>,
}

struct PlatformBatteryState {
    current_state: Mutex<BatteryFeatureState>,
    observers: Mutex<BTreeMap<u64, Arc<Mutex<BatteryObserverCallback>>>>,
    next_observer_id: AtomicU64,
}

enum BatteryRuntimeMessage {
    NotifyObserver { observer_id: u64 },
}

impl PlatformBatteryHandle {
    pub(crate) fn new() -> Result<Self, SystemControlError> {
        let inner = Arc::new(PlatformBatteryState {
            current_state: Mutex::new(FeatureState::Ready(BatteryState::new(
                STUB_BATTERY_PERCENTAGE,
                BatteryChargingState::Discharging,
            ))),
            observers: Mutex::new(BTreeMap::new()),
            next_observer_id: AtomicU64::new(1),
        });
        let (command_sender, command_receiver) = channel();
        let worker_inner = Arc::clone(&inner);
        std::thread::Builder::new()
            .name("system-control-stub-battery".to_string())
            .spawn(move || run_battery_runtime(worker_inner, command_receiver))
            .map_err(|error| SystemControlError::RuntimeThreadSpawnFailed {
                message: error.to_string(),
            })?;

        Ok(Self {
            backend: Arc::new(PlatformBatteryBackend {
                inner,
                command_sender,
            }),
        })
    }

    pub(crate) fn current_state(&self) -> BatteryFeatureState {
        self.backend
            .inner
            .current_state
            .lock()
            .expect("battery stub state poisoned")
            .clone()
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
        let observer = Arc::new(Mutex::new(Box::new(observer) as BatteryObserverCallback));
        self.backend
            .inner
            .observers
            .lock()
            .expect("battery stub observers poisoned")
            .insert(observer_id, observer);
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

impl PlatformBatteryState {
    fn current_state(&self) -> BatteryFeatureState {
        self.current_state
            .lock()
            .expect("battery stub state poisoned")
            .clone()
    }

    fn observer(&self, observer_id: u64) -> Option<Arc<Mutex<BatteryObserverCallback>>> {
        self.observers
            .lock()
            .expect("battery stub observers poisoned")
            .get(&observer_id)
            .cloned()
    }
}

fn run_battery_runtime(
    inner: Arc<PlatformBatteryState>,
    receiver: Receiver<BatteryRuntimeMessage>,
) {
    while let Ok(message) = receiver.recv() {
        match message {
            BatteryRuntimeMessage::NotifyObserver { observer_id } => {
                if let Some(observer) = inner.observer(observer_id)
                    && let Ok(observer) = observer.lock()
                {
                    observer(inner.current_state());
                }
            }
        }
    }
}

impl Drop for PlatformBatteryObservation {
    fn drop(&mut self) {
        if let Some(inner) = self.inner.upgrade()
            && let Ok(mut observers) = inner.observers.lock()
        {
            observers.remove(&self.observer_id);
        }
    }
}
