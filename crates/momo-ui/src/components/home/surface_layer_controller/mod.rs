mod style;

use super::compositor::use_compositor_event_inbox;
use daiko::{
    Element, Id,
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
const HOME_SURFACE_LAYER_STATE_ID: &str = "momo_home_surface_layer_state";

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
        let transition = ctx.use_local_state(VisibilityTransition::default);
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

        if requested_toggle_count % 2 == 1 && !self.launch_is_active {
            match surface_layer_control.current_layer() {
                SurfaceLayer::Background => {
                    ctx.set_surface_layer(SurfaceLayer::Top);
                    ctx.set_surface_keyboard_interactivity(SurfaceKeyboardInteractivity::Exclusive);
                    surface_layer_control.set_current_layer(SurfaceLayer::Top);
                    *transition.write_silent() = VisibilityTransition::Showing;
                }
                SurfaceLayer::Top | SurfaceLayer::Bottom | SurfaceLayer::Overlay => {
                    ctx.set_surface_keyboard_interactivity(SurfaceKeyboardInteractivity::None);
                    ctx.set_surface_layer(SurfaceLayer::Background);
                    surface_layer_control.set_current_layer(SurfaceLayer::Background);
                    *transition.write_silent() = VisibilityTransition::Hiding;
                }
            }
        }

        Element::new().with_style(no_view_style())
    }
}

fn focus_event(event: &WindowEvent) -> Option<FocusEvent> {
    match &event.data {
        WindowEventData::FocusGained => Some(FocusEvent::Gained),
        WindowEventData::FocusLost => Some(FocusEvent::Lost),
        _ => None,
    }
}
