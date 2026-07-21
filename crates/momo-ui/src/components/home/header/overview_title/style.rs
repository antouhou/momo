use crate::components::home::header::HEADER_BUTTON_HEIGHT;
use daiko::{
    layout::{AlignItems, FlexDirection, ItemSize, JustifyContent},
    style::{Color, Style},
    widgets::text::{TextStyle, TextWrap},
};
use momo_kit::style::SYSTEM_TRAY_TEXT_SIZE;

pub(super) fn overview_header_title_frame_style() -> Style {
    Style::new()
        .with_fixed_height(ItemSize::Points(HEADER_BUTTON_HEIGHT))
        .with_direction(FlexDirection::Row)
        .with_align_items(AlignItems::Center)
        .with_justify_content(JustifyContent::Center)
}

pub(super) fn overview_header_title_text_style() -> TextStyle {
    TextStyle::default()
        .with_font_size(SYSTEM_TRAY_TEXT_SIZE)
        .with_line_height(1.0)
        .with_font_color(Color::from_rgb(236, 246, 255))
        .with_wrap(TextWrap::NoWrap)
}
