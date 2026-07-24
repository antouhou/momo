mod style;

use super::{
    compositor::use_compositor_event_inbox,
    model::{LaunchControllerRequest, use_launch_controller_request_channel},
    overview::{WindowSwitchRequest, use_window_switch_request_channel},
    state::{HomeView, use_home_view, use_home_view_request_channel},
};
use daiko::{
    Element, Id,
    channel::Channel,
    component::{Component, ComponentContext},
    integration::{
        SurfaceKeyboardInteractivity, SurfaceLayer,
        input::{Key, KeyState},
    },
    state_management::StateHandle,
    window_events::{WindowEvent, WindowEventData},
};
use momo_app::{TOGGLE_OVERVIEW_SHORTCUT_ID, WINDOW_SWITCH_SHORTCUT_ID};
use momo_compositor::CompositorEvent;
use style::no_view_style;

// Please note that only one Component can read from the channel. It is not enforced by the
//  compiler, but something to keep in mind
const HOME_FOCUS_LOST_CHANNEL_ID: &str = "momo_home_focus_lost_channel";
const HOME_HIDE_SHELL_CHANNEL_ID: &str = "momo_home_hide_shell_channel";
pub(super) const HOME_SURFACE_LAYER_STATE_ID: &str = "momo_home_surface_layer_state";

pub(super) fn use_focus_lost_channel(ctx: &mut ComponentContext) -> Channel<()> {
    ctx.use_channel_with_id(HOME_FOCUS_LOST_CHANNEL_ID)
}

pub(super) fn use_hide_shell_channel(ctx: &mut ComponentContext) -> Channel<()> {
    ctx.use_channel_with_id(HOME_HIDE_SHELL_CHANNEL_ID)
}

#[derive(Clone, Copy)]
pub(super) struct SurfaceLayerController {
    pub(super) launch_is_active: bool,
}

#[derive(Clone, Copy)]
enum FocusEvent {
    Gained,
    Lost,
}

