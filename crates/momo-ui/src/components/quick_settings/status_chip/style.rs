use super::super::common::QuickSettingsControlState;
use super::super::style::{
    settings_button_focus_transform, settings_emphasized_surface_border_color,
    settings_emphasized_surface_border_focus_color, settings_emphasized_surface_color,
    settings_emphasized_surface_focus_color, settings_emphasized_surface_hover_color,
    settings_label_text_style, settings_text_color, CONTROL_RADIUS, CONTROL_TRANSITION_MS,
    SETTINGS_COMPACT_CONTENT_GAP, SETTINGS_STATUS_CHIP_HEIGHT, SETTINGS_STATUS_CHIP_PADDING,
    SETTINGS_STATUS_CHIP_WIDTH,
};
use daiko::animation::easing::EasingFunction;
use daiko::animation::{transition, AnimationParameters};
use daiko::component::ComponentContext;
use daiko::layout::{AlignItems, FlexDirection, JustifyContent};
use daiko::style::{Border, BorderRadius, CursorIcon, Stroke, Style};
use daiko::widgets::text::TextStyle;
use std::time::Duration;

pub(crate) fn status_chip_content_style() -> Style {
    Style::new()
        .with_direction(FlexDirection::Row)
        .with_spacing((SETTINGS_COMPACT_CONTENT_GAP, SETTINGS_COMPACT_CONTENT_GAP))
        .with_align_items(AlignItems::Center)
        .with_justify_content(JustifyContent::Center)
}

pub(crate) fn settings_status_chip_style(
    state: QuickSettingsControlState,
    ctx: &mut ComponentContext,
) -> Style {
    let background = if state.is_focused {
        settings_emphasized_surface_focus_color()
    } else if state.is_hovered {
        settings_emphasized_surface_hover_color()
    } else {
        settings_emphasized_surface_color()
    };
    let border_color = if state.is_focused {
        settings_emphasized_surface_border_focus_color()
    } else if state.is_hovered {
        settings_emphasized_surface_border_color()
    } else {
        settings_emphasized_surface_border_color()
    };

    Style::new()
        .with_fixed_size(SETTINGS_STATUS_CHIP_WIDTH, SETTINGS_STATUS_CHIP_HEIGHT)
        .with_direction(FlexDirection::Row)
        .with_align_items(AlignItems::Center)
        .with_justify_content(JustifyContent::Center)
        .with_padding(SETTINGS_STATUS_CHIP_PADDING)
        .with_transform(Some(settings_button_focus_transform(
            SETTINGS_STATUS_CHIP_WIDTH,
            SETTINGS_STATUS_CHIP_HEIGHT,
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

pub(crate) fn status_value_style() -> TextStyle {
    settings_label_text_style(settings_text_color())
}
