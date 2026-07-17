use daiko::{
    Vec2,
    component::ComponentContext,
    layout::{AlignItems, FlexDirection, JustifyContent},
    style::{BorderRadius, Color, CursorIcon, Indent, Style},
    widgets::text::{TextStyle, TextWrap},
};

const LAYER_TOGGLE_SIZE: f32 = 48.0;

pub(super) fn dock_outer_container() -> Style {
    Style::new()
        .with_grow(1.0)
        .with_justify_content(JustifyContent::Center)
}

pub(super) fn dock_style() -> Style {
    Style::new()
        .with_direction(FlexDirection::Row)
        .with_align_items(AlignItems::Center)
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

pub(super) fn layer_toggle_button_style(
    _ctx: &mut ComponentContext,
    is_top: bool,
    is_hovering: bool,
    is_focus_visible: bool,
) -> Style {
    let is_highlighted = is_hovering || is_focus_visible;
    let background = match (is_top, is_highlighted) {
        (true, true) => Color::from_rgb(216, 126, 74),
        (true, false) => Color::from_rgb(186, 101, 54),
        (false, true) => Color::from_rgb(76, 108, 150),
        (false, false) => Color::from_rgb(52, 78, 112),
    };

    Style::new()
        .with_fixed_size(LAYER_TOGGLE_SIZE, LAYER_TOGGLE_SIZE)
        .with_centered_content()
        .with_border_radius(BorderRadius::all(12.0))
        .with_background_color(background)
        .with_cursor(CursorIcon::PointingHand)
}

pub(super) fn layer_toggle_label_style() -> TextStyle {
    TextStyle::default()
        .with_font_size(14.0)
        .with_line_height(1.0)
        .with_font_color(Color::from_rgb(245, 247, 252))
        .with_wrap(TextWrap::NoWrap)
}
