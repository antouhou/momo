use super::super::common::QuickSettingsControlState;
use super::super::style::{
    SETTINGS_TILE_CONTENT_GAP, SETTINGS_TILE_HEIGHT, SETTINGS_TILE_PADDING,
    SETTINGS_TILE_TEXT_HEIGHT, SETTINGS_TILE_WIDTH, TILE_RADIUS,
};
use daiko::animation::easing::EasingFunction;
use daiko::animation::{AnimationParameters, transition};
use daiko::component::ComponentContext;
use daiko::layout::{AlignItems, FlexDirection, ItemSize, JustifyContent};
use daiko::style::{Border, BorderRadius, Color, CursorIcon, Stroke, Style};
use daiko::widgets::text::{TextStyle, TextWrap, Weight};
use std::time::Duration;

pub(crate) fn settings_tile_content_style() -> Style {
    Style::new()
        .with_direction(FlexDirection::Row)
        .with_align_items(AlignItems::Center)
        .with_justify_content(JustifyContent::FlexStart)
        .with_spacing((SETTINGS_TILE_CONTENT_GAP, SETTINGS_TILE_CONTENT_GAP))
}

pub(crate) fn settings_tile_text_column_style() -> Style {
    Style::new()
        .with_fixed_height(ItemSize::Points(SETTINGS_TILE_TEXT_HEIGHT))
        .with_direction(FlexDirection::Column)
        .with_justify_content(JustifyContent::Center)
        .with_align_items(AlignItems::FlexStart)
}

pub(crate) fn settings_tile_button_style(
    state: QuickSettingsControlState,
    ctx: &mut ComponentContext,
    is_active: bool,
) -> Style {
    let background = if is_active {
        Color::from_rgb(104, 79, 140)
    } else if state.is_highlighted() {
        Color::from_rgb(38, 42, 46)
    } else {
        Color::from_rgb(24, 28, 31)
    };
    let border_color = if is_active {
        Color::from_rgba_unmultiplied(211, 191, 255, 112)
    } else if state.is_highlighted() {
        Color::from_rgba_unmultiplied(255, 255, 255, 92)
    } else {
        Color::from_rgba_unmultiplied(255, 255, 255, 28)
    };

    Style::new()
        .with_fixed_size(SETTINGS_TILE_WIDTH, SETTINGS_TILE_HEIGHT)
        .with_direction(FlexDirection::Row)
        .with_align_items(AlignItems::Center)
        .with_padding(SETTINGS_TILE_PADDING)
        .with_background_color(transition(
            background,
            AnimationParameters::default()
                .with_duration(Duration::from_millis(120))
                .with_easing(EasingFunction::EaseOut)
                .to_transition_options(),
            ctx,
        ))
        .with_border(Border::uniform(Stroke::new(
            1.0,
            transition(
                border_color,
                AnimationParameters::default()
                    .with_duration(Duration::from_millis(120))
                    .with_easing(EasingFunction::EaseOut)
                    .to_transition_options(),
                ctx,
            ),
        )))
        .with_border_radius(BorderRadius::all(TILE_RADIUS))
        .with_cursor(CursorIcon::PointingHand)
}

pub(crate) fn settings_tile_icon_style(is_active: bool) -> Style {
    let background = if is_active {
        Color::from_rgba_unmultiplied(255, 255, 255, 26)
    } else {
        Color::from_rgba_unmultiplied(255, 255, 255, 12)
    };

    Style::new()
        .with_fixed_size(38.0, 38.0)
        .with_direction(FlexDirection::Row)
        .with_align_items(AlignItems::Center)
        .with_justify_content(JustifyContent::Center)
        .with_background_color(background)
        .with_border_radius(BorderRadius::all(19.0))
}

pub(crate) fn tile_title_style(is_active: bool) -> TextStyle {
    let color = if is_active {
        Color::from_rgb(248, 241, 255)
    } else {
        Color::from_rgb(235, 240, 247)
    };

    TextStyle::default()
        .with_font_size(18.0)
        .with_weight(Weight::NORMAL)
        .with_font_color(color)
        .with_wrap(TextWrap::NoWrap)
}
