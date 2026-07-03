mod style;

use crate::components::login_screen::power_button::style::{power_button_style, power_text_style};
use daiko::Element;
use daiko::component::{Component, ComponentContext};
use daiko::widgets::text::Text;
use momo_kit::interaction::ButtonBehavior;

#[derive(Clone, Copy)]
pub(super) struct PowerButton;

impl Component for PowerButton {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let button = ButtonBehavior::new(ctx).apply();

        if button.just_activated {
            println!("Pressed power button");
        }

        let is_highlighted = button.is_hovering || button.is_focus_visible;

        Element::new()
            .with_tag("power-button")
            .with_style(power_button_style(ctx, is_highlighted))
            .with_content(Text::new("⏻").with_style(power_text_style(is_highlighted)))
    }
}
