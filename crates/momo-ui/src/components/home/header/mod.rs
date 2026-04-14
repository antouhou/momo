use crate::components::home::clock_chip::clock_chip;
use daiko::Element;
use daiko::layout::JustifyContent;
use daiko::style::Style;
use daiko::widgets::container::{Container, Fit};
use daiko::widgets::heading::{Heading, HeadingLevel};
use daiko::widgets::text::VerticalTextAlignment;

pub(super) fn build_home_header(clock_text: String) -> Element {
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
        .with_content(clock_chip(clock_text))
}
