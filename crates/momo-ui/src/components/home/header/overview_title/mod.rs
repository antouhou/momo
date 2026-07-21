mod style;

use self::style::{overview_header_title_frame_style, overview_header_title_text_style};
use daiko::{
    Element,
    component::{Component, ComponentContext},
    widgets::text::Text,
};

const OVERVIEW_HEADER_TITLE: &str = "Overview";

#[derive(Clone, Copy)]
pub(in crate::components::home) struct OverviewHeaderTitle;

impl Component for OverviewHeaderTitle {
    fn to_element(&self, _context: &mut ComponentContext) -> Element {
        Element::new()
            .with_tag("overview-header-title")
            .with_style(overview_header_title_frame_style())
            .with_content(
                Text::new(OVERVIEW_HEADER_TITLE).with_style(overview_header_title_text_style()),
            )
    }
}
