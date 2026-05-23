mod style;

use self::style::{settings_status_chip_style, status_chip_content_style, status_value_style};
use super::common::{QuickSettingsGlyph, control_state, glyph_element, is_menu_view_active};
use super::state::SettingsMenuView;
use super::style::{SETTINGS_ICON_FRAME_SIZE, SETTINGS_ICON_SIZE, settings_text_color};
use daiko::Element;
use daiko::component::{Component, ComponentContext};
use daiko::widgets::text::Text;

const BATTERY_ICON: &[u8] = include_bytes!("../../../../assets/battery-5.svg");
pub(super) const SETTINGS_STATUS_CHIP_TAG: &str = "header-settings-status-chip";

#[derive(Clone, Copy)]
pub(super) struct StatusChip;

impl Component for StatusChip {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let focusable = ctx.focusable();
        let is_active = is_menu_view_active(ctx, SettingsMenuView::Main);
        focusable.set_preferred_focus(is_active);
        focusable.set_navigation_enabled(is_active);
        let state = control_state(ctx);

        Element::new()
            .with_tag(SETTINGS_STATUS_CHIP_TAG)
            .with_style(settings_status_chip_style(state, ctx))
            .with_content(status_chip_content())
    }
}

fn status_chip_content() -> Element {
    Element::new()
        .with_style(status_chip_content_style())
        .with_content(glyph_element(
            QuickSettingsGlyph::Asset(BATTERY_ICON),
            SETTINGS_ICON_SIZE,
            SETTINGS_ICON_FRAME_SIZE,
            settings_text_color(),
        ))
        .with_content(Text::new("96%").with_style(status_value_style()))
}
