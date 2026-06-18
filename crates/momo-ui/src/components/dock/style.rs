use daiko::Vec2;
use daiko::layout::JustifyContent;
use daiko::style::{BorderRadius, Color, Indent, Style};

pub(super) fn dock_outer_container() -> Style {
    Style::new()
        .with_grow(1.0)
        .with_justify_content(JustifyContent::Center)
}

pub(super) fn dock_style() -> Style {
    Style::new()
        .with_spacing(Vec2::new(16.0, 0.0))
        .with_padding(Indent::uniform(16.0))
        .with_border_radius(BorderRadius {
            top_left: 16.0,
            top_right: 16.0,
            bottom_left: 0.0,
            bottom_right: 0.0,
        })
        .with_background_color(Color::from_rgba_premultiplied(0, 0, 0, 128))
}
