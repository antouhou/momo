use std::collections::BTreeMap;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::sync::{Arc, Mutex, Weak};
use std::time::Duration;

use crate::SystemControlError;
use crate::battery::{
    BatteryChargingState, BatteryFeatureState, BatteryState, BatteryUnavailableReason,
    BatteryUnsupportedReason,
};
use crate::bluetooth::FeatureState;
use crate::volume::{VolumeFeatureState, VolumeRequestError, VolumeState, VolumeUnavailableReason};

const DEFAULT_AUDIO_SINK: &str = "@DEFAULT_AUDIO_SINK@";
const POWER_SUPPLY_PATH: &str = "/sys/class/power_supply";
const VOLUME_SUBSCRIBE_RESTART_DELAY: Duration = Duration::from_secs(5);
const BATTERY_POLL_INTERVAL: Duration = Duration::from_secs(5);

type VolumeObserverCallback = Box<dyn Fn(VolumeFeatureState) + Send + 'static>;
type BatteryObserverCallback = Box<dyn Fn(BatteryFeatureState) + Send + 'static>;

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

struct LinuxVolumeState {
    current_state: Mutex<VolumeFeatureState>,
    observers: Mutex<BTreeMap<u64, Arc<Mutex<VolumeObserverCallback>>>>,
    next_observer_id: AtomicU64,
}

enum VolumeRuntimeMessage {
    NotifyObserver { observer_id: u64 },
    Refresh,
    SetOutputPercentage { output_percentage: u8 },
}

