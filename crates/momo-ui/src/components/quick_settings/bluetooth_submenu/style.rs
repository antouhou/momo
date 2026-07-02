use super::super::style::{
    CONTROL_TRANSITION_MS, SETTINGS_SUBMENU_DEVICE_ICON_RING_SIZE, settings_accent_border_color,
    settings_accent_color, settings_accent_text_color,
    settings_submenu_device_available_border_color,
    settings_submenu_device_available_surface_color,
    settings_submenu_device_unavailable_border_color,
    settings_submenu_device_unavailable_surface_color, settings_surface_muted_color,
    settings_text_color, settings_warning_border_color, settings_warning_surface_color,
    settings_warning_text_color,
};
use daiko::animation::easing::EasingFunction;
use daiko::animation::{AnimationParameters, transition};
use daiko::component::ComponentContext;
use daiko::layout::{AlignItems, FlexDirection, JustifyContent};
use daiko::style::{Border, BorderRadius, Stroke, Style};
use std::time::Duration;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum DeviceRowAvailability {
    Connected,
    Connecting,
    Disconnecting,
    Available,
    Unavailable,
}

pub(super) fn submenu_device_icon_ring_style(
    availability: DeviceRowAvailability,
    ctx: &mut ComponentContext,
) -> Style {
    let (background_color, border_color) = match availability {
        DeviceRowAvailability::Connected => {
            (settings_accent_color(), settings_accent_border_color())
        }
        DeviceRowAvailability::Connecting => (
            settings_warning_surface_color(),
            settings_warning_border_color(),
        ),
        DeviceRowAvailability::Disconnecting => {
            (settings_accent_color(), settings_warning_border_color())
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

pub(super) fn submenu_device_label_color(
    availability: DeviceRowAvailability,
) -> daiko::style::Color {
    match availability {
        DeviceRowAvailability::Connected | DeviceRowAvailability::Available => {
            settings_text_color()
        }
        DeviceRowAvailability::Connecting | DeviceRowAvailability::Disconnecting => {
            settings_warning_text_color()
        }
        DeviceRowAvailability::Unavailable => settings_surface_muted_color(),
    }
}

pub(super) fn submenu_device_icon_color(
    availability: DeviceRowAvailability,
) -> daiko::style::Color {
    match availability {
        DeviceRowAvailability::Connected => settings_accent_text_color(),
        DeviceRowAvailability::Connecting | DeviceRowAvailability::Disconnecting => {
            settings_warning_text_color()
        }
        DeviceRowAvailability::Available => settings_text_color(),
        DeviceRowAvailability::Unavailable => settings_surface_muted_color(),
    }
}
