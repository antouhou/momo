mod style;

use daiko::{
    Element, Id,
    channel::Channel,
    component::{Component, ComponentContext},
    integration::SurfaceLayer,
    state_management::StateHandle,
    window_events::{WindowEvent, WindowEventData},
};
use style::no_view_style;

pub(super) const HOME_FOCUS_LOST_CHANNEL_ID: &str = "momo_home_focus_lost_channel";
const HOME_SURFACE_LAYER_REQUEST_CHANNEL_ID: &str = "momo_home_surface_layer_request_channel";
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

#[derive(Clone, Copy)]
enum SurfaceLayerRequest {
    Toggle,
}

pub(crate) struct SurfaceLayerControl {
    current_layer: StateHandle<SurfaceLayer>,
    requests: Channel<SurfaceLayerRequest>,
}

impl SurfaceLayerControl {
    pub(crate) fn current_layer(&self) -> SurfaceLayer {
        *self.current_layer.read()
    }

    pub(crate) fn request_toggle(&self) {
        let _ = self.requests.send(SurfaceLayerRequest::Toggle);
    }

    fn requested_layer(&self) -> Option<SurfaceLayer> {
        self.requests
            .iter()
            .fold(None, |requested_layer, request| match request {
                SurfaceLayerRequest::Toggle => Some(toggle_layer(
                    requested_layer.unwrap_or_else(|| self.current_layer()),
                )),
            })
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
        requests: ctx.use_channel_with_id(HOME_SURFACE_LAYER_REQUEST_CHANNEL_ID),
    }
}

impl Component for SurfaceLayerController {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let surface_layer_control = use_surface_layer_control(ctx);
        let focus_lost_channel = ctx.use_channel_with_id::<()>(HOME_FOCUS_LOST_CHANNEL_ID);
        let requested_layer = surface_layer_control.requested_layer();
        let latest_focus_event = ctx
            .window_events()
            .iter()
            .filter_map(focus_event)
            .next_back();

        let focus_layer = match latest_focus_event {
            Some(FocusEvent::Gained) => Some(SurfaceLayer::Top),
            Some(FocusEvent::Lost) if self.launch_is_active => {
                let _ = focus_lost_channel.send(());
                Some(SurfaceLayer::Background)
            }
            Some(FocusEvent::Lost) | None => None,
        };

        if let Some(layer) = focus_layer.or(requested_layer) {
            ctx.set_surface_layer(layer);
            surface_layer_control.set_current_layer(layer);
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

fn toggle_layer(current_layer: SurfaceLayer) -> SurfaceLayer {
    match current_layer {
        SurfaceLayer::Background => SurfaceLayer::Top,
        SurfaceLayer::Top => SurfaceLayer::Background,
        SurfaceLayer::Bottom | SurfaceLayer::Overlay => SurfaceLayer::Background,
    }
}
