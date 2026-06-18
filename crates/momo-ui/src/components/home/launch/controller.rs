use crate::components::home::launch::{
    HOME_LAUNCH_OVERLAY_EVENT_CHANNEL_ID, LaunchOverlayEvent, LaunchPhase, LaunchTransitionState,
};
use crate::components::home::model::HOME_LAUNCH_CHANNEL_ID;
use daiko::component::ComponentContext;
use daiko::navigation::{FocusOrigin, NavigationInputAction};
use daiko::state_management::StateHandle;
use std::sync::Arc;

pub(in crate::components::home) struct LaunchControllerOutput {
    pub active_launch: Option<LaunchTransitionState>,
    pub preferred_focus_app_id: Option<Arc<String>>,
    pub launched_app_id: Option<Arc<String>>,
}

pub(in crate::components::home) fn use_launch_controller(
    ctx: &mut ComponentContext,
) -> LaunchControllerOutput {
    let launch_channel = ctx.use_channel_with_id(HOME_LAUNCH_CHANNEL_ID);
    let overlay_event_channel = ctx.use_channel_with_id(HOME_LAUNCH_OVERLAY_EVENT_CHANNEL_ID);
    let launch_state = ctx.use_local_state(|| None::<LaunchTransitionState>);
    let restore_focus_app_id = ctx.use_local_state(|| None::<Arc<String>>);
    let home_scope = ctx.focus_scope();
    let launch_focusable = ctx.focusable();

    let mut next_launch = None;
    for request in launch_channel.iter() {
        next_launch = Some(request);
    }

    let mut overlay_expanded_app_id = None;
    let mut overlay_contracted_app_id = None;
    for event in overlay_event_channel.iter() {
        match event {
            LaunchOverlayEvent::Expanded { app_id } => overlay_expanded_app_id = Some(app_id),
            LaunchOverlayEvent::Contracted { app_id } => overlay_contracted_app_id = Some(app_id),
        }
    }

    let launch_is_active = launch_state.read().is_some() || next_launch.is_some();
    launch_focusable.set_navigation_enabled(launch_is_active);
    launch_focusable.capture_when_engaged(if launch_is_active {
        &[NavigationInputAction::Cancel, NavigationInputAction::Back]
    } else {
        &[]
    });
    let should_reverse_launch = launch_focusable.just_cancelled()
        || launch_focusable.drain_captured_actions().any(|action| {
            matches!(
                action,
                NavigationInputAction::Cancel | NavigationInputAction::Back
            )
        });

    if let Some(request) = next_launch {
        *launch_state.write() = Some(LaunchTransitionState {
            request,
            phase: LaunchPhase::Expanding,
        });
        *restore_focus_app_id.write() = None;
        launch_focusable.request_focus(FocusOrigin::Programmatic);
        launch_focusable.engage();
    }

    let mut launch = launch_state.read().clone();
    if let Some(active_launch) = launch.clone() {
        match active_launch.phase {
            LaunchPhase::Expanding => {
                if should_reverse_launch {
                    set_phase(&launch_state, active_launch, LaunchPhase::Contracting);
                    launch = launch_state.read().clone();
                } else if overlay_expanded_app_id.as_deref().map(String::as_str)
                    == Some(active_launch.request.app.id())
                {
                    set_phase(&launch_state, active_launch, LaunchPhase::WaitingForSurface);
                    launch = launch_state.read().clone();
                }
            }
            LaunchPhase::Contracting => {
                if overlay_contracted_app_id.as_deref().map(String::as_str)
                    == Some(active_launch.request.app.id())
                {
                    *restore_focus_app_id.write() = Some(Arc::clone(&active_launch.request.app.id));
                    *launch_state.write() = None;
                    launch_focusable.disengage();
                    launch_focusable.clear_focus();
                    home_scope.request_focus(FocusOrigin::Navigation);
                    launch = None;
                }
            }
            LaunchPhase::WaitingForSurface => {
                if should_reverse_launch {
                    set_phase(&launch_state, active_launch, LaunchPhase::Contracting);
                    launch = launch_state.read().clone();
                }
            }
        }
    }

    if launch.is_some() && !launch_focusable.is_focused() {
        launch_focusable.request_focus(FocusOrigin::Programmatic);
    }
    if launch.is_some() && !launch_focusable.is_engaged() {
        launch_focusable.engage();
    }

    LaunchControllerOutput {
        launched_app_id: launch
            .as_ref()
            .map(|active| Arc::clone(&active.request.app.id)),
        preferred_focus_app_id: restore_focus_app_id.read().clone(),
        active_launch: launch,
    }
}

fn set_phase(
    launch_state: &StateHandle<Option<LaunchTransitionState>>,
    active_launch: LaunchTransitionState,
    phase: LaunchPhase,
) {
    *launch_state.write() = Some(LaunchTransitionState {
        phase,
        ..active_launch
    });
}
