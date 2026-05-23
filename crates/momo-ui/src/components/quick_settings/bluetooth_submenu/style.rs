use super::super::common::QuickSettingsControlState;
use super::super::style::{
    CONTROL_RADIUS, CONTROL_TRANSITION_MS, SETTINGS_COMPACT_CONTENT_GAP, SETTINGS_MENU_GAP,
    SETTINGS_MENU_INNER_WIDTH, SETTINGS_ROUND_BUTTON_SIZE, SETTINGS_SUBMENU_BUTTON_PADDING,
    SETTINGS_SUBMENU_DEVICE_ICON_RING_SIZE, SETTINGS_SUBMENU_DEVICE_ROW_PADDING,
    SETTINGS_SUBMENU_SECTION_LABEL_HEIGHT, SETTINGS_SUBMENU_SECTION_PADDING,
    SETTINGS_SUBMENU_SECTION_TITLE_TEXT_SIZE, SETTINGS_SUBMENU_SWITCH_HEIGHT,
    SETTINGS_SUBMENU_SWITCH_INSET, SETTINGS_SUBMENU_SWITCH_KNOB_SIZE,
    SETTINGS_SUBMENU_SWITCH_KNOB_Y, SETTINGS_SUBMENU_SWITCH_WIDTH,
    SETTINGS_SUBMENU_TOGGLE_PADDING, settings_accent_border_color, settings_accent_color,
    settings_accent_text_color, settings_bright_surface_color,
    settings_bright_surface_muted_color, settings_inverse_text_color,
    settings_label_text_style, settings_panel_border_color,
    settings_submenu_device_available_border_color,
    settings_submenu_device_available_surface_color,
    settings_submenu_device_unavailable_border_color,
    settings_submenu_device_unavailable_surface_color, settings_surface_border_color,
    settings_surface_border_hover_color, settings_surface_color, settings_surface_hover_color,
    settings_surface_muted_color, settings_text_color,
};
use daiko::Vec2;
use daiko::animation::easing::EasingFunction;
use daiko::animation::{AnimationParameters, transition};
use daiko::component::ComponentContext;
use daiko::layout::{AlignItems, FlexDirection, ItemSize, JustifyContent, SizeConstraint};
use daiko::style::{Border, BorderRadius, CursorIcon, Overflow, Stroke, Style};
use daiko::widgets::text::TextStyle;
use std::time::Duration;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum DeviceRowAvailability {
    Connected,
    Available,
    Unavailable,
}

pub(super) fn bluetooth_submenu_style() -> Style {
    Style::new()
        .with_size_constraint(SizeConstraint::exact_content_height().with_exact_width(SETTINGS_MENU_INNER_WIDTH))
        .with_direction(FlexDirection::Column)
        .with_spacing((SETTINGS_MENU_GAP, SETTINGS_MENU_GAP))
}

pub(super) fn bluetooth_submenu_body_style() -> Style {
    Style::new()
        .with_size_constraint(SizeConstraint::exact_content_height().with_exact_width(SETTINGS_MENU_INNER_WIDTH))
        .with_direction(FlexDirection::Column)
        .with_spacing((SETTINGS_MENU_GAP, SETTINGS_MENU_GAP))
}

pub(super) fn submenu_back_button_style(
    state: QuickSettingsControlState,
    ctx: &mut ComponentContext,
) -> Style {
    menu_button_surface_style(state, ctx, false)
        .with_fixed_width(ItemSize::Percent(1.0))
        .with_padding(SETTINGS_SUBMENU_BUTTON_PADDING)
        .with_justify_content(JustifyContent::FlexStart)
}

pub(super) fn submenu_action_row_style(
    state: QuickSettingsControlState,
    ctx: &mut ComponentContext,
    is_emphasized: bool,
) -> Style {
    menu_button_surface_style(state, ctx, is_emphasized)
        .with_fixed_width(ItemSize::Percent(1.0))
        .with_fixed_height(ItemSize::Points(SETTINGS_ROUND_BUTTON_SIZE))
        .with_padding(SETTINGS_SUBMENU_BUTTON_PADDING)
}

pub(super) fn submenu_toggle_row_style(
    state: QuickSettingsControlState,
    ctx: &mut ComponentContext,
) -> Style {
    menu_button_surface_style(state, ctx, false)
        .with_fixed_width(ItemSize::Percent(1.0))
        .with_fixed_height(ItemSize::Points(SETTINGS_ROUND_BUTTON_SIZE))
        .with_padding(SETTINGS_SUBMENU_TOGGLE_PADDING)
}

