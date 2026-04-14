use crate::components::home::app_tile::AppTile;
use crate::components::home::model::{
    GRID_GAP, HOME_SCROLLABLE_ID, LaunchRequest, MOCK_APPS, TILE_HEIGHT, TILE_WIDTH,
    columns_for_width,
};
use daiko::Element;
use daiko::channel::Channel;
use daiko::component::{Component, ComponentContext};
use daiko::layout::{AlignItems, JustifyContent};
use daiko::style::Style;
use daiko::widgets::container::{Container, Fit};
use daiko::widgets::scrollable::Scrollable;

#[derive(Clone)]
pub(super) struct AppGrid {
    pub launch_channel: Channel<LaunchRequest>,
    pub interactions_disabled: bool,
    pub hidden_app_id: Option<&'static str>,
    pub preferred_focus_app_id: Option<&'static str>,
}

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
                    preferred_focus: self.preferred_focus_app_id == Some(app.id)
                        || (self.preferred_focus_app_id.is_none() && row_index == 0 && index == 0),
                    launch_channel: self.launch_channel.clone(),
                    interactions_disabled: self.interactions_disabled,
                    is_hidden_for_launch: self.hidden_app_id == Some(app.id),
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
