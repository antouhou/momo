use crate::components::home::launch::{
    HOME_LAUNCH_OVERLAY_EVENT_CHANNEL_ID, LaunchOverlayEvent, LaunchPhase, LaunchTransitionState,
};
use crate::components::home::model::HOME_LAUNCH_CHANNEL_ID;
use daiko::component::ComponentContext;
use daiko::navigation::{FocusOrigin, NavigationInputAction};

pub(in crate::components::home) struct LaunchControllerOutput {
    pub active_launch: Option<LaunchTransitionState>,
    pub preferred_focus_app_id: Option<&'static str>,
    pub launched_app_id: Option<&'static str>,
}

pub(in crate::components::home) fn use_launch_controller(
    ctx: &mut ComponentContext,
) -> LaunchControllerOutput {
    let launch_channel = ctx.use_channel_with_id(HOME_LAUNCH_CHANNEL_ID);
    let overlay_event_channel = ctx.use_channel_with_id(HOME_LAUNCH_OVERLAY_EVENT_CHANNEL_ID);
    let launch_state = ctx.use_local_state(|| None::<LaunchTransitionState>);
    let restore_focus_app_id = ctx.use_local_state(|| None::<&'static str>);
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

    if let Some(request) = next_launch {
        *launch_state.write() = Some(LaunchTransitionState {
            request,
            phase: LaunchPhase::Expanding,
        });
        *restore_focus_app_id.write() = None;
        launch_focusable.request_focus(FocusOrigin::Programmatic);
        launch_focusable.engage();
    }

    let mut launch = *launch_state.read();
    if let Some(active_launch) = launch
        && overlay_expanded_app_id == Some(active_launch.request.app.id)
        && active_launch.phase == LaunchPhase::Expanding
    {
        *launch_state.write() = Some(LaunchTransitionState {
            phase: LaunchPhase::WaitingForSurface,
            ..active_launch
        });
        launch = *launch_state.read();
    }

    if let Some(active_launch) = launch
        && overlay_contracted_app_id == Some(active_launch.request.app.id)
        && active_launch.phase == LaunchPhase::Contracting
    {
        *restore_focus_app_id.write() = Some(active_launch.request.app.id);
        *launch_state.write() = None;
        launch_focusable.disengage();
        launch_focusable.clear_focus();
        home_scope.request_focus(FocusOrigin::Navigation);
        launch = None;
    }

    if launch.is_some() && !launch_focusable.is_focused() {
        launch_focusable.request_focus(FocusOrigin::Programmatic);
    }
    if launch.is_some() && !launch_focusable.is_engaged() {
        launch_focusable.engage();
    }

    let should_reverse_launch = launch_focusable.just_cancelled()
        || launch_focusable.drain_captured_actions().any(|action| {
            matches!(
                action,
                NavigationInputAction::Cancel | NavigationInputAction::Back
            )
        });

    if let Some(active_launch) = launch
        && should_reverse_launch
        && active_launch.phase != LaunchPhase::Contracting
    {
        *launch_state.write() = Some(LaunchTransitionState {
            phase: LaunchPhase::Contracting,
            ..active_launch
        });
        launch = *launch_state.read();
    }

    LaunchControllerOutput {
        launched_app_id: launch.map(|active| active.request.app.id),
        preferred_focus_app_id: *restore_focus_app_id.read(),
        active_launch: launch,
    }
}
