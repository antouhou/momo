use crate::components::home::clock_chip::ClockChip;
use crate::components::home::model::SCREEN_PADDING;
use daiko::Element;
use daiko::component::{Component, ComponentContext};
use daiko::layout::{AlignItems, FlexDirection, JustifyContent, SizeConstraint};
use daiko::style::{Indent, Style};
use daiko::widgets::container::{Container, Fit};
use daiko::widgets::heading::{Heading, HeadingLevel};
use daiko::widgets::text::VerticalTextAlignment;

#[derive(Clone, Copy)]
pub(super) struct HomeHeader;

impl Component for HomeHeader {
    fn to_element(&self, _ctx: &mut ComponentContext) -> Element {
        Element::new()
            .with_tag("apps-header")
            .with_style(header_style())
            .with_content(header_row())
    }
}

fn header_row() -> Element {
    Element::new()
        .with_tag("apps-header-row")
        .with_style(header_row_style())
        .with_content(
            Container::horizontal()
                .with_fit(Fit::new().exact_content_size())
                .align_items_center()
                .build()
                .with_tag("apps-header-title")
                .with_content(
                    Heading::new("Apps", HeadingLevel::H1)
                        .with_vertical_text_alignment(VerticalTextAlignment::Center),
                ),
        )
        .with_content(ClockChip)
}

fn header_style() -> Style {
    Style::new()
        .with_direction(FlexDirection::Column)
        .with_size_constraint(SizeConstraint::exact_content_height())
        .with_padding(Indent::new(
            SCREEN_PADDING,
            SCREEN_PADDING,
            SCREEN_PADDING,
            0.0,
        ))
}

fn header_row_style() -> Style {
    Style::new()
        .with_direction(FlexDirection::Row)
        .with_justify_content(JustifyContent::SpaceBetween)
        .with_align_items(AlignItems::Center)
}
