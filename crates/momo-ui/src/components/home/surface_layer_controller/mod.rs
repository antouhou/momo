mod style;

use super::{
    compositor::use_compositor_event_inbox,
    model::{HOME_LAUNCH_CONTROLLER_REQUEST_CHANNEL_ID, LaunchControllerRequest},
    state::{HomeView, use_home_view, use_home_view_request_channel},
};
use daiko::{
    Element, Id,
    channel::Channel,
    component::{Component, ComponentContext},
    integration::{SurfaceKeyboardInteractivity, SurfaceLayer},
    state_management::StateHandle,
    window_events::{WindowEvent, WindowEventData},
};
use momo_compositor::{CompositorEvent, ShortcutId};
use style::no_view_style;

// Please note that only one Component can read from the channel. It is not enforced by the
//  compiler, but something to keep in mind
pub(super) const HOME_FOCUS_LOST_CHANNEL_ID: &str = "momo_home_focus_lost_channel";
pub(super) const HOME_HIDE_SHELL_CHANNEL_ID: &str = "momo_home_hide_shell_channel";
pub(super) const HOME_SURFACE_LAYER_STATE_ID: &str = "momo_home_surface_layer_state";

#[derive(Clone, Copy)]
pub(super) struct SurfaceLayerController {
    pub(super) launch_is_active: bool,
}

#[derive(Clone, Copy)]
enum FocusEvent {
    Gained,
    Lost,
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
enum VisibilityTransition {
    #[default]
    Stable,
    Showing,
    Hiding,
}

#[derive(Clone, Copy)]
struct OverviewVisit {
    previous_home_view: HomeView,
    previous_surface_layer: SurfaceLayer,
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
        let focus_lost_channel = ctx.use_channel_with_id::<()>(HOME_FOCUS_LOST_CHANNEL_ID);
        let hide_shell_channel = ctx.use_channel_with_id::<()>(HOME_HIDE_SHELL_CHANNEL_ID);
        let transition = ctx.use_local_state(VisibilityTransition::default);
        let overview_visit = ctx.use_local_state(|| None::<OverviewVisit>);
        let current_home_view = use_home_view(ctx);
        let home_view_request_channel = use_home_view_request_channel(ctx);
        let launch_controller_request_channel = ctx.use_channel_with_id::<LaunchControllerRequest>(
            HOME_LAUNCH_CONTROLLER_REQUEST_CHANNEL_ID,
        );
        let compositor_event_inbox = use_compositor_event_inbox(ctx);
        let requested_toggle_count = {
            let mut compositor_event_inbox = compositor_event_inbox.write_silent();
            compositor_event_inbox
                .pending_events
                .drain(..)
                .filter(|event| {
                    matches!(event, CompositorEvent::ShortcutActivated(shortcut_id) if *shortcut_id == ShortcutId::new(0))
                })
                .count()
        };
        let latest_focus_event = ctx
            .window_events()
            .iter()
            .filter_map(focus_event)
            .next_back();
        let hide_shell_requested = hide_shell_channel.iter().next().is_some();

        if overview_visit.read().is_some() && current_home_view != HomeView::Overview {
            *overview_visit.write_silent() = None;
        }

        match latest_focus_event {
            Some(FocusEvent::Gained) => {
                ctx.set_surface_layer(SurfaceLayer::Top);
                surface_layer_control.set_current_layer(SurfaceLayer::Top);
                if *transition.read() == VisibilityTransition::Showing {
                    ctx.set_surface_keyboard_interactivity(SurfaceKeyboardInteractivity::OnDemand);
                    *transition.write_silent() = VisibilityTransition::Stable;
                }
            }
            Some(FocusEvent::Lost) if *transition.read() == VisibilityTransition::Hiding => {
                ctx.set_surface_keyboard_interactivity(SurfaceKeyboardInteractivity::OnDemand);
                *transition.write_silent() = VisibilityTransition::Stable;
            }
            Some(FocusEvent::Lost) if self.launch_is_active => {
                let _ = focus_lost_channel.send(());
                ctx.set_surface_layer(SurfaceLayer::Background);
                surface_layer_control.set_current_layer(SurfaceLayer::Background);
            }
            Some(FocusEvent::Lost) | None => {}
        }

