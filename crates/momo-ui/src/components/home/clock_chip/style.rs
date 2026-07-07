use daiko::{
    component::ComponentContext,
    style::{Color, Style},
    widgets::text::{TextStyle, TextWrap},
};

use crate::components::home::header::{
    HEADER_BUTTON_HEIGHT, HEADER_BUTTON_RADIUS, HEADER_CLOCK_WIDTH, HeaderButtonMetrics,
    HeaderButtonState, header_button_style,
};

const CLOCK_TEXT_SIZE: f32 = 22.0;

pub(super) fn clock_button_style(ctx: &mut ComponentContext, state: HeaderButtonState) -> Style {
    header_button_style(
        ctx,
        HeaderButtonMetrics {
            width: HEADER_CLOCK_WIDTH,
            height: HEADER_BUTTON_HEIGHT,
            radius: HEADER_BUTTON_RADIUS,
        },
        state,
        true,
    )
}

pub(super) fn clock_text_style(state: HeaderButtonState) -> TextStyle {
    let text_color = if state.is_pressed || state.is_hovered || state.is_focused {
        Color::from_rgb(10, 13, 18)
    } else {
        Color::from_rgb(232, 238, 250)
    };

    TextStyle::default()
        .with_font_size(CLOCK_TEXT_SIZE)
        .with_line_height(1.0)
        .with_font_color(text_color)
        .with_wrap(TextWrap::NoWrap)
}
