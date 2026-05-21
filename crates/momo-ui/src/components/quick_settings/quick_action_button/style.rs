use super::super::common::QuickSettingsControlState;
use super::super::style::{CONTROL_RADIUS, CONTROL_TRANSITION_MS, SETTINGS_ROUND_BUTTON_SIZE};
use daiko::animation::easing::EasingFunction;
use daiko::animation::{AnimationParameters, transition};
use daiko::component::ComponentContext;
use daiko::layout::{AlignItems, FlexDirection, JustifyContent};
use daiko::style::{Border, BorderRadius, Color, CursorIcon, Stroke, Style};
use std::time::Duration;

pub(crate) fn settings_round_button_style(
    state: QuickSettingsControlState,
    ctx: &mut ComponentContext,
    is_active: bool,
    is_danger: bool,
) -> Style {
    let background = if is_danger && state.is_highlighted() {
        Color::from_rgb(92, 32, 43)
    } else if is_danger {
        Color::from_rgb(74, 28, 36)
    } else if is_active || state.is_highlighted() {
        Color::from_rgb(236, 240, 243)
    } else {
        Color::from_rgb(24, 28, 31)
    };
    let border_color = if is_danger && state.is_highlighted() {
        Color::from_rgba_unmultiplied(255, 189, 198, 184)
    } else if is_danger {
        Color::from_rgba_unmultiplied(255, 160, 174, 72)
    } else if is_active || state.is_highlighted() {
        Color::from_rgba_unmultiplied(255, 255, 255, 138)
    } else {
        Color::from_rgba_unmultiplied(255, 255, 255, 30)
    };

    Style::new()
        .with_fixed_size(SETTINGS_ROUND_BUTTON_SIZE, SETTINGS_ROUND_BUTTON_SIZE)
        .with_direction(FlexDirection::Row)
        .with_align_items(AlignItems::Center)
        .with_justify_content(JustifyContent::Center)
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
