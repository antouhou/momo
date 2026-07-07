mod style;

use daiko::{
    Element,
    component::{Child, Component, ComponentContext, IntoChild},
    navigation::{FocusEntryPolicy, TraversalPolicy},
};
pub(super) use self::style::{
    HEADER_BUTTON_HEIGHT, HEADER_BUTTON_RADIUS, HEADER_CLOCK_WIDTH, HeaderButtonMetrics,
    HeaderButtonState, header_button_style,
};
use self::style::{central_container_style, header_row_style, header_side_style, header_style};
use crate::components::home::clock_chip::ClockChip;

pub(super) struct HomeHeader {
    center: Child,
}

impl HomeHeader {
    pub fn new(center: impl IntoChild) -> Self {
        Self {
            center: center.into_child(),
        }
    }
}

impl Component for HomeHeader {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let focus_scope = ctx.focus_scope();
        focus_scope.set_entry_policy(FocusEntryPolicy::Spatial(
            TraversalPolicy::RectilinearDistance,
        ));

        Element::new()
            .with_tag("apps-header")
            .with_style(header_style())
            .with_content(
                Element::new()
                    .with_tag("apps-header-row")
                    .with_style(header_row_style())
                    .with_content(Element::new().with_style(header_side_style(false)))
                    .with_content(
                        Element::new()
                            .with_content(self.center.clone())
                            .with_style(central_container_style()),
                    )
                    .with_content(
                        Element::new()
                            .with_style(header_side_style(true))
                            .with_content(ClockChip),
                    ),
            )
    }
}
