use crate::components::home::header::{
    HEADER_BUTTON_HEIGHT, HEADER_BUTTON_RADIUS, HEADER_CLOCK_WIDTH, HeaderButtonMetrics,
    HeaderButtonState, header_button_style,
};
use crate::components::home::model::HOME_CLOCK_STATE_ID;
use crate::components::home::time::read_system_time;
use daiko::component::{Component, ComponentContext};
use daiko::navigation::FocusOrigin;
use daiko::style::{Color, Style};
use daiko::widgets::text::{Text, TextStyle, TextWrap};
use daiko::{Element, Id};

#[derive(Clone, Copy)]
pub(super) struct ClockChip;

impl Component for ClockChip {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let mut pointer = ctx.pointer();
        let focusable = ctx.focusable();
        let clock_text = ctx.use_global_state(Id::new(HOME_CLOCK_STATE_ID), read_system_time);

        if pointer.just_pressed() {
            focusable.request_focus(FocusOrigin::Pointer);
        }

        let state = HeaderButtonState {
            is_active: false,
            is_pressed: pointer.is_pressed(),
            is_hovered: pointer.is_hovering(),
            is_focused: focusable.is_focused(),
        };
        let text_color = if state.is_pressed || state.is_hovered || state.is_focused {
            Color::from_rgb(10, 13, 18)
        } else {
            Color::from_rgb(232, 238, 250)
        };

        Element::new()
            .with_tag("clock-chip")
            .with_style(clock_button_style(ctx, state))
            .with_content(
                Text::new(clock_text.read().clone()).with_style(
                    TextStyle::default()
                        .with_font_size(22.0)
                        .with_line_height(1.0)
                        .with_font_color(text_color)
                        .with_wrap(TextWrap::NoWrap),
                ),
            )
    }
}

fn clock_button_style(ctx: &mut ComponentContext, state: HeaderButtonState) -> Style {
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
