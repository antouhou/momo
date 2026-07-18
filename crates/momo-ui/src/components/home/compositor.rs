use std::sync::mpsc::Receiver;

use daiko::{AppContext, Id, component::ComponentContext};
use momo_compositor::{CompositorAction, CompositorEvent};
use tracing::warn;

pub(super) const HOME_COMPOSITOR_ACTION_STATE_ID: &str = "momo_home_compositor_action_state";

#[derive(Clone, Copy, Default)]
pub(super) struct CompositorActionState {
    pub(super) launcher_toggle_revision: u64,
}

pub(crate) fn initialize_compositor_actions(
    app_context: &mut AppContext,
    event_receiver: Option<Receiver<CompositorEvent>>,
) {
    let action_state = app_context.peek_global_state(
        Id::new(HOME_COMPOSITOR_ACTION_STATE_ID),
        CompositorActionState::default,
    );
    let Some(event_receiver) = event_receiver else {
        return;
    };

    std::thread::spawn(move || {
        while let Ok(event) = event_receiver.recv() {
            match event {
                CompositorEvent::ActionActivated(CompositorAction::ToggleLauncher) => {
                    let next_revision =
                        action_state.read().launcher_toggle_revision.wrapping_add(1);
                    action_state.write().launcher_toggle_revision = next_revision;
                }
                CompositorEvent::Disconnected => {
                    warn!("compositor integration disconnected");
                }
                CompositorEvent::Connected
                | CompositorEvent::SnapshotChanged(_)
                | CompositorEvent::ViewFocused { .. }
                | CompositorEvent::WorkspaceChanged { .. } => {}
            }
        }
    });
}

pub(super) fn take_launcher_toggle_count(ctx: &mut ComponentContext) -> u64 {
    let action_state = ctx.use_global_state(
        Id::new(HOME_COMPOSITOR_ACTION_STATE_ID),
        CompositorActionState::default,
    );
    let current_revision = action_state.read().launcher_toggle_revision;
    let consumed_revision = ctx.use_local_state(move || current_revision);
    let toggle_count = current_revision.wrapping_sub(*consumed_revision.read());
    *consumed_revision.write_silent() = current_revision;
    toggle_count
}
