use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;

use super::pipewire::{read_linux_volume_state, set_linux_output_volume};
use super::state::LinuxVolumeState;

const VOLUME_SUBSCRIBE_RESTART_DELAY: Duration = Duration::from_secs(5);

pub(super) enum VolumeRuntimeMessage {
    NotifyObserver { observer_id: u64 },
    Refresh,
    SetOutputPercentage { output_percentage: u8 },
}

pub(super) fn run_volume_runtime(
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
