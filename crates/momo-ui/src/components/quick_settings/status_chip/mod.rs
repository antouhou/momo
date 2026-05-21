mod style;

use self::style::{settings_status_chip_style, status_chip_content_style, status_value_style};
use super::common::{QuickSettingsGlyph, control_state, glyph_element};
use daiko::Element;
use daiko::component::{Component, ComponentContext};
use daiko::style::Color;
use daiko::widgets::text::Text;

const BATTERY_ICON: &[u8] = include_bytes!("../../../../assets/battery-5.svg");

#[derive(Clone, Copy)]
pub(super) struct StatusChip;

impl Component for StatusChip {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let state = control_state(ctx);

        Element::new()
            .with_style(settings_status_chip_style(state, ctx))
            .with_content(status_chip_content())
    }
}

fn status_chip_content() -> Element {
    Element::new()
        .with_style(status_chip_content_style())
        .with_content(glyph_element(
            QuickSettingsGlyph::Asset(BATTERY_ICON),
            14,
            16.0,
            Color::from_rgb(12, 16, 20),
        ))
        .with_content(Text::new("96%").with_style(status_value_style()))
}
