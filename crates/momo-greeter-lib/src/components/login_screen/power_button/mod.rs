mod style;

use crate::components::login_screen::power_button::style::{
    ICON_SIZE, power_button_style, power_icon_color,
};
use daiko::Element;
use daiko::component::{Component, ComponentContext};
use daiko::widgets::image::{Image, ImageParams, ImageSource, ImageType};
use momo_kit::assets::POWER_ICON;
use momo_kit::interaction::ButtonBehavior;

#[derive(Clone, Copy)]
pub(super) struct PowerButton;

impl Component for PowerButton {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let button = ButtonBehavior::new(ctx).apply();

        if button.just_activated {
            tracing::debug!("pressed power button");
        }

        let is_highlighted = button.is_hovering || button.is_focus_visible;

        Element::new()
            .with_tag("power-button")
            .with_style(power_button_style(ctx, is_highlighted))
            .with_content(
                Image::new(ImageParams {
                    max_width: ICON_SIZE,
                    max_height: ICON_SIZE,
                    image_type: Some(ImageType::Svg),
                    source: ImageSource::BytesSlice(POWER_ICON),
                })
                .fill_color(Some(power_icon_color(is_highlighted))),
            )
    }
}
