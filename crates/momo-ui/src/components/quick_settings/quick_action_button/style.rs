use super::super::{
    common::QuickSettingsControlState,
    style::{
        CONTROL_RADIUS, CONTROL_TRANSITION_MS, SETTINGS_ROUND_BUTTON_SIZE,
        settings_bright_surface_border_color, settings_bright_surface_border_focus_color,
        settings_bright_surface_color, settings_bright_surface_focus_color,
        settings_button_focus_transform, settings_danger_surface_border_color,
        settings_danger_surface_border_focus_color, settings_danger_surface_border_hover_color,
        settings_danger_surface_color, settings_danger_surface_focus_color,
        settings_danger_surface_hover_color, settings_surface_border_color, settings_surface_color,
    },
};
use crate::components::quick_settings::style::settings_accent_color;
use daiko::{
    animation::{AnimationParameters, easing::EasingFunction, transition},
    component::ComponentContext,
    layout::{AlignItems, FlexDirection, JustifyContent},
    style::{Border, BorderRadius, CursorIcon, Stroke, Style},
};
use std::time::Duration;

pub(crate) fn settings_round_button_style(
    state: QuickSettingsControlState,
    ctx: &mut ComponentContext,
    is_active: bool,
    is_danger: bool,
) -> Style {
    let background = if is_danger && state.is_focused {
        settings_danger_surface_focus_color()
    } else if is_danger && state.is_hovered {
        settings_danger_surface_hover_color()
    } else if is_danger {
        settings_danger_surface_color()
    } else if state.is_focused {
        settings_bright_surface_focus_color()
    } else if state.is_hovered {
        settings_bright_surface_color()
    } else if is_active {
        settings_accent_color()
    } else {
        settings_surface_color()
    };
    let border_color = if is_danger && state.is_focused {
        settings_danger_surface_border_focus_color()
    } else if is_danger && state.is_hovered {
        settings_danger_surface_border_hover_color()
    } else if is_danger {
        settings_danger_surface_border_color()
    } else if state.is_focused {
        settings_bright_surface_border_focus_color()
    } else if state.is_hovered {
        settings_bright_surface_border_color()
    } else {
        settings_surface_border_color()
    };

    Style::new()
        .with_fixed_size(SETTINGS_ROUND_BUTTON_SIZE, SETTINGS_ROUND_BUTTON_SIZE)
        .with_direction(FlexDirection::Row)
        .with_align_items(AlignItems::Center)
        .with_justify_content(JustifyContent::Center)
        .with_transform(Some(settings_button_focus_transform(
            SETTINGS_ROUND_BUTTON_SIZE,
            SETTINGS_ROUND_BUTTON_SIZE,
            state.is_focused,
            ctx,
        )))
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
