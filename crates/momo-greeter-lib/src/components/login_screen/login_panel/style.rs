use crate::components::login_screen::style::{
    CONTROL_GAP, SECTION_GAP, text_primary_color, text_secondary_color,
};
use daiko::layout::{AlignItems, FlexDirection, ItemSize, JustifyContent, SizeConstraint};
use daiko::style::{Border, BorderRadius, Color, Indent, Stroke, Style};
use daiko::widgets::text::{TextStyle, TextWrap, Weight};

const PANEL_WIDTH: f32 = 470.0;
const PANEL_RADIUS: f32 = 28.0;
const PANEL_PADDING: f32 = 28.0;
const PANEL_GAP: f32 = 16.0;
const AVATAR_SIZE: f32 = 78.0;
const AVATAR_RADIUS: f32 = AVATAR_SIZE / 2.0;
const AVATAR_TEXT_SIZE: f32 = 30.0;
const INPUT_HEIGHT: f32 = 54.0;
const INPUT_RADIUS: f32 = 14.0;
const INPUT_HORIZONTAL_PADDING: f32 = 18.0;
const INPUT_LABEL_SIZE: f32 = 15.0;

pub(super) fn content_style() -> Style {
    Style::new()
        .with_direction(FlexDirection::Column)
        .with_align_items(AlignItems::Center)
        .with_spacing((SECTION_GAP, SECTION_GAP))
        .with_size_constraint(SizeConstraint::exact_content_size())
}

pub(super) fn panel_style() -> Style {
    Style::new()
        .with_fixed_width(ItemSize::Points(PANEL_WIDTH))
        .with_direction(FlexDirection::Column)
        .with_align_items(AlignItems::Center)
        .with_spacing((PANEL_GAP, PANEL_GAP))
        .with_padding(Indent::uniform(PANEL_PADDING))
        .with_background_color(Color::from_rgba_unmultiplied(7, 12, 28, 180))
        .with_border(Border::uniform(Stroke::new(
            1.0,
            Color::from_rgba_unmultiplied(255, 255, 255, 42),
        )))
        .with_border_radius(BorderRadius::all(PANEL_RADIUS))
}

pub(super) fn avatar_style(user_index: usize) -> Style {
    Style::new()
        .with_fixed_size(AVATAR_SIZE, AVATAR_SIZE)
        .with_centered_content()
        .with_background_color(profile_color(user_index))
        .with_border_radius(BorderRadius::all(AVATAR_RADIUS))
}

pub(super) fn avatar_text_style() -> TextStyle {
    TextStyle::default()
        .with_font_size(AVATAR_TEXT_SIZE)
        .with_weight(Weight::LIGHT)
        .with_font_color(text_primary_color())
        .with_center_alignment()
        .with_wrap(TextWrap::NoWrap)
}

pub(super) fn input_label_text_style() -> TextStyle {
    TextStyle::default()
        .with_font_size(INPUT_LABEL_SIZE)
        .with_font_color(text_secondary_color())
        .with_wrap(TextWrap::NoWrap)
}

pub(super) fn input_style() -> Style {
    Style::new()
        .with_fixed_width(ItemSize::Points(PANEL_WIDTH - PANEL_PADDING * 2.0))
        .with_fixed_height(ItemSize::Points(INPUT_HEIGHT))
        .with_padding(Indent::uniform_horizontal(INPUT_HORIZONTAL_PADDING))
        .with_background_color(Color::from_rgba_unmultiplied(255, 255, 255, 20))
        .with_border(Border::uniform(Stroke::new(
            1.0,
            Color::from_rgba_unmultiplied(255, 255, 255, 72),
        )))
        .with_border_radius(BorderRadius::all(INPUT_RADIUS))
}

pub(super) fn actions_style() -> Style {
    Style::new()
        .with_direction(FlexDirection::Row)
        .with_align_items(AlignItems::Center)
        .with_justify_content(JustifyContent::Center)
        .with_spacing((CONTROL_GAP, CONTROL_GAP))
}

fn profile_color(user_index: usize) -> Color {
    match user_index % 3 {
        0 => Color::from_rgb(34, 105, 152),
        1 => Color::from_rgb(118, 65, 135),
        _ => Color::from_rgb(54, 91, 82),
    }
}
