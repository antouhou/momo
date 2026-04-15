use crate::components::home::model::HOME_CLOCK_STATE_ID;
use crate::components::home::time::read_system_time;
use daiko::component::{Component, ComponentContext};
use daiko::style::{Color, Indent, Style};
use daiko::widgets::text::{Text, TextStyle, TextWrap};
use daiko::{Element, Id};

#[derive(Clone, Copy)]
pub(super) struct ClockChip;

impl Component for ClockChip {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let clock_text = ctx.use_global_state(Id::new(HOME_CLOCK_STATE_ID), read_system_time);

        Element::new()
            .with_tag("clock-chip")
            .with_style(
                Style::new()
                    .with_padding(Indent::from((18.0, 12.0)))
                    .with_size_constraint(daiko::layout::SizeConstraint::exact_content_size()),
            )
            .with_content(
                Text::new(clock_text.read().clone()).with_style(
                    TextStyle::default()
                        .with_font_size(24.0)
                        .with_font_color(Color::from_rgb(232, 238, 250))
                        .with_wrap(TextWrap::NoWrap),
                ),
            )
    }
}
