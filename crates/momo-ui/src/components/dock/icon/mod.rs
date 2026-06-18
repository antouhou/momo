mod style;

use daiko::Element;
use daiko::component::{Component, ComponentContext};
use style::dock_icon_container;

pub struct DockIcon {}

impl Component for DockIcon {
    fn to_element(&self, _ctx: &mut ComponentContext) -> Element {
        Element::new().with_style(dock_icon_container())
    }
}
