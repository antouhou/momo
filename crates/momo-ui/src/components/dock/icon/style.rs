use daiko::style::{BorderRadius, Color, Style};

pub(super) fn dock_icon_container() -> Style {
    Style::new()
        .with_fixed_size(64.0, 64.0)
        .with_border_radius(BorderRadius::all(16.0))
        .with_background_color(Color::from_rgba_premultiplied(100, 100, 100, 178))
}
