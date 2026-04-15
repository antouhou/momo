use daiko::state_management::StateHandle;
use std::process::Command;
use std::thread;
use std::time::Duration;

pub(super) fn read_system_time() -> String {
    Command::new("date")
        .arg("+%H:%M")
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|time| time.trim().to_string())
        .filter(|time| !time.is_empty())
        .unwrap_or_else(|| "--:--".to_string())
}


pub(super) fn spawn_clock_thread(clock_text: StateHandle<String>) {
    thread::spawn(move || {
        loop {
            let next_time = read_system_time();
            let needs_update = {
                let current_time = clock_text.read().clone();
                current_time != next_time
            };

            if needs_update {
                *clock_text.write() = next_time;
            }

            thread::sleep(Duration::from_secs(1));
        }
    });
}
