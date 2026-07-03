mod style;

use daiko::component::{Component, ComponentContext};
use daiko::{Element};
use daiko::effects::BackdropBlur;
use daiko::style::Style;
use crate::components::login_screen::style::login_screen_main_style;

#[derive(Clone, Copy)]
pub struct LoginScreen {}

impl LoginScreen {
    pub fn new() -> Self {
        Self { }
    }
}

impl Component for LoginScreen {
    fn to_element(&self, _ctx: &mut ComponentContext) -> Element {
        let root = Element::new()
            .with_tag("login_screen-root")
            .with_style(login_screen_main_style());

        root
    }
}
