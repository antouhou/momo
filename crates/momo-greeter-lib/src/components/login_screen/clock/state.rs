use daiko::Id;
use daiko::component::ComponentContext;
use daiko::state_management::StateHandle;
use std::process::Command;
use std::thread;
use std::time::Duration;

const CLOCK_STATE_ID: &str = "momo_greeter_clock_text";
const CLOCK_REFRESH_INTERVAL: Duration = Duration::from_secs(1);
const CLOCK_PLACEHOLDER: &str = "--:--";

#[derive(Clone, Copy, Default)]
pub(super) struct ClockLocalState {
    pub(super) worker_started: bool,
}

pub(super) fn clock_text(ctx: &mut ComponentContext) -> StateHandle<String> {
    ctx.use_shared_state(Id::new(CLOCK_STATE_ID), || CLOCK_PLACEHOLDER.to_string())
}

pub(super) fn spawn_clock_thread(clock_text: StateHandle<String>) {
    thread::spawn(move || {
        loop {
            let next_time = read_system_time();
            if *clock_text.read() != next_time {
                *clock_text.write() = next_time;
            }
            thread::sleep(CLOCK_REFRESH_INTERVAL);
        }
    });
}

fn read_system_time() -> String {
    Command::new("date")
        .arg("+%H:%M")
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|time| time.trim().to_string())
        .filter(|time| !time.is_empty())
        .unwrap_or_else(|| CLOCK_PLACEHOLDER.to_string())
}
