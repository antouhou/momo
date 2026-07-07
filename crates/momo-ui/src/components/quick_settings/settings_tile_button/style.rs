use std::time::Duration;
use daiko::{
    animation::{AnimationParameters, easing::EasingFunction, transition},
    component::ComponentContext,
    layout::{AlignItems, FlexDirection, ItemSize, JustifyContent},
    style::{Border, BorderRadius, CursorIcon, Stroke, Style},
    widgets::text::TextStyle,
};
use super::super::{
    common::QuickSettingsControlState,
    style::{
        CONTROL_TRANSITION_MS, SETTINGS_TILE_CONTENT_GAP, SETTINGS_TILE_HEIGHT,
        SETTINGS_TILE_PADDING, SETTINGS_TILE_TEXT_HEIGHT, SETTINGS_TILE_WIDTH, TILE_RADIUS,
        settings_button_focus_transform, settings_label_text_style, settings_surface_border_color,
        settings_surface_border_focus_color, settings_surface_border_hover_color,
        settings_surface_color, settings_surface_focus_color, settings_surface_hover_color,
        settings_text_color, settings_tile_icon_background_color, settings_tile_icon_border_color,
    },
};

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
) -> Style {
    let background = if state.is_focused {
        settings_surface_focus_color()
    } else if state.is_hovered {
        settings_surface_hover_color()
    } else {
        settings_surface_color()
    };
    let border_color = if state.is_focused {
        settings_surface_border_focus_color()
    } else if state.is_hovered {
        settings_surface_border_hover_color()
    } else {
        settings_surface_border_color()
    };

    Style::new()
        .with_fixed_size(SETTINGS_TILE_WIDTH, SETTINGS_TILE_HEIGHT)
        .with_direction(FlexDirection::Row)
        .with_align_items(AlignItems::Center)
        .with_padding(SETTINGS_TILE_PADDING)
        .with_transform(Some(settings_button_focus_transform(
            SETTINGS_TILE_WIDTH,
            SETTINGS_TILE_HEIGHT,
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
        .with_border_radius(BorderRadius::all(TILE_RADIUS))
        .with_cursor(CursorIcon::PointingHand)
}

pub(crate) fn settings_tile_icon_style(ctx: &mut ComponentContext, is_active: bool) -> Style {
    Style::new()
        .with_fixed_size(SETTINGS_TILE_TEXT_HEIGHT, SETTINGS_TILE_TEXT_HEIGHT)
        .with_direction(FlexDirection::Row)
        .with_align_items(AlignItems::Center)
        .with_justify_content(JustifyContent::Center)
        .with_background_color(transition(
            settings_tile_icon_background_color(is_active),
            AnimationParameters::default()
                .with_duration(Duration::from_millis(CONTROL_TRANSITION_MS))
                .with_easing(EasingFunction::EaseOut)
                .to_transition_options(),
            ctx,
        ))
        .with_border(Border::uniform(Stroke::new(
            1.0,
            transition(
                settings_tile_icon_border_color(is_active),
                AnimationParameters::default()
                    .with_duration(Duration::from_millis(CONTROL_TRANSITION_MS))
                    .with_easing(EasingFunction::EaseOut)
                    .to_transition_options(),
                ctx,
            ),
        )))
        .with_border_radius(BorderRadius::all(SETTINGS_TILE_TEXT_HEIGHT * 0.5))
}

pub(crate) fn tile_title_style() -> TextStyle {
    settings_label_text_style(settings_text_color())
}
