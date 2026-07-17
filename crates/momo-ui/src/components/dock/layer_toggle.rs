use super::style::{layer_toggle_button_style, layer_toggle_label_style};
use daiko::{Element, component::{Component, ComponentContext}, integration::SurfaceLayer, widgets::text::Text, Id};
use momo_kit::interaction::ButtonBehavior;

pub(super) struct LayerToggleButton {
    pub(super) interactions_disabled: bool,
}

// Temporary button
impl Component for LayerToggleButton {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let requested_layer = ctx.use_local_state(|| SurfaceLayer::Top);
        let button = ButtonBehavior::new(ctx)
            .with_enabled(!self.interactions_disabled)
            .apply();
        let mut current_layer = *requested_layer.read();

        if button.just_activated {
            let next_layer = match current_layer {
                SurfaceLayer::Background => SurfaceLayer::Top,
                SurfaceLayer::Top => SurfaceLayer::Background,
                SurfaceLayer::Bottom | SurfaceLayer::Overlay => SurfaceLayer::Background,
            };
            ctx.set_surface_layer(next_layer);
            *requested_layer.write() = next_layer;
            current_layer = next_layer;
        }

        let is_top = current_layer == SurfaceLayer::Top;
        let label = if is_top { "TOP" } else { "BG" };
        Element::new()
            .with_tag("dock-layer-toggle")
            .with_style(layer_toggle_button_style(
                ctx,
                is_top,
                button.is_hovering,
                button.is_focus_visible,
            ))
            .with_content(Text::new(label).with_style(layer_toggle_label_style()))
    }
}
