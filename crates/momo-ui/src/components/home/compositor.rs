use daiko::{AppContext, Id, component::ComponentContext, state_management::StateHandle};
use momo_compositor::{CompositorCommandSender, CompositorEvent, CompositorSnapshot};
use std::sync::mpsc::Receiver;

pub(super) const HOME_COMPOSITOR_EVENT_INBOX_STATE_ID: &str =
    "momo_home_compositor_event_inbox_state";
const HOME_COMPOSITOR_INTEGRATION_STATE_ID: &str = "momo_home_compositor_integration_state";

pub(super) struct CompositorIntegrationState {
    pub(super) snapshot: CompositorSnapshot,
    pub(super) command_sender: Option<CompositorCommandSender>,
}

#[derive(Default)]
pub(super) struct CompositorEventInbox {
    pub(super) pending_events: Vec<CompositorEvent>,
}

pub(crate) fn initialize_compositor_events(
    app_context: &mut AppContext,
    initial_snapshot: CompositorSnapshot,
    command_sender: Option<CompositorCommandSender>,
    event_receiver: Option<Receiver<CompositorEvent>>,
) {
    let event_inbox = app_context.peek_global_state(
        Id::new(HOME_COMPOSITOR_EVENT_INBOX_STATE_ID),
        CompositorEventInbox::default,
    );
    let integration_state =
        app_context.peek_global_state(Id::new(HOME_COMPOSITOR_INTEGRATION_STATE_ID), move || {
            CompositorIntegrationState {
                snapshot: initial_snapshot,
                command_sender,
            }
        });
    let Some(event_receiver) = event_receiver else {
        return;
    };

    std::thread::spawn(move || {
        while let Ok(event) = event_receiver.recv() {
            if let CompositorEvent::SnapshotChanged(snapshot) = &event {
                integration_state.write().snapshot = snapshot.clone();
            }
            event_inbox.write().pending_events.push(event);
        }
    });
}

pub(super) fn use_compositor_integration_state(
    ctx: &mut ComponentContext,
) -> StateHandle<CompositorIntegrationState> {
    ctx.use_global_state(Id::new(HOME_COMPOSITOR_INTEGRATION_STATE_ID), || {
        CompositorIntegrationState {
            snapshot: CompositorSnapshot::default(),
            command_sender: None,
        }
    })
}

pub(super) fn use_compositor_event_inbox(
    ctx: &mut ComponentContext,
) -> StateHandle<CompositorEventInbox> {
    ctx.use_global_state(
        Id::new(HOME_COMPOSITOR_EVENT_INBOX_STATE_ID),
        CompositorEventInbox::default,
    )
}
