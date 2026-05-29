use super::super::common::QuickSettingsControlState;
use super::super::style::{
    CONTROL_RADIUS, CONTROL_TRANSITION_MS, SETTINGS_COMPACT_CONTENT_GAP,
    SETTINGS_ROUND_BUTTON_SIZE, SETTINGS_SUBMENU_BUTTON_PADDING, SETTINGS_SUBMENU_SWITCH_HEIGHT,
    SETTINGS_SUBMENU_SWITCH_INSET, SETTINGS_SUBMENU_SWITCH_KNOB_SIZE,
    SETTINGS_SUBMENU_SWITCH_KNOB_Y, SETTINGS_SUBMENU_SWITCH_WIDTH, SETTINGS_SUBMENU_TOGGLE_PADDING,
    settings_accent_border_color, settings_accent_color, settings_bright_surface_color,
    settings_emphasized_surface_border_color, settings_emphasized_surface_border_hover_color,
    settings_emphasized_surface_color, settings_emphasized_surface_hover_color,
    settings_label_text_style, settings_surface_border_color, settings_surface_border_hover_color,
    settings_surface_color, settings_surface_hover_color, settings_surface_muted_color,
    settings_text_color,
};
use super::{SubmenuButtonState, SubmenuButtonSurface};
use daiko::animation::easing::EasingFunction;
use daiko::animation::{AnimationParameters, transition};
use daiko::component::ComponentContext;
use daiko::layout::{AlignItems, FlexDirection, ItemSize, JustifyContent, SizeConstraint};
use daiko::style::{Border, BorderRadius, Color, CursorIcon, Overflow, Stroke};
use daiko::widgets::text::TextStyle;
use daiko::{Vec2, style::Style};
use std::time::Duration;

pub(super) fn submenu_button_style(
    state: QuickSettingsControlState,
    ctx: &mut ComponentContext,
    surface: SubmenuButtonSurface,
    has_trailing_control: bool,
) -> Style {
    let is_emphasized = matches!(surface, SubmenuButtonSurface::Emphasized);
    let padding = if has_trailing_control {
        SETTINGS_SUBMENU_TOGGLE_PADDING
    } else {
        SETTINGS_SUBMENU_BUTTON_PADDING
    };

    menu_button_surface_style(state, ctx, is_emphasized)
        .with_fixed_width(ItemSize::Percent(1.0))
        .with_padding(padding)
}

pub(super) fn submenu_button_label_style(foreground_color: Color) -> TextStyle {
    settings_label_text_style(foreground_color)
}

pub(super) fn submenu_button_foreground_color(
    surface: SubmenuButtonSurface,
    state: SubmenuButtonState,
) -> Color {
    match (surface, state) {
        (SubmenuButtonSurface::Emphasized, _) => settings_text_color(),
        (_, SubmenuButtonState::Enabled) => settings_text_color(),
        (_, SubmenuButtonState::Disabled) => settings_surface_muted_color(),
    }
}

pub(super) fn submenu_label_group_style() -> Style {
    Style::new()
        .with_direction(FlexDirection::Row)
        .with_align_items(AlignItems::Center)
        .with_spacing((SETTINGS_COMPACT_CONTENT_GAP, SETTINGS_COMPACT_CONTENT_GAP))
}

pub(super) fn submenu_leading_slot_style() -> Style {
    Style::new()
        .with_fixed_size(SETTINGS_ROUND_BUTTON_SIZE, SETTINGS_ROUND_BUTTON_SIZE)
        .with_direction(FlexDirection::Row)
        .with_align_items(AlignItems::Center)
        .with_justify_content(JustifyContent::Center)
}

pub(super) fn submenu_toggle_switch_style(ctx: &mut ComponentContext, is_enabled: bool) -> Style {
    let background = if is_enabled {
        settings_accent_color()
    } else {
        settings_surface_muted_color()
    };
    let border_color = if is_enabled {
        settings_accent_border_color()
    } else {
        settings_surface_border_color()
    };

    Style::new()
        .with_fixed_size(
            SETTINGS_SUBMENU_SWITCH_WIDTH,
            SETTINGS_SUBMENU_SWITCH_HEIGHT,
        )
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
        .with_border_radius(BorderRadius::all(SETTINGS_SUBMENU_SWITCH_HEIGHT * 0.5))
        .with_overflow(Overflow::Hidden)
}

pub(super) fn submenu_toggle_knob_style(ctx: &mut ComponentContext, is_enabled: bool) -> Style {
    let x = if is_enabled {
        SETTINGS_SUBMENU_SWITCH_WIDTH
            - SETTINGS_SUBMENU_SWITCH_KNOB_SIZE
            - SETTINGS_SUBMENU_SWITCH_INSET
    } else {
        SETTINGS_SUBMENU_SWITCH_INSET
    };

    Style::new()
        .with_absolute_position(transition(
            Vec2::new(x, SETTINGS_SUBMENU_SWITCH_KNOB_Y),
            AnimationParameters::default()
                .with_duration(Duration::from_millis(CONTROL_TRANSITION_MS))
                .with_easing(EasingFunction::EaseOut)
                .to_transition_options(),
            ctx,
        ))
        .with_size_constraint(SizeConstraint::fixed(
            SETTINGS_SUBMENU_SWITCH_KNOB_SIZE,
            SETTINGS_SUBMENU_SWITCH_KNOB_SIZE,
        ))
        .with_background_color(settings_bright_surface_color())
        .with_border_radius(BorderRadius::all(SETTINGS_SUBMENU_SWITCH_KNOB_SIZE * 0.5))
}

fn menu_button_surface_style(
    state: QuickSettingsControlState,
    ctx: &mut ComponentContext,
    is_emphasized: bool,
) -> Style {
    let background = if is_emphasized && state.is_highlighted() {
        settings_emphasized_surface_hover_color()
    } else if is_emphasized {
        settings_emphasized_surface_color()
    } else if state.is_highlighted() {
        settings_surface_hover_color()
    } else {
        settings_surface_color()
    };
    let border_color = if is_emphasized && state.is_focused {
        settings_emphasized_surface_border_hover_color()
    } else if is_emphasized {
        settings_emphasized_surface_border_color()
    } else if state.is_highlighted() {
        settings_surface_border_hover_color()
    } else {
        settings_surface_border_color()
    };

    Style::new()
        .with_fixed_height(ItemSize::Points(SETTINGS_ROUND_BUTTON_SIZE))
        .with_direction(FlexDirection::Row)
        .with_align_items(AlignItems::Center)
        .with_justify_content(JustifyContent::SpaceBetween)
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
