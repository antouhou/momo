use super::OverviewCardFrame;
use daiko::{
    Vec2,
    layout::{AlignItems, FlexDirection, ItemSize, JustifyContent},
    style::{Color, Style},
    widgets::text::{TextStyle, TextWrap, Weight},
};
use momo_kit::{
    components::{ROUND_ICON_BUTTON_BORDER_WIDTH, ROUND_ICON_BUTTON_SIZE},
    style::SYSTEM_TEXT_SIZE,
};

const OVERVIEW_WINDOW_CONTROLS_GAP: f32 = 18.0;
const OVERVIEW_WINDOW_CONTROLS_SPACING: f32 = 12.0;

pub(super) fn window_controls_target_position(active_card_frame: OverviewCardFrame) -> Vec2 {
    let controls_height = ROUND_ICON_BUTTON_SIZE + ROUND_ICON_BUTTON_BORDER_WIDTH * 2.0;
    Vec2::new(
        active_card_frame.position.x,
        active_card_frame.position.y - controls_height - OVERVIEW_WINDOW_CONTROLS_GAP,
    )
}

pub(super) fn window_controls_style(rendered_position: Vec2, width: f32) -> Style {
    Style::new()
        .with_absolute_position(rendered_position)
        .with_fixed_width(ItemSize::Points(width))
        .with_fixed_height(ItemSize::Points(ROUND_ICON_BUTTON_SIZE))
        .with_direction(FlexDirection::Row)
        .with_align_items(AlignItems::Center)
        .with_justify_content(JustifyContent::Center)
        .with_spacing((OVERVIEW_WINDOW_CONTROLS_SPACING, 0.0))
}

pub(super) fn window_title_style() -> TextStyle {
    TextStyle::default()
        .with_font_size(SYSTEM_TEXT_SIZE)
        .with_weight(Weight::MEDIUM)
        .with_font_color(Color::from_rgb(236, 246, 255))
        .with_wrap(TextWrap::NoWrap)
}
