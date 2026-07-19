use daiko::{AppContext, Id, component::ComponentContext, state_management::StateHandle};
use momo_compositor::CompositorEvent;
use std::sync::mpsc::Receiver;

pub(super) const HOME_COMPOSITOR_EVENT_INBOX_STATE_ID: &str =
    "momo_home_compositor_event_inbox_state";

#[derive(Default)]
pub(super) struct CompositorEventInbox {
    pub(super) pending_events: Vec<CompositorEvent>,
}

pub(crate) fn initialize_compositor_events(
    app_context: &mut AppContext,
    event_receiver: Option<Receiver<CompositorEvent>>,
) {
    let event_inbox = app_context.peek_global_state(
        Id::new(HOME_COMPOSITOR_EVENT_INBOX_STATE_ID),
        CompositorEventInbox::default,
    );
    let Some(event_receiver) = event_receiver else {
        return;
    };

    std::thread::spawn(move || {
        while let Ok(event) = event_receiver.recv() {
            event_inbox.write().pending_events.push(event);
        }
    });
}

pub(super) fn use_compositor_event_inbox(
    ctx: &mut ComponentContext,
) -> StateHandle<CompositorEventInbox> {
    ctx.use_global_state(
        Id::new(HOME_COMPOSITOR_EVENT_INBOX_STATE_ID),
        CompositorEventInbox::default,
    )
}
