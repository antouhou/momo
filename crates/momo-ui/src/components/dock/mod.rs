mod icon;
mod style;

use daiko::Element;
use daiko::component::{Component, ComponentContext};

pub struct Dock {}

impl Component for Dock {
    fn to_element(&self, _ctx: &mut ComponentContext) -> Element {
        Element::new()
            .with_style(style::dock_outer_container())
            .with_content(
                Element::new()
                    .with_style(style::dock_style())
                    .with_content(icon::DockIcon {})
                    .with_content(icon::DockIcon {})
                    .with_content(icon::DockIcon {}),
            )
    }
}
