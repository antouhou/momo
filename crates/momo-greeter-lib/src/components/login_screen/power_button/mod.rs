mod style;

use crate::components::login_screen::power_button::style::{power_button_style, power_text_style};
use daiko::Element;
use daiko::component::{Component, ComponentContext};
use daiko::navigation::FocusOrigin;
use daiko::widgets::text::Text;

#[derive(Clone, Copy)]
pub(super) struct PowerButton;

impl Component for PowerButton {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let mut pointer = ctx.pointer();
        let focusable = ctx.focusable();

        if pointer.just_pressed() {
            focusable.request_focus(FocusOrigin::Pointer);
        }

        if pointer.just_pressed() || focusable.just_activated() {
            println!("Pressed power button");
        }

        let is_highlighted = pointer.is_hovering() || focusable.is_focus_visible();

        Element::new()
            .with_tag("power-button")
            .with_style(power_button_style(ctx, is_highlighted))
            .with_content(Text::new("⏻").with_style(power_text_style(is_highlighted)))
    }
}
