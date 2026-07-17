use crate::{
    app_state::{AppOpResult, use_apps_state},
    components::home::{
        launch::{
            HOME_LAUNCH_OVERLAY_EVENT_CHANNEL_ID, LaunchOverlayEvent, LaunchPhase,
            LaunchTransitionState,
        },
        model::{HOME_LAUNCH_CHANNEL_ID, LaunchRequest, LaunchRestoreFocus},
    },
};
use daiko::{
    component::ComponentContext,
    integration::{SurfaceLayer},
    navigation::{FocusKey, FocusOrigin, NavigationInputAction},
    state_management::StateHandle,
    window_events::{WindowEvent, WindowEventData},
};
use std::sync::Arc;
use tracing::error;

pub trait LaunchStateExtension {
    fn set_phase(&self, phase: LaunchPhase);
}

impl LaunchStateExtension for StateHandle<Option<LaunchTransitionState>> {
    fn set_phase(&self, phase: LaunchPhase) {
        let mut guard = self.write();
        if let Some(state) = guard.as_mut() {
            state.phase = phase;
        }
    }
}

pub(in crate::components::home) struct LaunchControllerOutput {
    pub active_launch: Option<LaunchTransitionState>,
    pub preferred_focus_app_id: Option<Arc<String>>,
    pub preferred_dock_focus_key: Option<FocusKey>,
    pub launched_app_id: Option<Arc<String>>,
}

