use daiko::component::ComponentContext;
use daiko::style::{Border, BorderRadius, Color, Indent, Stroke, Style};
use daiko::widgets::text::{Text, TextStyle, TextWrap};

pub const SETTINGS_MENU_OFFSET: f32 = 12.0;
const SETTINGS_MENU_WIDTH: f32 = 220.0;

pub fn settings_exit_button_style(
    button_state: &daiko::widgets::button::state::ButtonState,
    _ctx: &mut ComponentContext,
) -> Style {
    let background = if button_state.is_pressed {
        Color::from_rgb(54, 24, 30)
    } else if button_state.is_hovered || button_state.is_focused {
        Color::from_rgb(74, 31, 40)
    } else {
        Color::from_rgb(62, 26, 34)
    };

    Style::new()
        .with_direction(daiko::layout::FlexDirection::Row)
        .with_align_items(daiko::layout::AlignItems::Center)
        .with_justify_content(daiko::layout::JustifyContent::Center)
        .with_fixed_width(daiko::layout::ItemSize::Percent(1.0))
        .with_padding(Indent::from((14.0, 12.0)))
        .with_background_color(background)
        .with_border(Border::uniform(Stroke::new(
            1.0,
            Color::from_rgb(148, 91, 101),
        )))
        .with_border_radius(BorderRadius::all(14.0))
}

pub fn settings_menu_style() -> Style {
    Style::new()
        .with_fixed_width(daiko::layout::ItemSize::Points(SETTINGS_MENU_WIDTH))
        .with_padding(16.0)
        .with_direction(daiko::layout::FlexDirection::Column)
        .with_background_color(Color::from_rgb(13, 20, 31))
        .with_border(Border::uniform(Stroke::new(
            1.0,
            Color::from_rgb(72, 93, 124),
        )))
        .with_border_radius(BorderRadius::all(20.0))
}

pub fn menu_heading() -> Text {
    Text::new("Settings").with_style(
        TextStyle::default()
            .with_font_size(16.0)
            .with_font_color(Color::from_rgb(240, 245, 255))
            .with_wrap(TextWrap::NoWrap),
    )
}
