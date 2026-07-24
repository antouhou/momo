use crate::components::home::{
    app_tile::{AppButtonSurfaceMetrics, app_button_surface_style},
    model::TILE_BORDER_WIDTH,
};
use daiko::{
    component::ComponentContext,
    layout::{AlignItems, FlexDirection, ItemSize, JustifyContent},
    style::{Color, CursorIcon, Style, Transform},
};

pub(super) const OVERVIEW_APPS_BUTTON_SIZE: f32 = 72.0;
pub(super) const OVERVIEW_APPS_BUTTON_ICON_SIZE: usize = 40;
const OVERVIEW_APPS_BUTTON_RADIUS: f32 = 16.0;

pub(super) fn overview_apps_button_band_style() -> Style {
    Style::new()
        .with_fixed_width(ItemSize::Percent(1.0))
        .with_fixed_height(ItemSize::Points(OVERVIEW_APPS_BUTTON_SIZE))
        .with_direction(FlexDirection::Row)
        .with_align_items(AlignItems::Center)
        .with_justify_content(JustifyContent::Center)
}

pub(super) fn overview_apps_button_style(
    ctx: &mut ComponentContext,
    transform: &Transform,
    is_hovering: bool,
    is_focus_visible: bool,
) -> Style {
    let mut style = app_button_surface_style(
        ctx,
        AppButtonSurfaceMetrics {
            width: OVERVIEW_APPS_BUTTON_SIZE,
            height: OVERVIEW_APPS_BUTTON_SIZE,
            border_radius: OVERVIEW_APPS_BUTTON_RADIUS,
            border_width: TILE_BORDER_WIDTH,
        },
        Color::from_rgb(88, 112, 138),
        transform,
        is_hovering,
        is_focus_visible,
    )
    .with_centered_content();

    if is_hovering {
        style.set_cursor(CursorIcon::PointingHand);
    }

    style
}

pub(super) fn overview_apps_button_icon_color() -> Color {
    Color::from_rgb(210, 220, 230)
}
