use super::{ShellApp, TOGGLE_OVERVIEW_SHORTCUT_ID, WINDOW_SWITCH_SHORTCUT_ID};
use crate::{ShellConfiguration, ShellMode};
use momo_compositor::{
    BackendMetadata, CapabilitySet, CompositorBackend, CompositorError, CompositorEvent,
    CompositorSession, CompositorSnapshot, CompositorStartupConfiguration, ShortcutRegistration,
    ShortcutTrigger,
};
use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

struct FakeBackend {
    connected: Arc<AtomicBool>,
    shortcut_registered: Arc<AtomicBool>,
    event_loop_stopped: Arc<AtomicBool>,
}

impl CompositorBackend for FakeBackend {
    fn metadata(&self) -> BackendMetadata {
        BackendMetadata { name: "fake" }
    }

    fn capabilities(&self) -> CapabilitySet {
        CapabilitySet::default()
    }

    fn start(
        self,
        configuration: CompositorStartupConfiguration,
    ) -> Result<CompositorSession, CompositorError> {
        self.connected.store(true, Ordering::SeqCst);
        assert_eq!(
            configuration.shortcuts,
            vec![
                ShortcutRegistration {
                    id: TOGGLE_OVERVIEW_SHORTCUT_ID,
                    trigger: ShortcutTrigger::super_key(),
                },
                ShortcutRegistration {
                    id: WINDOW_SWITCH_SHORTCUT_ID,
                    trigger: ShortcutTrigger::alt_tab(),
                },
            ]
        );
        self.shortcut_registered.store(true, Ordering::SeqCst);
        CompositorSession::spawn(
            BackendMetadata { name: "fake" },
            CapabilitySet::default(),
            CompositorSnapshot::default(),
            move |event_sender, _command_receiver, shutdown_receiver| {
                event_sender
                    .send(CompositorEvent::ShortcutActivated(
                        TOGGLE_OVERVIEW_SHORTCUT_ID,
                    ))
                    .map_err(|error| CompositorError::new(error.to_string()))?;
                shutdown_receiver.blocking_wait();
                self.event_loop_stopped.store(true, Ordering::SeqCst);
                Ok(())
            },
        )
    }
}

#[test]
fn shell_mode_registers_shortcuts_and_forwards_the_overview_shortcut() {
    let connected = Arc::new(AtomicBool::new(false));
    let shortcut_registered = Arc::new(AtomicBool::new(false));
    let event_loop_stopped = Arc::new(AtomicBool::new(false));
    let backend = FakeBackend {
        connected: Arc::clone(&connected),
        shortcut_registered: Arc::clone(&shortcut_registered),
        event_loop_stopped: Arc::clone(&event_loop_stopped),
    };
    let app = ShellApp::new(
        ShellConfiguration {
            mode: ShellMode::Shell,
        },
        backend,
    );

    let mut started = app.start().expect("shell should start");
    let event_receiver = started
        .compositor_session
        .as_mut()
        .and_then(CompositorSession::take_event_receiver)
        .expect("shell mode should expose compositor events");
    let event = event_receiver
        .recv_timeout(Duration::from_secs(1))
        .expect("launcher shortcut should arrive");

    assert!(connected.load(Ordering::SeqCst));
    assert!(shortcut_registered.load(Ordering::SeqCst));
    assert_eq!(
        event,
        CompositorEvent::ShortcutActivated(TOGGLE_OVERVIEW_SHORTCUT_ID)
    );
    started
        .compositor_session
        .as_mut()
        .expect("shell mode should have a compositor runtime")
        .stop();
    assert!(event_loop_stopped.load(Ordering::SeqCst));
}

#[test]
fn standalone_mode_does_not_connect_or_register_shortcuts() {
    let connected = Arc::new(AtomicBool::new(false));
    let shortcut_registered = Arc::new(AtomicBool::new(false));
    let event_loop_stopped = Arc::new(AtomicBool::new(false));
    let backend = FakeBackend {
        connected: Arc::clone(&connected),
        shortcut_registered: Arc::clone(&shortcut_registered),
        event_loop_stopped: Arc::clone(&event_loop_stopped),
    };
    let app = ShellApp::new(
        ShellConfiguration {
            mode: ShellMode::Standalone,
        },
        backend,
    );

    let started = app.start().expect("standalone app should start");

    assert!(started.compositor_session.is_none());
    assert!(!connected.load(Ordering::SeqCst));
    assert!(!shortcut_registered.load(Ordering::SeqCst));
    assert!(!event_loop_stopped.load(Ordering::SeqCst));
}
