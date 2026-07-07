use std::{
    sync::{Arc, mpsc::Receiver},
    time::Duration,
};
use super::{state::LinuxBatteryState, sysfs::read_linux_battery_state};

const BATTERY_POLL_INTERVAL: Duration = Duration::from_secs(5);

pub(super) enum BatteryRuntimeMessage {
    NotifyObserver { observer_id: u64 },
}

pub(super) fn run_battery_runtime(
    inner: Arc<LinuxBatteryState>,
    receiver: Receiver<BatteryRuntimeMessage>,
) {
    refresh_battery_state(&inner);

    loop {
        match receiver.recv_timeout(BATTERY_POLL_INTERVAL) {
            Ok(BatteryRuntimeMessage::NotifyObserver { observer_id }) => {
                if let Some(observer) = inner.observer(observer_id)
                    && let Ok(observer) = observer.lock()
                {
                    observer(inner.current_state());
                }
            }
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                refresh_battery_state(&inner);
            }
            Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => break,
        }
    }
}

fn refresh_battery_state(inner: &LinuxBatteryState) {
    let next_state = read_linux_battery_state();
    if inner.set_current_state(next_state.clone()) {
        inner.notify(next_state);
    }
}
