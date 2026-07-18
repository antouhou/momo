use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::Duration,
};

use momo_compositor::{
    BackendMetadata, CapabilitySet, CompositorAction, CompositorBackend, CompositorCommand,
    CompositorError, CompositorEvent, CompositorSnapshot, ConnectionConfiguration,
};

use super::ShellApp;
use crate::{ShellConfiguration, ShellMode};

struct FakeBackend {
    connected: Arc<AtomicBool>,
    action_registered: Arc<AtomicBool>,
    action_sent: bool,
}

impl CompositorBackend for FakeBackend {
    fn metadata(&self) -> BackendMetadata {
        BackendMetadata { name: "fake" }
    }

    fn capabilities(&self) -> CapabilitySet {
        CapabilitySet::default()
    }

    fn connect(&mut self, _configuration: &ConnectionConfiguration) -> Result<(), CompositorError> {
        self.connected.store(true, Ordering::SeqCst);
        Ok(())
    }

    fn snapshot(&self) -> Result<CompositorSnapshot, CompositorError> {
        Ok(CompositorSnapshot::default())
    }

    fn dispatch(&mut self, command: CompositorCommand) -> Result<(), CompositorError> {
        assert_eq!(
            command,
            CompositorCommand::RegisterAction(CompositorAction::ToggleLauncher)
        );
        self.action_registered.store(true, Ordering::SeqCst);
        Ok(())
    }

    fn poll_events(&mut self) -> Result<Vec<CompositorEvent>, CompositorError> {
        if self.action_sent {
            thread::sleep(Duration::from_millis(1));
            Ok(Vec::new())
        } else {
            self.action_sent = true;
            Ok(vec![CompositorEvent::ActionActivated(
                CompositorAction::ToggleLauncher,
            )])
        }
    }
}

#[test]
fn shell_mode_registers_and_forwards_the_launcher_action() {
    let connected = Arc::new(AtomicBool::new(false));
    let action_registered = Arc::new(AtomicBool::new(false));
    let backend = FakeBackend {
        connected: Arc::clone(&connected),
        action_registered: Arc::clone(&action_registered),
        action_sent: false,
    };
    let app = ShellApp::new(
        ShellConfiguration {
            mode: ShellMode::Shell,
        },
        backend,
    );

    let mut started = app.start().expect("shell should start");
    let event_receiver = started
        .runtime
        .as_mut()
        .and_then(super::CompositorRuntime::take_event_receiver)
        .expect("shell mode should expose compositor events");
    let event = event_receiver
        .recv_timeout(Duration::from_secs(1))
        .expect("launcher action should arrive");

    assert!(connected.load(Ordering::SeqCst));
    assert!(action_registered.load(Ordering::SeqCst));
    assert_eq!(
        event,
        CompositorEvent::ActionActivated(CompositorAction::ToggleLauncher)
    );
}

#[test]
fn standalone_mode_does_not_connect_or_register_actions() {
    let connected = Arc::new(AtomicBool::new(false));
    let action_registered = Arc::new(AtomicBool::new(false));
    let backend = FakeBackend {
        connected: Arc::clone(&connected),
        action_registered: Arc::clone(&action_registered),
        action_sent: false,
    };
    let app = ShellApp::new(
        ShellConfiguration {
            mode: ShellMode::Standalone,
        },
        backend,
    );

    let started = app.start().expect("standalone app should start");

    assert!(started.runtime.is_none());
    assert!(!connected.load(Ordering::SeqCst));
    assert!(!action_registered.load(Ordering::SeqCst));
}