        if hide_shell_requested && !self.launch_is_active {
            *overview_visit.write_silent() = None;
            hide_shell(ctx, &surface_layer_control, &transition);
        }

        if !hide_shell_requested && requested_toggle_count % 2 == 1 {
            if self.launch_is_active {
                reverse_launch_animation_and_show_overview(
                    ctx,
                    current_home_view,
                    &launch_controller_request_channel,
                    &home_view_request_channel,
                    &surface_layer_control,
                    &transition,
                    &overview_visit,
                );
            } else {
                toggle_overview(
                    ctx,
                    current_home_view,
                    &home_view_request_channel,
                    &surface_layer_control,
                    &transition,
                    &overview_visit,
                );
            }
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
    transition: &StateHandle<VisibilityTransition>,
    overview_visit: &StateHandle<Option<OverviewVisit>>,
) {
    let _ = launch_controller_request_channel.send(LaunchControllerRequest::ReverseLaunchAnimation);
    let overview_visit_is_active = overview_visit.read().is_some();
    if !overview_visit_is_active {
        *overview_visit.write_silent() = Some(OverviewVisit {
            previous_home_view: current_home_view,
            previous_surface_layer: surface_layer_control.current_layer(),
        });
    }
    if current_home_view != HomeView::Overview {
        let _ = home_view_request_channel.send(HomeView::Overview);
    }
    show_shell(ctx, surface_layer_control, transition);
}

fn toggle_overview(
    ctx: &mut ComponentContext,
    current_home_view: HomeView,
    home_view_request_channel: &Channel<HomeView>,
    surface_layer_control: &SurfaceLayerControl,
    transition: &StateHandle<VisibilityTransition>,
    overview_visit: &StateHandle<Option<OverviewVisit>>,
) {
    let active_visit = *overview_visit.read();
    if let Some(active_visit) = active_visit {
        if current_home_view != active_visit.previous_home_view {
            let _ = home_view_request_channel.send(active_visit.previous_home_view);
        }
        if active_visit.previous_surface_layer == SurfaceLayer::Background {
            hide_shell(ctx, surface_layer_control, transition);
        }
        *overview_visit.write_silent() = None;
        return;
    }

    let current_surface_layer = surface_layer_control.current_layer();
    if current_surface_layer != SurfaceLayer::Background && current_home_view == HomeView::Overview
    {
        hide_shell(ctx, surface_layer_control, transition);
        return;
    }

    *overview_visit.write_silent() = Some(OverviewVisit {
        previous_home_view: current_home_view,
        previous_surface_layer: current_surface_layer,
    });
    if current_home_view != HomeView::Overview {
        let _ = home_view_request_channel.send(HomeView::Overview);
    }
    if current_surface_layer == SurfaceLayer::Background {
        show_shell(ctx, surface_layer_control, transition);
    }
}

fn show_shell(
    ctx: &mut ComponentContext,
    surface_layer_control: &SurfaceLayerControl,
    transition: &StateHandle<VisibilityTransition>,
) {
    ctx.set_surface_layer(SurfaceLayer::Top);
    ctx.set_surface_keyboard_interactivity(SurfaceKeyboardInteractivity::Exclusive);
    surface_layer_control.set_current_layer(SurfaceLayer::Top);
    *transition.write_silent() = VisibilityTransition::Showing;
}

fn hide_shell(
    ctx: &mut ComponentContext,
    surface_layer_control: &SurfaceLayerControl,
    transition: &StateHandle<VisibilityTransition>,
) {
    ctx.set_surface_keyboard_interactivity(SurfaceKeyboardInteractivity::None);
    ctx.set_surface_layer(SurfaceLayer::Background);
    surface_layer_control.set_current_layer(SurfaceLayer::Background);
    *transition.write_silent() = VisibilityTransition::Hiding;
}

fn focus_event(event: &WindowEvent) -> Option<FocusEvent> {
    match &event.data {
        WindowEventData::FocusGained => Some(FocusEvent::Gained),
        WindowEventData::FocusLost => Some(FocusEvent::Lost),
        _ => None,
    }
}
