use crate::components::dock::icon::DOCK_ICON_SIZE;
use daiko::{
    layout::{AlignItems, FlexDirection, JustifyContent},
    style::{Color, Style},
};

const OVERVIEW_GLYPH_SIZE: f32 = 40.0;

pub(super) fn overview_glyph_frame_style() -> Style {
    Style::new()
        .with_fixed_size(DOCK_ICON_SIZE, DOCK_ICON_SIZE)
        .with_direction(FlexDirection::Row)
        .with_align_items(AlignItems::Center)
        .with_justify_content(JustifyContent::Center)
}

pub(super) fn overview_glyph_size() -> usize {
    OVERVIEW_GLYPH_SIZE as usize
}

pub(super) fn overview_glyph_color(is_active: bool) -> Color {
    if is_active {
        Color::from_rgb(246, 249, 252)
    } else {
        Color::from_rgb(210, 220, 230)
    }
}

pub(super) fn overview_accent_color(is_active: bool) -> Color {
    if is_active {
        Color::from_rgb(126, 176, 224)
    } else {
        Color::from_rgb(88, 112, 138)
    }
}
