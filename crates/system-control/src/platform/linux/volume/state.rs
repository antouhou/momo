use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex, atomic::AtomicU64},
};

use crate::volume::VolumeFeatureState;

pub(super) type VolumeObserverCallback = Box<dyn Fn(VolumeFeatureState) + Send + 'static>;

pub(super) struct LinuxVolumeState {
    current_state: Mutex<VolumeFeatureState>,
    observers: Mutex<BTreeMap<u64, Arc<Mutex<VolumeObserverCallback>>>>,
    pub(super) next_observer_id: AtomicU64,
}

impl LinuxVolumeState {
    pub(super) fn new(current_state: VolumeFeatureState) -> Self {
        Self {
            current_state: Mutex::new(current_state),
            observers: Mutex::new(BTreeMap::new()),
            next_observer_id: AtomicU64::new(1),
        }
    }

    pub(super) fn current_state(&self) -> VolumeFeatureState {
        self.current_state
            .lock()
            .expect("Linux volume state poisoned")
            .clone()
    }

    pub(super) fn set_current_state(&self, next_state: VolumeFeatureState) -> bool {
        let mut current_state = self
            .current_state
            .lock()
            .expect("Linux volume state poisoned");
        if *current_state == next_state {
            return false;
        }

        *current_state = next_state;
        true
    }

    pub(super) fn add_observer(&self, observer_id: u64, observer: VolumeObserverCallback) {
        self.observers
            .lock()
            .expect("Linux volume observers poisoned")
            .insert(observer_id, Arc::new(Mutex::new(observer)));
    }

    pub(super) fn remove_observer(&self, observer_id: u64) {
        if let Ok(mut observers) = self.observers.lock() {
            observers.remove(&observer_id);
        }
    }

    pub(super) fn observer(&self, observer_id: u64) -> Option<Arc<Mutex<VolumeObserverCallback>>> {
        self.observers
            .lock()
            .expect("Linux volume observers poisoned")
            .get(&observer_id)
            .cloned()
    }

    pub(super) fn notify(&self, next_state: VolumeFeatureState) {
        let observers = self
            .observers
            .lock()
            .expect("Linux volume observers poisoned")
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