fn menu_button_surface_style(
    state: QuickSettingsControlState,
    ctx: &mut ComponentContext,
    is_emphasized: bool,
) -> Style {
    let background = if is_emphasized && state.is_highlighted() {
        settings_bright_surface_color()
    } else if is_emphasized {
        settings_bright_surface_muted_color()
    } else if state.is_highlighted() {
        settings_surface_hover_color()
    } else {
        settings_surface_color()
    };
    let border_color = if is_emphasized && state.is_highlighted() {
        settings_bright_surface_color()
    } else if is_emphasized {
        settings_panel_border_color()
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

pub(super) fn submenu_section_style() -> Style {
    Style::new()
        .with_direction(FlexDirection::Column)
        .with_spacing((SETTINGS_COMPACT_CONTENT_GAP, SETTINGS_COMPACT_CONTENT_GAP))
}

pub(super) fn submenu_section_label_style() -> Style {
    Style::new()
        .with_padding(SETTINGS_SUBMENU_SECTION_PADDING)
        .with_fixed_height(ItemSize::Points(SETTINGS_SUBMENU_SECTION_LABEL_HEIGHT))
        .with_direction(FlexDirection::Row)
        .with_align_items(AlignItems::Center)
        .with_justify_content(JustifyContent::FlexStart)
}

pub(super) fn submenu_section_title_style() -> TextStyle {
    settings_label_text_style(settings_surface_muted_color())
        .with_font_size(SETTINGS_SUBMENU_SECTION_TITLE_TEXT_SIZE)
}

pub(super) fn submenu_action_label_style(is_emphasized: bool) -> TextStyle {
    if is_emphasized {
        settings_label_text_style(settings_inverse_text_color())
    } else {
        settings_label_text_style(settings_text_color())
    }
}

pub(super) fn submenu_device_row_style(
    state: QuickSettingsControlState,
    ctx: &mut ComponentContext,
    _availability: DeviceRowAvailability,
) -> Style {
    let background = if state.is_highlighted() {
        settings_surface_hover_color()
    } else {
        settings_surface_color()
    };
    let border_color = if state.is_highlighted() {
        settings_surface_border_hover_color()
    } else {
        settings_surface_border_color()
    };

    Style::new()
        .with_fixed_width(ItemSize::Percent(1.0))
        .with_fixed_height(ItemSize::Points(SETTINGS_ROUND_BUTTON_SIZE))
        .with_direction(FlexDirection::Row)
        .with_align_items(AlignItems::Center)
        .with_justify_content(JustifyContent::SpaceBetween)
        .with_padding(SETTINGS_SUBMENU_DEVICE_ROW_PADDING)
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

pub(super) fn submenu_device_icon_ring_style(
    availability: DeviceRowAvailability,
    ctx: &mut ComponentContext,
) -> Style {
    let (background_color, border_color) = match availability {
        DeviceRowAvailability::Connected => {
            (settings_accent_color(), settings_accent_border_color())
        }
        DeviceRowAvailability::Available => (
            settings_submenu_device_available_surface_color(),
            settings_submenu_device_available_border_color(),
        ),
        DeviceRowAvailability::Unavailable => (
            settings_submenu_device_unavailable_surface_color(),
            settings_submenu_device_unavailable_border_color(),
        ),
    };

    Style::new()
        .with_fixed_size(
            SETTINGS_SUBMENU_DEVICE_ICON_RING_SIZE,
            SETTINGS_SUBMENU_DEVICE_ICON_RING_SIZE,
        )
        .with_direction(FlexDirection::Row)
        .with_align_items(AlignItems::Center)
        .with_justify_content(JustifyContent::Center)
        .with_background_color(transition(
            background_color,
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
        .with_border_radius(BorderRadius::all(
            SETTINGS_SUBMENU_DEVICE_ICON_RING_SIZE * 0.5,
        ))
}

pub(super) fn submenu_device_label_style(availability: DeviceRowAvailability) -> TextStyle {
    settings_label_text_style(match availability {
        DeviceRowAvailability::Connected => settings_text_color(),
        DeviceRowAvailability::Available => settings_text_color(),
        DeviceRowAvailability::Unavailable => settings_surface_muted_color(),
    })
}

pub(super) fn submenu_device_icon_color(
    availability: DeviceRowAvailability,
) -> daiko::style::Color {
    match availability {
        DeviceRowAvailability::Connected => settings_accent_text_color(),
        DeviceRowAvailability::Available => settings_text_color(),
        DeviceRowAvailability::Unavailable => settings_surface_muted_color(),
    }
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
        .with_size_constraint(SizeConstraint::fixed(
            SETTINGS_SUBMENU_SWITCH_WIDTH,
            SETTINGS_SUBMENU_SWITCH_HEIGHT,
        ))
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
        .with_border_radius(BorderRadius::all(
            SETTINGS_SUBMENU_SWITCH_KNOB_SIZE * 0.5,
        ))
}
