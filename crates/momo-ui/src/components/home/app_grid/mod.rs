use crate::components::home::app_tile::AppTile;
use crate::components::home::model::{
    GRID_GAP, HOME_SCROLLABLE_ID, MOCK_APPS, TILE_HEIGHT, TILE_WIDTH, columns_for_width,
};
use daiko::Element;
use daiko::component::{Component, ComponentContext};
use daiko::layout::{AlignItems, JustifyContent};
use daiko::style::Style;
use daiko::widgets::container::{Container, Fit};
use daiko::widgets::scrollable::Scrollable;

#[derive(Clone, Copy)]
pub(super) struct AppGrid;

impl Component for AppGrid {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let available_width = ctx
            .layout()
            .map(|layout| layout.size.x)
            .unwrap_or_else(|| 0.0);
        let columns = columns_for_width(available_width.max(TILE_WIDTH));
        let row_width = columns as f32 * TILE_WIDTH + (columns as f32 - 1.0) * GRID_GAP;

        let mut content = Container::vertical()
            .with_fit(Fit::new().exact_content_height())
            .align_items_center()
            .with_spacing((GRID_GAP, GRID_GAP))
            .build();

        for (row_index, row) in MOCK_APPS.chunks(columns).enumerate() {
            let mut row_container = Container::horizontal()
                .with_fit(Fit::new().exact_width(row_width).exact_height(TILE_HEIGHT))
                .align_items_start()
                .with_spacing((GRID_GAP, GRID_GAP))
                .build();

            for (index, app) in row.iter().enumerate() {
                row_container.add_content(AppTile {
                    app: *app,
                    preferred_focus: row_index == 0 && index == 0,
                });
            }

            content.add_content(row_container);
        }

        Element::new()
            .with_tag("apps-grid")
            .with_style(
                Style::new()
                    .with_direction(daiko::layout::FlexDirection::Column)
                    .with_justify_content(JustifyContent::Center)
                    .with_align_items(AlignItems::Center),
            )
            .with_content(Scrollable::new(content, HOME_SCROLLABLE_ID))
    }
}
