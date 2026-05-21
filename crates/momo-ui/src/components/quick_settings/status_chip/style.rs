use super::super::common::QuickSettingsControlState;
use super::super::style::{
    CONTROL_RADIUS, CONTROL_TRANSITION_MS, SETTINGS_STATUS_CHIP_CONTENT_GAP,
    SETTINGS_STATUS_CHIP_HEIGHT, SETTINGS_STATUS_CHIP_PADDING, SETTINGS_STATUS_CHIP_WIDTH,
};
use daiko::animation::easing::EasingFunction;
use daiko::animation::{AnimationParameters, transition};
use daiko::component::ComponentContext;
use daiko::layout::{AlignItems, FlexDirection, JustifyContent};
use daiko::style::{Border, BorderRadius, Color, CursorIcon, Stroke, Style};
use daiko::widgets::text::{TextStyle, TextWrap, Weight};
use std::time::Duration;

pub(crate) fn status_chip_content_style() -> Style {
    Style::new()
        .with_direction(FlexDirection::Row)
        .with_spacing((
            SETTINGS_STATUS_CHIP_CONTENT_GAP,
            SETTINGS_STATUS_CHIP_CONTENT_GAP,
        ))
        .with_align_items(AlignItems::Center)
        .with_justify_content(JustifyContent::Center)
}

pub(crate) fn settings_status_chip_style(
    state: QuickSettingsControlState,
    ctx: &mut ComponentContext,
) -> Style {
    let background = if state.is_highlighted() {
        Color::from_rgb(236, 240, 243)
    } else {
        Color::from_rgb(214, 220, 226)
    };
    let border_color = if state.is_highlighted() {
        Color::from_rgb(236, 240, 243)
    } else {
        Color::from_rgba_unmultiplied(255, 255, 255, 48)
    };

    Style::new()
        .with_fixed_size(SETTINGS_STATUS_CHIP_WIDTH, SETTINGS_STATUS_CHIP_HEIGHT)
        .with_direction(FlexDirection::Row)
        .with_align_items(AlignItems::Center)
        .with_justify_content(JustifyContent::Center)
        .with_padding(SETTINGS_STATUS_CHIP_PADDING)
        .with_background_color(transition(
            background,
            AnimationParameters::default()
                .with_duration(Duration::from_millis(CONTROL_TRANSITION_MS))
                .with_easing(EasingFunction::EaseOut)
                .to_transition_options(),
            ctx,
        ))
        .with_border(Border::uniform(Stroke::new(
            1.0,
            transition(
                border_color,
                AnimationParameters::default()
                    .with_duration(Duration::from_millis(CONTROL_TRANSITION_MS))
                    .with_easing(EasingFunction::EaseOut)
                    .to_transition_options(),
                ctx,
            ),
        )))
        .with_border_radius(BorderRadius::all(CONTROL_RADIUS))
        .with_cursor(CursorIcon::PointingHand)
}

pub(crate) fn status_value_style() -> TextStyle {
    TextStyle::default()
        .with_font_size(18.0)
        .with_weight(Weight::NORMAL)
        .with_font_color(Color::from_rgb(12, 16, 20))
        .with_wrap(TextWrap::NoWrap)
}
