use daiko::component::{Component, ComponentContext};
use daiko::Element;
use daiko::style::{Background, Color, Style};
use daiko::widgets::text::Text;

pub struct Home;

pub const HOME_STYLE : &'static Style = &Style {
    background: Background::SolidColor(Color::BLACK),
    ..Style::DEFAULT
};

impl Component for Home {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        Element::new().with_content(Text::new("Hello World!")).with_style(HOME_STYLE)
    }
}