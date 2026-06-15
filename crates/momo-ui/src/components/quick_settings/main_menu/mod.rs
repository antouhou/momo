mod style;
mod tile_grid;
mod tile_specs;
mod top_row;

use self::tile_grid::SettingsTileGrid;
use self::top_row::SettingsTopRow;
use super::common::{settings_middle_row, settings_row};
use super::volume_control::VolumeControl;
use crate::components::quick_settings::style::{
    SETTINGS_MENU_GAP, SETTINGS_TILE_HEIGHT, settings_content_container_style,
};
use daiko::widgets::scrollable::Scrollable;
use daiko::{Element, component::Component, component::ComponentContext};

#[derive(Clone, Copy)]
pub(super) struct MainMenu {
    pub(super) show_scroll_bars_when_overflowing: bool,
}

impl Component for MainMenu {
    fn to_element(&self, _ctx: &mut ComponentContext) -> Element {
        Element::new()
            .with_style(settings_content_container_style())
            .with_content(settings_row(SettingsTopRow))
            .with_content(settings_middle_row(VolumeControl))
            .with_content(
                Scrollable::new(SettingsTileGrid, "quick_settings_scrollable")
                    .with_visible_scroll_bars(self.show_scroll_bars_when_overflowing)
                    .with_focus_reveal_band(0.0, SETTINGS_TILE_HEIGHT + SETTINGS_MENU_GAP * 2.0),
            )
    }
}
