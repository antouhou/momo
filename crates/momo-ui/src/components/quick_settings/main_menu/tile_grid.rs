use super::super::settings_tile_button::{SettingsTileButton, TILE_ROWS};
use super::style::{settings_tile_grid_style, settings_tile_row_style};
use daiko::Element;
use daiko::component::{Component, ComponentContext};
use daiko::navigation::FocusEntryPolicy;

#[derive(Clone, Copy)]
pub(super) struct SettingsTileGrid;

impl Component for SettingsTileGrid {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        ctx.focus_scope()
            .set_entry_policy(FocusEntryPolicy::Remembered);

        let mut grid = Element::new().with_style(settings_tile_grid_style());

        for row in TILE_ROWS {
            let mut row_element = Element::new().with_style(settings_tile_row_style());
            for tile in row {
                row_element.add_content(SettingsTileButton { spec: tile });
            }
            grid.add_content(row_element);
        }

        grid
    }
}
