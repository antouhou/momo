use daiko::{
    layout::ItemSize,
    style::{Color, Style},
    widgets::text::{TextStyle, TextWrap},
};

const CLOCK_WIDTH: f32 = 104.0;
const CLOCK_HEIGHT: f32 = 38.0;
const CLOCK_TEXT_SIZE: f32 = 22.0;

pub(super) fn clock_style() -> Style {
    Style::new()
        .with_fixed_width(ItemSize::Points(CLOCK_WIDTH))
        .with_fixed_height(ItemSize::Points(CLOCK_HEIGHT))
        .with_centered_content()
}

pub(super) fn clock_text_style() -> TextStyle {
    TextStyle::default()
        .with_font_size(CLOCK_TEXT_SIZE)
        .with_line_height(1.0)
        .with_font_color(Color::from_rgb(232, 238, 250))
        .with_wrap(TextWrap::NoWrap)
}