pub(in crate::components::home) fn use_launch_controller(
    ctx: &mut ComponentContext,
) -> LaunchControllerOutput {
    let window_events = ctx.window_events();
    let handoff_signal = detect_launch_handoff_signal(window_events);
    let launch_channel = ctx.use_channel_with_id::<LaunchRequest>(HOME_LAUNCH_CHANNEL_ID);
    let overlay_event_channel = ctx.use_channel_with_id(HOME_LAUNCH_OVERLAY_EVENT_CHANNEL_ID);
    let launch_state_handle = ctx.use_local_state(|| None::<LaunchTransitionState>);
    let restore_focus_app_id = ctx.use_local_state(|| None::<Arc<String>>);
    let restore_dock_focus_key = ctx.use_local_state(|| None::<(Arc<String>, FocusKey)>);
    let home_scope = ctx.focus_scope();
    let home_focusable_handle = ctx.focusable();

    let mut next_launch_request = None;
    for launch_request in launch_channel.iter() {
        next_launch_request = Some(launch_request);
    }

    let mut overlay_expanded_app_id = None;
    let mut overlay_contracted_app_id = None;
    for overlay_event in overlay_event_channel.iter() {
        match overlay_event {
            LaunchOverlayEvent::Expanded { app_id } => overlay_expanded_app_id = Some(app_id),
            LaunchOverlayEvent::Contracted { app_id } => overlay_contracted_app_id = Some(app_id),
        }
    }

    let launch_transition_state = launch_state_handle.read().clone();
    let launch_is_active = launch_transition_state.is_some() || next_launch_request.is_some();

    home_focusable_handle.set_navigation_enabled(launch_is_active);
    home_focusable_handle.capture_when_engaged(if launch_is_active {
        &[NavigationInputAction::Cancel, NavigationInputAction::Back]
    } else {
        &[]
    });

    let just_pressed_cancel = home_focusable_handle
        .drain_captured_actions()
        .any(|action| {
            matches!(
                action,
                NavigationInputAction::Cancel | NavigationInputAction::Back
            )
        });
    let mut should_reverse_launch = home_focusable_handle.just_cancelled() || just_pressed_cancel;

    let apps_state_handle = use_apps_state(ctx);
    let mut current_launch_failed = false;

    {
        let mut apps = apps_state_handle.write_silent();
        let res = &mut apps.app_ops_results;
        let mut launch_results = res.drain(..);
        if let Some(state) = launch_transition_state.as_ref() {
            let current_launch_result = launch_results
                .find(|launch_result| launch_result.is_for_app(state.request.app.id()));

            if let Some(launch_result) = current_launch_result
                && let AppOpResult::LaunchFailed(_, err) = launch_result
            {
                error!(
                    "Error while launching the app {}({}): {:?}\nLaunch info: {:?}",
                    state.request.app.name.as_str(),
                    state.request.app.id(),
                    err.error,
                    err.launch_command_entry
                );
                current_launch_failed = true;
            }
        }
    }

    if launch_transition_state.is_some() && (current_launch_failed || handoff_signal.is_some()) {
        next_launch_request = None;
        should_reverse_launch = true;
    }

    if let Some(request) = next_launch_request {
        ctx.set_surface_layer(SurfaceLayer::Background);
        *launch_state_handle.write() = Some(LaunchTransitionState {
            request,
            phase: LaunchPhase::Expanding,
        });
        *restore_focus_app_id.write() = None;
        *restore_dock_focus_key.write() = None;
        home_focusable_handle.request_focus(FocusOrigin::Programmatic);
        home_focusable_handle.engage();
    }

    let mut launch_transition_state = launch_state_handle.read().clone();
    if let Some(current_launch_transition_state) = launch_transition_state {
        match current_launch_transition_state.phase {
            LaunchPhase::Expanding => {
                if should_reverse_launch {
                    launch_state_handle.set_phase(LaunchPhase::Contracting);
                } else if overlay_expanded_app_id.as_deref().map(String::as_str)
                    == Some(current_launch_transition_state.request.app.id())
                {
                    launch_state_handle.set_phase(LaunchPhase::WaitingForSurface);
                }
            }
            LaunchPhase::Contracting => {
                if overlay_contracted_app_id.as_deref().map(String::as_str)
                    == Some(current_launch_transition_state.request.app.id())
                {
                    match &current_launch_transition_state.request.restore_focus {
                        LaunchRestoreFocus::AppGrid => {
                            *restore_focus_app_id.write() =
                                Some(Arc::clone(&current_launch_transition_state.request.app.id));
                            *restore_dock_focus_key.write() = None;
                        }
                        LaunchRestoreFocus::Dock(focus_key) => {
                            *restore_focus_app_id.write() = None;
                            *restore_dock_focus_key.write() = Some((
                                Arc::clone(&current_launch_transition_state.request.app.id),
                                *focus_key,
                            ));
                        }
                    }
                    *launch_state_handle.write() = None;
                    home_focusable_handle.disengage();
                    home_focusable_handle.clear_focus();
                    home_scope.request_focus(FocusOrigin::Navigation);
                }
            }
            LaunchPhase::WaitingForSurface => {
                if should_reverse_launch {
                    launch_state_handle.set_phase(LaunchPhase::Contracting);
                }
            }
        }
    }

    launch_transition_state = launch_state_handle.read().clone();

    if launch_transition_state.is_some() && !home_focusable_handle.is_focused() {
        home_focusable_handle.request_focus(FocusOrigin::Programmatic);
    }
    if launch_transition_state.is_some() && !home_focusable_handle.is_engaged() {
        home_focusable_handle.engage();
    }

    LaunchControllerOutput {
        launched_app_id: launch_transition_state
            .as_ref()
            .map(|active| Arc::clone(&active.request.app.id)),
        preferred_focus_app_id: restore_focus_app_id.read().clone(),
        preferred_dock_focus_key: restore_dock_focus_key.read().as_ref().map(|val| val.1),
        active_launch: launch_transition_state,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum LaunchHandoffSignal {
    WindowFocusLost,
}

fn detect_launch_handoff_signal(window_events: &[WindowEvent]) -> Option<LaunchHandoffSignal> {
    window_events
        .iter()
        .any(|event| matches!(event.data, WindowEventData::FocusLost))
        .then_some(LaunchHandoffSignal::WindowFocusLost)
}
