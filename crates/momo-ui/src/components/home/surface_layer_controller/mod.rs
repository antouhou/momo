mod style;

use daiko::{
    Element,
    component::{Component, ComponentContext},
    integration::SurfaceLayer,
    window_events::{WindowEvent, WindowEventData},
};
use style::no_view_style;

pub(super) const HOME_FOCUS_LOST_CHANNEL_ID: &str = "momo_home_focus_lost_channel";

#[derive(Clone, Copy)]
pub(super) struct SurfaceLayerController {
    pub(super) launch_is_active: bool,
}

#[derive(Clone, Copy)]
enum FocusEvent {
    Gained,
    Lost,
}

impl Component for SurfaceLayerController {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let focus_lost_channel = ctx.use_channel_with_id::<()>(HOME_FOCUS_LOST_CHANNEL_ID);
        let latest_focus_event = ctx
            .window_events()
            .iter()
            .filter_map(focus_event)
            .next_back();

        match latest_focus_event {
            Some(FocusEvent::Gained) => ctx.set_surface_layer(SurfaceLayer::Top),
            Some(FocusEvent::Lost) if self.launch_is_active => {
                ctx.set_surface_layer(SurfaceLayer::Background);
                let _ = focus_lost_channel.send(());
            }
            Some(FocusEvent::Lost) | None => {}
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