impl PlatformVolumeHandle {
    pub(crate) fn new() -> Result<Self, SystemControlError> {
        let inner = Arc::new(LinuxVolumeState {
            current_state: Mutex::new(FeatureState::Loading),
            observers: Mutex::new(BTreeMap::new()),
            next_observer_id: AtomicU64::new(1),
        });
        let (command_sender, command_receiver) = channel();
        let worker_inner = inner.clone();
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

impl LinuxVolumeState {
    fn current_state(&self) -> VolumeFeatureState {
        self.current_state
            .lock()
            .expect("Linux volume state poisoned")
            .clone()
    }

    fn set_current_state(&self, next_state: VolumeFeatureState) -> bool {
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

    fn add_observer(&self, observer_id: u64, observer: VolumeObserverCallback) {
        self.observers
            .lock()
            .expect("Linux volume observers poisoned")
            .insert(observer_id, Arc::new(Mutex::new(observer)));
    }

    fn remove_observer(&self, observer_id: u64) {
        if let Ok(mut observers) = self.observers.lock() {
            observers.remove(&observer_id);
        }
    }

    fn observer(&self, observer_id: u64) -> Option<Arc<Mutex<VolumeObserverCallback>>> {
        self.observers
            .lock()
            .expect("Linux volume observers poisoned")
            .get(&observer_id)
            .cloned()
    }

    fn notify(&self, next_state: VolumeFeatureState) {
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

fn run_volume_runtime(
    inner: Arc<LinuxVolumeState>,
    receiver: Receiver<VolumeRuntimeMessage>,
    sender: Sender<VolumeRuntimeMessage>,
) {
    spawn_volume_event_listener(sender);
    refresh_volume_state(&inner);

    while let Ok(message) = receiver.recv() {
        match message {
            VolumeRuntimeMessage::NotifyObserver { observer_id } => {
                if let Some(observer) = inner.observer(observer_id)
                    && let Ok(observer) = observer.lock()
                {
                    observer(inner.current_state());
                }
            }
            VolumeRuntimeMessage::Refresh => {
                refresh_volume_state(&inner);
            }
            VolumeRuntimeMessage::SetOutputPercentage { output_percentage } => {
                let _ = set_linux_output_volume(output_percentage);
                refresh_volume_state(&inner);
            }
        }
    }
}

fn spawn_volume_event_listener(sender: Sender<VolumeRuntimeMessage>) {
    let _ = std::thread::Builder::new()
        .name("system-control-linux-volume-events".to_string())
        .spawn(move || run_volume_event_listener(sender));
}

fn run_volume_event_listener(sender: Sender<VolumeRuntimeMessage>) {
    loop {
        if sender.send(VolumeRuntimeMessage::Refresh).is_err() {
            break;
        }

        let mut child = match Command::new("pactl")
            .arg("subscribe")
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
        {
            Ok(child) => child,
            Err(_) => {
                std::thread::sleep(VOLUME_SUBSCRIBE_RESTART_DELAY);
                continue;
            }
        };

        if let Some(stdout) = child.stdout.take() {
            for line in BufReader::new(stdout).lines() {
                let Ok(line) = line else {
                    break;
                };

                if is_relevant_volume_event(&line)
                    && sender.send(VolumeRuntimeMessage::Refresh).is_err()
                {
                    let _ = child.kill();
                    let _ = child.wait();
                    return;
                }
            }
        }

        let _ = child.kill();
        let _ = child.wait();
        std::thread::sleep(VOLUME_SUBSCRIBE_RESTART_DELAY);
    }
}

fn is_relevant_volume_event(line: &str) -> bool {
    line.contains(" on sink ") || line.contains(" on server")
}

fn refresh_volume_state(inner: &LinuxVolumeState) {
    let next_state = read_linux_volume_state();
    if inner.set_current_state(next_state.clone()) {
        inner.notify(next_state);
    }
}

fn read_linux_volume_state() -> VolumeFeatureState {
    let output = match Command::new("wpctl")
        .args(["get-volume", DEFAULT_AUDIO_SINK])
        .output()
    {
        Ok(output) => output,
        Err(error) => {
            return FeatureState::Unavailable(VolumeUnavailableReason::BackendUnavailable {
                message: format!("failed to run wpctl: {error}"),
            });
        }
    };

    if !output.status.success() {
        return FeatureState::Unavailable(VolumeUnavailableReason::BackendUnavailable {
            message: command_error_message("wpctl get-volume", &output.stderr),
        });
    }

    match parse_wpctl_volume_output(&String::from_utf8_lossy(&output.stdout)) {
        Some(state) => FeatureState::Ready(state),
        None => FeatureState::Unavailable(VolumeUnavailableReason::BackendUnavailable {
            message: "failed to parse wpctl volume output".to_string(),
        }),
    }
}

fn set_linux_output_volume(output_percentage: u8) -> Result<(), VolumeRequestError> {
    let output_percentage = output_percentage.min(100);
    let volume_argument = format!("{output_percentage}%");
    let output = Command::new("wpctl")
        .args(["set-volume", DEFAULT_AUDIO_SINK, &volume_argument])
        .output()
        .map_err(|_| VolumeRequestError::RuntimeUnavailable)?;

    if output.status.success() {
        Ok(())
    } else {
        Err(VolumeRequestError::RuntimeUnavailable)
    }
}

fn parse_wpctl_volume_output(output: &str) -> Option<VolumeState> {
    let trimmed = output.trim();
    let value = trimmed
        .strip_prefix("Volume:")
        .unwrap_or(trimmed)
        .split_whitespace()
        .next()?
        .parse::<f32>()
        .ok()?;
    let output_percentage = (value * 100.0).round().clamp(0.0, 100.0) as u8;
    let is_muted = trimmed.contains("[MUTED]");

    Some(VolumeState::new(output_percentage, is_muted))
}

impl Drop for PlatformVolumeObservation {
    fn drop(&mut self) {
        if let Some(inner) = self.inner.upgrade() {
            inner.remove_observer(self.observer_id);
        }
    }
}

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

struct LinuxBatteryState {
    current_state: Mutex<BatteryFeatureState>,
    observers: Mutex<BTreeMap<u64, Arc<Mutex<BatteryObserverCallback>>>>,
    next_observer_id: AtomicU64,
}

enum BatteryRuntimeMessage {
    NotifyObserver { observer_id: u64 },
}

impl PlatformBatteryHandle {
    pub(crate) fn new() -> Result<Self, SystemControlError> {
        let inner = Arc::new(LinuxBatteryState {
            current_state: Mutex::new(FeatureState::Loading),
            observers: Mutex::new(BTreeMap::new()),
            next_observer_id: AtomicU64::new(1),
        });
        let (command_sender, command_receiver) = channel();
        let worker_inner = inner.clone();
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

impl LinuxBatteryState {
    fn current_state(&self) -> BatteryFeatureState {
        self.current_state
            .lock()
            .expect("Linux battery state poisoned")
            .clone()
    }

    fn set_current_state(&self, next_state: BatteryFeatureState) -> bool {
        let mut current_state = self
            .current_state
            .lock()
            .expect("Linux battery state poisoned");
        if *current_state == next_state {
            return false;
        }

        *current_state = next_state;
        true
    }

    fn add_observer(&self, observer_id: u64, observer: BatteryObserverCallback) {
        self.observers
            .lock()
            .expect("Linux battery observers poisoned")
            .insert(observer_id, Arc::new(Mutex::new(observer)));
    }

    fn remove_observer(&self, observer_id: u64) {
        if let Ok(mut observers) = self.observers.lock() {
            observers.remove(&observer_id);
        }
    }

    fn observer(&self, observer_id: u64) -> Option<Arc<Mutex<BatteryObserverCallback>>> {
        self.observers
            .lock()
            .expect("Linux battery observers poisoned")
            .get(&observer_id)
            .cloned()
    }

    fn notify(&self, next_state: BatteryFeatureState) {
        let observers = self
            .observers
            .lock()
            .expect("Linux battery observers poisoned")
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

fn run_battery_runtime(inner: Arc<LinuxBatteryState>, receiver: Receiver<BatteryRuntimeMessage>) {
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

fn read_linux_battery_state() -> BatteryFeatureState {
    let battery_path = match find_primary_battery_path() {
        Ok(Some(path)) => path,
        Ok(None) => {
            return FeatureState::Unsupported(BatteryUnsupportedReason::NoBatteryPresent);
        }
        Err(error) => {
            return FeatureState::Unavailable(BatteryUnavailableReason::BackendUnavailable {
                message: format!("failed to inspect power supplies: {error}"),
            });
        }
    };

    let capacity = match read_trimmed(battery_path.join("capacity")) {
        Ok(capacity) => capacity,
        Err(error) => {
            return FeatureState::Unavailable(BatteryUnavailableReason::BackendUnavailable {
                message: format!("failed to read battery capacity: {error}"),
            });
        }
    };
    let percentage = match capacity.parse::<u8>() {
        Ok(percentage) => percentage,
        Err(error) => {
            return FeatureState::Unavailable(BatteryUnavailableReason::BackendUnavailable {
                message: format!("failed to parse battery capacity: {error}"),
            });
        }
    };
    let status =
        read_trimmed(battery_path.join("status")).unwrap_or_else(|_| "Unknown".to_string());

    FeatureState::Ready(BatteryState::new(
        percentage,
        parse_battery_charging_state(&status),
    ))
}

fn find_primary_battery_path() -> std::io::Result<Option<PathBuf>> {
    let mut batteries = Vec::new();

    for entry in fs::read_dir(POWER_SUPPLY_PATH)? {
        let entry = entry?;
        let path = entry.path();
        if read_trimmed(path.join("type"))
            .map(|power_supply_type| power_supply_type == "Battery")
            .unwrap_or(false)
        {
            batteries.push(path);
        }
    }

    batteries.sort();
    Ok(batteries.into_iter().next())
}

fn read_trimmed(path: impl AsRef<Path>) -> std::io::Result<String> {
    fs::read_to_string(path).map(|value| value.trim().to_string())
}

fn parse_battery_charging_state(status: &str) -> BatteryChargingState {
    match status {
        "Charging" => BatteryChargingState::Charging,
        "Discharging" => BatteryChargingState::Discharging,
        "Full" => BatteryChargingState::Full,
        _ => BatteryChargingState::Unknown,
    }
}

fn command_error_message(command: &str, stderr: &[u8]) -> String {
    let stderr = String::from_utf8_lossy(stderr);
    let stderr = stderr.trim();
    if stderr.is_empty() {
        format!("{command} failed")
    } else {
        format!("{command} failed: {stderr}")
    }
}

impl Drop for PlatformBatteryObservation {
    fn drop(&mut self) {
        if let Some(inner) = self.inner.upgrade() {
            inner.remove_observer(self.observer_id);
        }
    }
}