#[derive(Clone, Copy)]
enum ShortcutAction {
    ToggleOverview,
    WindowSwitch,
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
enum VisibilityTransition {
    #[default]
    Stable,
    Showing,
    Hiding,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct OverviewVisit {
    previous_home_view: HomeView,
    previous_surface_layer: SurfaceLayer,
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
struct SurfaceLayerControllerState {
    visibility_transition: VisibilityTransition,
    overview_visit: Option<OverviewVisit>,
    window_switch_is_active: bool,
}

pub(crate) struct SurfaceLayerControl {
    current_layer: StateHandle<SurfaceLayer>,
}

impl SurfaceLayerControl {
    pub(crate) fn current_layer(&self) -> SurfaceLayer {
        *self.current_layer.read()
    }

    fn set_current_layer(&self, layer: SurfaceLayer) {
        if self.current_layer() != layer {
            *self.current_layer.write() = layer;
        }
    }
}

pub(crate) fn use_surface_layer_control(ctx: &mut ComponentContext) -> SurfaceLayerControl {
    SurfaceLayerControl {
        current_layer: ctx
            .use_shared_state(Id::new(HOME_SURFACE_LAYER_STATE_ID), || SurfaceLayer::Top),
    }
}

impl Component for SurfaceLayerController {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let surface_layer_control = use_surface_layer_control(ctx);
        let focus_lost_channel = use_focus_lost_channel(ctx);
        let hide_shell_channel = use_hide_shell_channel(ctx);
        let controller_state = ctx.use_local_state(SurfaceLayerControllerState::default);
        let current_controller_state = *controller_state.read();
        let mut next_controller_state = current_controller_state;
        let current_home_view = use_home_view(ctx);
        let home_view_request_channel = use_home_view_request_channel(ctx);
        let window_switch_request_channel = use_window_switch_request_channel(ctx);
        let launch_controller_request_channel = use_launch_controller_request_channel(ctx);
        let compositor_event_inbox = use_compositor_event_inbox(ctx);
        let (requested_toggle_count, requested_window_switch_count) = {
            let mut compositor_event_inbox = compositor_event_inbox.write_silent();
            compositor_event_inbox
                .pending_events
                .drain(..)
                .filter_map(shortcut_action)
                .fold(
                    (0, 0),
                    |(toggle_count, window_switch_count), action| match action {
                        ShortcutAction::ToggleOverview => (toggle_count + 1, window_switch_count),
                        ShortcutAction::WindowSwitch => (toggle_count, window_switch_count + 1),
                    },
                )
        };
        let (alt_was_released, previous_arrow_count, next_arrow_count) =
            ctx.keyboard_events().iter().fold(
                (false, 0, 0),
                |(alt_was_released, previous_arrow_count, next_arrow_count), event| {
                    match window_switch_navigation_request(event) {
                        Some(WindowSwitchRequest::CyclePrevious) => {
                            (alt_was_released, previous_arrow_count + 1, next_arrow_count)
                        }
                        Some(WindowSwitchRequest::CycleNext) => {
                            (alt_was_released, previous_arrow_count, next_arrow_count + 1)
                        }
                        Some(WindowSwitchRequest::Begin | WindowSwitchRequest::Commit) | None => (
                            alt_was_released || alt_release_event(event),
                            previous_arrow_count,
                            next_arrow_count,
                        ),
                    }
                },
            );
        let latest_focus_event = ctx
            .window_events()
            .iter()
            .filter_map(focus_event)
            .next_back();
        let hide_shell_requested = hide_shell_channel.iter().next().is_some();

        if next_controller_state.overview_visit.is_some() && current_home_view != HomeView::Overview
        {
            next_controller_state.overview_visit = None;
        }

        match latest_focus_event {
            Some(FocusEvent::Gained) => {
                ctx.set_surface_layer(SurfaceLayer::Top);
                surface_layer_control.set_current_layer(SurfaceLayer::Top);
                if next_controller_state.visibility_transition == VisibilityTransition::Showing {
                    if !next_controller_state.window_switch_is_active {
                        ctx.set_surface_keyboard_interactivity(
                            SurfaceKeyboardInteractivity::OnDemand,
                        );
                    }
                    next_controller_state.visibility_transition = VisibilityTransition::Stable;
                }
            }
            Some(FocusEvent::Lost)
                if next_controller_state.visibility_transition == VisibilityTransition::Hiding =>
            {
                ctx.set_surface_keyboard_interactivity(SurfaceKeyboardInteractivity::OnDemand);
                next_controller_state.visibility_transition = VisibilityTransition::Stable;
            }
            Some(FocusEvent::Lost) if self.launch_is_active => {
                let _ = focus_lost_channel.send(());
                ctx.set_surface_layer(SurfaceLayer::Background);
                surface_layer_control.set_current_layer(SurfaceLayer::Background);
            }
            Some(FocusEvent::Lost) | None => {}
        }

        if hide_shell_requested && !self.launch_is_active {
            next_controller_state.overview_visit = None;
            next_controller_state.window_switch_is_active = false;
            hide_shell(ctx, &surface_layer_control, &mut next_controller_state);
        }

        if !hide_shell_requested && requested_toggle_count % 2 == 1 {
            next_controller_state.window_switch_is_active = false;
            if self.launch_is_active {
                reverse_launch_animation_and_show_overview(
                    ctx,
                    current_home_view,
                    &launch_controller_request_channel,
                    &home_view_request_channel,
                    &surface_layer_control,
                    &mut next_controller_state,
                );
            } else {
                toggle_overview(
                    ctx,
                    current_home_view,
                    &home_view_request_channel,
                    &surface_layer_control,
                    &mut next_controller_state,
                );
            }
        }

        if !hide_shell_requested && requested_window_switch_count > 0 {
            let window_switch_was_active = next_controller_state.window_switch_is_active;
            let cycle_request_count = if window_switch_was_active {
                requested_window_switch_count
            } else {
                next_controller_state.window_switch_is_active = true;
                next_controller_state.overview_visit = None;
                if self.launch_is_active {
                    let _ = launch_controller_request_channel
                        .send(LaunchControllerRequest::ReverseLaunchAnimation);
                }
                if current_home_view != HomeView::Overview {
                    let _ = home_view_request_channel.send(HomeView::Overview);
                }
                show_shell(ctx, &surface_layer_control, &mut next_controller_state);
                let _ = window_switch_request_channel.send(WindowSwitchRequest::Begin);
                requested_window_switch_count - 1
            };

            for _ in 0..cycle_request_count {
                let _ = window_switch_request_channel.send(WindowSwitchRequest::CycleNext);
            }
        }

        if next_controller_state.window_switch_is_active {
            for _ in 0..previous_arrow_count {
                let _ = window_switch_request_channel.send(WindowSwitchRequest::CyclePrevious);
            }
            for _ in 0..next_arrow_count {
                let _ = window_switch_request_channel.send(WindowSwitchRequest::CycleNext);
            }
        }

        if alt_was_released && next_controller_state.window_switch_is_active {
            let _ = window_switch_request_channel.send(WindowSwitchRequest::Commit);
            next_controller_state.window_switch_is_active = false;
            ctx.set_surface_keyboard_interactivity(SurfaceKeyboardInteractivity::OnDemand);
        }

        if next_controller_state != current_controller_state {
            *controller_state.write_silent() = next_controller_state;
        }

        Element::new().with_style(no_view_style())
    }
}

fn reverse_launch_animation_and_show_overview(
    ctx: &mut ComponentContext,
    current_home_view: HomeView,
    launch_controller_request_channel: &Channel<LaunchControllerRequest>,
    home_view_request_channel: &Channel<HomeView>,
    surface_layer_control: &SurfaceLayerControl,
    controller_state: &mut SurfaceLayerControllerState,
) {
    let _ = launch_controller_request_channel.send(LaunchControllerRequest::ReverseLaunchAnimation);
    if controller_state.overview_visit.is_none() {
        controller_state.overview_visit = Some(OverviewVisit {
            previous_home_view: current_home_view,
            previous_surface_layer: surface_layer_control.current_layer(),
        });
    }
    if current_home_view != HomeView::Overview {
        let _ = home_view_request_channel.send(HomeView::Overview);
    }
    show_shell(ctx, surface_layer_control, controller_state);
}

fn toggle_overview(
    ctx: &mut ComponentContext,
    current_home_view: HomeView,
    home_view_request_channel: &Channel<HomeView>,
    surface_layer_control: &SurfaceLayerControl,
    controller_state: &mut SurfaceLayerControllerState,
) {
    if let Some(active_visit) = controller_state.overview_visit {
        if current_home_view != active_visit.previous_home_view {
            let _ = home_view_request_channel.send(active_visit.previous_home_view);
        }
        if active_visit.previous_surface_layer == SurfaceLayer::Background {
            hide_shell(ctx, surface_layer_control, controller_state);
        }
        controller_state.overview_visit = None;
        return;
    }

    let current_surface_layer = surface_layer_control.current_layer();
    if current_surface_layer != SurfaceLayer::Background && current_home_view == HomeView::Overview
    {
        hide_shell(ctx, surface_layer_control, controller_state);
        return;
    }

    controller_state.overview_visit = Some(OverviewVisit {
        previous_home_view: current_home_view,
        previous_surface_layer: current_surface_layer,
    });
    if current_home_view != HomeView::Overview {
        let _ = home_view_request_channel.send(HomeView::Overview);
    }
    if current_surface_layer == SurfaceLayer::Background {
        show_shell(ctx, surface_layer_control, controller_state);
    }
}

fn show_shell(
    ctx: &mut ComponentContext,
    surface_layer_control: &SurfaceLayerControl,
    controller_state: &mut SurfaceLayerControllerState,
) {
    ctx.set_surface_layer(SurfaceLayer::Top);
    ctx.set_surface_keyboard_interactivity(SurfaceKeyboardInteractivity::Exclusive);
    surface_layer_control.set_current_layer(SurfaceLayer::Top);
    controller_state.visibility_transition = VisibilityTransition::Showing;
}

fn hide_shell(
    ctx: &mut ComponentContext,
    surface_layer_control: &SurfaceLayerControl,
    controller_state: &mut SurfaceLayerControllerState,
) {
    ctx.set_surface_keyboard_interactivity(SurfaceKeyboardInteractivity::None);
    ctx.set_surface_layer(SurfaceLayer::Background);
    surface_layer_control.set_current_layer(SurfaceLayer::Background);
    controller_state.visibility_transition = VisibilityTransition::Hiding;
}

fn focus_event(event: &WindowEvent) -> Option<FocusEvent> {
    match &event.data {
        WindowEventData::FocusGained => Some(FocusEvent::Gained),
        WindowEventData::FocusLost => Some(FocusEvent::Lost),
        _ => None,
    }
}

fn shortcut_action(event: CompositorEvent) -> Option<ShortcutAction> {
    let CompositorEvent::ShortcutActivated(shortcut_id) = event else {
        return None;
    };
    if shortcut_id == TOGGLE_OVERVIEW_SHORTCUT_ID {
        Some(ShortcutAction::ToggleOverview)
    } else if shortcut_id == WINDOW_SWITCH_SHORTCUT_ID {
        Some(ShortcutAction::WindowSwitch)
    } else {
        None
    }
}

fn alt_release_event(event: &daiko::keyboard::KeyboardKeyState) -> bool {
    event.key_state() == KeyState::Released
        && matches!(
            event.key(),
            Key::Alt | Key::AltGraph | Key::AltLeft | Key::AltRight
        )
}

fn window_switch_navigation_request(
    event: &daiko::keyboard::KeyboardKeyState,
) -> Option<WindowSwitchRequest> {
    if !event.is_pressed() {
        return None;
    }
    match event.key() {
        Key::ArrowLeft => Some(WindowSwitchRequest::CyclePrevious),
        Key::ArrowRight => Some(WindowSwitchRequest::CycleNext),
        _ => None,
    }
}
