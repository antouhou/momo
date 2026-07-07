use super::super::{
    common::QuickSettingsControlState,
    style::{
        CONTROL_TRANSITION_MS, SETTINGS_COMPACT_CONTENT_GAP, SETTINGS_ICON_FRAME_SIZE,
        SETTINGS_MENU_GAP, SETTINGS_MENU_INNER_WIDTH, SETTINGS_STATUS_CHIP_PADDING,
        SETTINGS_TILE_HEIGHT, SETTINGS_TILE_PADDING, SETTINGS_VOLUME_SLIDER_ROW_HEIGHT,
        TILE_RADIUS, settings_button_focus_transform, settings_label_text_style,
        settings_surface_border_color, settings_surface_border_focus_color,
        settings_surface_border_hover_color, settings_surface_color, settings_surface_focus_color,
        settings_surface_hover_color, settings_text_color,
    },
};
use daiko::{
    animation::{AnimationParameters, easing::EasingFunction, transition},
    component::ComponentContext,
    layout::{AlignItems, FlexDirection, ItemSize, JustifyContent},
    style::{Border, BorderRadius, CursorIcon, Indent, Stroke, Style},
    widgets::text::TextStyle,
};
use std::time::Duration;

const VOLUME_CONTROL_PADDING: Indent = Indent::new(
    SETTINGS_TILE_PADDING.left,
    SETTINGS_STATUS_CHIP_PADDING.top,
    SETTINGS_TILE_PADDING.right,
    SETTINGS_STATUS_CHIP_PADDING.bottom,
);

pub(crate) fn volume_control_style(
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
        .with_fixed_height(ItemSize::Points(SETTINGS_TILE_HEIGHT))
        .with_direction(FlexDirection::Column)
        .with_align_items(AlignItems::FlexStart)
        .with_justify_content(JustifyContent::Center)
        .with_padding(VOLUME_CONTROL_PADDING)
        .with_spacing((SETTINGS_COMPACT_CONTENT_GAP, SETTINGS_COMPACT_CONTENT_GAP))
        .with_transform(Some(settings_button_focus_transform(
            SETTINGS_MENU_INNER_WIDTH,
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

pub(crate) fn volume_label_style() -> TextStyle {
    settings_label_text_style(settings_text_color())
}

pub(crate) fn volume_label_container_style() -> Style {
    Style::new()
        .with_fixed_height(ItemSize::Points(SETTINGS_ICON_FRAME_SIZE))
        .with_direction(FlexDirection::Row)
        .with_align_items(AlignItems::Center)
        .with_justify_content(JustifyContent::FlexStart)
}

pub(crate) fn volume_slider_row_style() -> Style {
    Style::new()
        .with_fixed_height(ItemSize::Points(SETTINGS_VOLUME_SLIDER_ROW_HEIGHT))
        .with_direction(FlexDirection::Row)
        .with_align_items(AlignItems::Center)
        .with_spacing((SETTINGS_MENU_GAP, SETTINGS_MENU_GAP))
}

pub(crate) fn volume_slider_track_style() -> Style {
    Style::new()
        .with_grow(1.0)
        .with_fixed_height(ItemSize::Points(SETTINGS_VOLUME_SLIDER_ROW_HEIGHT))
}
