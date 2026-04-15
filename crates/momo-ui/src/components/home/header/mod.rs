use crate::components::home::clock_chip::ClockChip;
use daiko::Element;
use daiko::component::{Component, ComponentContext};
use daiko::layout::JustifyContent;
use daiko::style::Style;
use daiko::widgets::container::{Container, Fit};
use daiko::widgets::heading::{Heading, HeadingLevel};
use daiko::widgets::text::VerticalTextAlignment;

#[derive(Clone, Copy)]
pub(super) struct HomeHeader;

impl Component for HomeHeader {
    fn to_element(&self, _ctx: &mut ComponentContext) -> Element {
        Container::horizontal()
            .with_style(Style::new().with_justify_content(JustifyContent::SpaceBetween))
            .with_fit(Fit::new().exact_content_height())
            .align_items_center()
            .build()
            .with_tag("apps-header")
            .with_content(
                Heading::new("Apps", HeadingLevel::H1)
                    .with_vertical_text_alignment(VerticalTextAlignment::Center),
            )
            .with_content(ClockChip)
    }
}
