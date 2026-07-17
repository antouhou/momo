use super::style::{layer_toggle_button_style, layer_toggle_label_style};
use crate::components::home::surface_layer_controller::use_surface_layer_control;
use daiko::{
    Element,
    component::{Component, ComponentContext},
    integration::SurfaceLayer,
    widgets::text::Text,
};
use momo_kit::interaction::ButtonBehavior;

pub(super) struct LayerToggleButton {
    pub(super) interactions_disabled: bool,
}

// Temporary button
impl Component for LayerToggleButton {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let surface_layer_control = use_surface_layer_control(ctx);
        let button = ButtonBehavior::new(ctx)
            .with_enabled(!self.interactions_disabled)
            .apply();

        if button.just_activated {
            surface_layer_control.request_toggle();
        }

        let is_top = surface_layer_control.current_layer() == SurfaceLayer::Top;
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
