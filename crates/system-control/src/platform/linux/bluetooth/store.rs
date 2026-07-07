use crate::{bluetooth::BluetoothFeatureState, feature_state::FeatureState};
use std::{
    collections::BTreeMap,
    sync::{
        Arc, Mutex,
        atomic::{AtomicU64, Ordering},
    },
};

type ObserverCallback = Box<dyn Fn(BluetoothFeatureState) + Send + 'static>;

pub(super) struct BackendState {
    current_state: Mutex<BluetoothFeatureState>,
    observers: Mutex<BTreeMap<u64, Arc<Mutex<ObserverCallback>>>>,
    next_observer_id: AtomicU64,
    active_connection_id: AtomicU64,
}

impl BackendState {
    pub(super) fn new() -> Self {
        Self {
            current_state: Mutex::new(FeatureState::Loading),
            observers: Mutex::new(BTreeMap::new()),
            next_observer_id: AtomicU64::new(1),
            active_connection_id: AtomicU64::new(0),
        }
    }

    pub(super) fn next_observer_id(&self) -> u64 {
        self.next_observer_id.fetch_add(1, Ordering::Relaxed)
    }

    pub(super) fn add_observer(&self, observer_id: u64, observer: ObserverCallback) {
        let observer = Arc::new(Mutex::new(observer));
        self.observers
            .lock()
            .expect("poisoned mutex")
            .insert(observer_id, Arc::clone(&observer));

        let current_state = self.current_state();
        observer.lock().expect("poisoned mutex").as_ref()(current_state);
    }

    pub(super) fn remove_observer(&self, observer_id: u64) {
        self.observers
            .lock()
            .expect("poisoned mutex")
            .remove(&observer_id);
    }

    pub(super) fn current_state(&self) -> BluetoothFeatureState {
        self.current_state.lock().expect("poisoned mutex").clone()
    }

    pub(super) fn publish(&self, next_state: BluetoothFeatureState) {
        {
            let mut state = self.current_state.lock().expect("poisoned mutex");
            *state = next_state.clone();
        }

        let observers = self
            .observers
            .lock()
            .expect("poisoned mutex")
            .values()
            .cloned()
            .collect::<Vec<_>>();

        for observer in observers {
            observer.lock().expect("poisoned mutex").as_ref()(next_state.clone());
        }
    }

    pub(super) fn activate_connection(&self, connection_id: u64) {
        self.active_connection_id
            .store(connection_id, Ordering::Relaxed);
    }

    pub(super) fn deactivate_connection(&self, connection_id: u64) {
        let _ = self.active_connection_id.compare_exchange(
            connection_id,
            0,
            Ordering::Relaxed,
            Ordering::Relaxed,
        );
    }

    pub(super) fn clear_active_connection(&self) {
        self.active_connection_id.store(0, Ordering::Relaxed);
    }

    pub(super) fn is_active_connection(&self, connection_id: u64) -> bool {
        self.active_connection_id.load(Ordering::Relaxed) == connection_id
    }
}
