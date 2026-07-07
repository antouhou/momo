use daiko::{
    component::ComponentContext,
    style::{Color, CursorIcon, Style, Transform},
};
use crate::components::home::{
    app_tile::{AppButtonSurfaceMetrics, app_button_surface_style},
    model::TILE_BORDER_WIDTH,
};

pub(super) const DOCK_BUTTON_SIZE: f32 = 72.0;
const DOCK_BUTTON_RADIUS: f32 = 16.0;
pub(super) const DOCK_ICON_SIZE: f32 = 52.0;
pub(super) const DOCK_ICON_GLYPH_SIZE: usize = 52;

pub(super) fn hidden_dock_button_style() -> Style {
    Style::new()
        .with_fixed_size(DOCK_BUTTON_SIZE, DOCK_BUTTON_SIZE)
        .with_background_color(Color::TRANSPARENT)
}

pub(super) fn dock_button_style(
    ctx: &mut ComponentContext,
    accent: Color,
    transform: &Transform,
    is_hovering: bool,
    is_focus_visible: bool,
) -> Style {
    let mut style = app_button_surface_style(
        ctx,
        AppButtonSurfaceMetrics {
            width: DOCK_BUTTON_SIZE,
            height: DOCK_BUTTON_SIZE,
            border_radius: DOCK_BUTTON_RADIUS,
            border_width: TILE_BORDER_WIDTH,
        },
        accent,
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
