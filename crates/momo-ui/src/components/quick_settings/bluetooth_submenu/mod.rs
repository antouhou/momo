mod style;

use self::style::{
    DeviceRowAvailability, bluetooth_submenu_body_style, bluetooth_submenu_style,
    submenu_device_icon_color, submenu_device_icon_ring_style, submenu_device_label_color,
    submenu_section_label_style, submenu_section_style, submenu_section_title_style,
};
use super::common::{
    QuickSettingsControlState, QuickSettingsGlyph, glyph_element, settings_bottom_row, settings_row,
};
use super::state::{
    SETTINGS_MENU_STATE_ID, SETTINGS_VIEW_TRANSITION_ID, SettingsMenuState, SettingsMenuView,
};
use super::style::{SETTINGS_ICON_FRAME_SIZE, SETTINGS_ICON_SIZE, settings_text_color};
use super::submenu_button::{
    SubmenuButton, SubmenuButtonState, SubmenuButtonSurface, submenu_button_glyph,
    submenu_button_leading_slot, submenu_button_surface_glyph, submenu_toggle_switch,
};
use crate::components::home::bluetooth::{
    BluetoothDeviceSection, BluetoothDeviceState, bluetooth_handle, bluetooth_state,
};
use daiko::component::{Component, ComponentContext};
use daiko::navigation::{FocusEntryPolicy, FocusOrigin, NavigationInputAction};
use daiko::widgets::scrollable::Scrollable;
use daiko::widgets::text::Text;
use daiko::{Element, Id, Vec2};
use system_control::{BluetoothConnectionState, BluetoothDeviceCategory};
use tracing::warn;

const BLUETOOTH_ICON: &[u8] = include_bytes!("../../../../assets/bluetooth-b.svg");
const KEYBOARD_ICON: &[u8] = include_bytes!("../../../../assets/keyboard.svg");
const AUDIO_ICON: &[u8] = include_bytes!("../../../../assets/volume.svg");
const AUDIO_FILE_ICON: &[u8] = include_bytes!("../../../../assets/file-audio.svg");
const PHONE_ICON: &[u8] = include_bytes!("../../../../assets/mobile-screen.svg");
const GEAR_ICON: &[u8] = include_bytes!("../../../../assets/gear-solid-full.svg");
const CAMERA_ICON: &[u8] = include_bytes!("../../../../assets/camera.svg");
const COMPUTER_ICON: &[u8] = include_bytes!("../../../../assets/computer.svg");
const DISPLAY_ICON: &[u8] = include_bytes!("../../../../assets/display.svg");
const GAME_CONTROLLER_ICON: &[u8] = include_bytes!("../../../../assets/gamepad.svg");
const HEALTH_ICON: &[u8] = include_bytes!("../../../../assets/heart.svg");
const HEADPHONES_ICON: &[u8] = include_bytes!("../../../../assets/headphones.svg");
const HEADSET_ICON: &[u8] = include_bytes!("../../../../assets/headset.svg");
const MEDIA_PLAYER_ICON: &[u8] = include_bytes!("../../../../assets/play-circle.svg");
const MICROPHONE_ICON: &[u8] = include_bytes!("../../../../assets/microphone.svg");
const MOUSE_ICON: &[u8] = include_bytes!("../../../../assets/computer-mouse.svg");
const NETWORK_ICON: &[u8] = include_bytes!("../../../../assets/network-wired.svg");
const PRINTER_ICON: &[u8] = include_bytes!("../../../../assets/print.svg");
const SENSOR_ICON: &[u8] = include_bytes!("../../../../assets/temperature-half.svg");
const TABLET_ICON: &[u8] = include_bytes!("../../../../assets/tablet.svg");

pub(super) const BLUETOOTH_TOGGLE_TAG: &str = "header-settings-bluetooth-toggle";
pub(super) const BLUETOOTH_SETTINGS_BUTTON_TAG: &str = "header-settings-bluetooth-settings-button";

#[derive(Clone, Copy)]
pub(super) struct BluetoothSubmenu;

impl Component for BluetoothSubmenu {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let focus_scope = ctx.focus_scope();
        focus_scope.set_entry_policy(FocusEntryPolicy::Remembered);
        focus_scope.capture_when_contains_focus(&[
            NavigationInputAction::Cancel,
            NavigationInputAction::Back,
        ]);

        let state =
            ctx.use_shared_state(Id::new(SETTINGS_MENU_STATE_ID), SettingsMenuState::default);
        let snapshot = *state.read();
        let should_go_back = focus_scope.drain_captured_actions().any(|action| {
            matches!(
                action,
                NavigationInputAction::Cancel | NavigationInputAction::Back
            )
        });

        if snapshot.active_view == SettingsMenuView::Bluetooth && should_go_back {
            *state.write() = SettingsMenuState {
                last_active_view: snapshot.active_view,
                active_view: SettingsMenuView::Main,
                ..snapshot
            };
        }
        let transition_status = crate::components::view_transition::view_transition_status(
            ctx,
            SETTINGS_VIEW_TRANSITION_ID,
        );

        Element::new()
            .with_tag("header-settings-bluetooth-submenu")
            .with_style(bluetooth_submenu_style())
            .with_content(settings_row(BluetoothToggleRow))
            .with_content(
                Scrollable::new(BluetoothSubmenuBody, "bluetooth_submenu_scrollable")
                    .size_to_content_with_clamp(Vec2::new(f32::INFINITY, f32::INFINITY))
                    .with_visible_scroll_bars(!transition_status.is_transitioning),
            )
            .with_content(settings_bottom_row(BluetoothSettingsButton))
    }
}

#[derive(Clone, Copy)]
struct BluetoothSubmenuBody;

impl Component for BluetoothSubmenuBody {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let bluetooth_state = bluetooth_state(ctx);
        let bluetooth_state = bluetooth_state.read();

        Element::new()
            .with_style(bluetooth_submenu_body_style())
            .with_content(device_section(
                "Recent",
                &bluetooth_state.recent_devices,
                bluetooth_state.is_enabled,
                "No recent devices",
            ))
            .with_content(device_section(
                "Nearby",
                &bluetooth_state.nearby_devices,
                bluetooth_state.is_enabled,
                "No nearby devices",
            ))
    }
}

#[derive(Clone, Copy)]
struct BluetoothToggleRow;

impl Component for BluetoothToggleRow {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let mut pointer = ctx.pointer();
        let focusable = ctx.focusable();
        let state =
            ctx.use_shared_state(Id::new(SETTINGS_MENU_STATE_ID), SettingsMenuState::default);
        let snapshot = *state.read();
        let is_active = snapshot.active_view == SettingsMenuView::Bluetooth;
        let bluetooth_state = bluetooth_state(ctx);
        let bluetooth_state = bluetooth_state.read();
        let toggle_enabled = bluetooth_state.can_toggle_power;
        let toggle_value = bluetooth_state.is_enabled;

        if pointer.just_pressed() {
            focusable.request_focus(FocusOrigin::Pointer);
        }

        if snapshot.last_active_view == SettingsMenuView::Main
            && snapshot.active_view == SettingsMenuView::Bluetooth
        {
            focusable.request_focus(FocusOrigin::Programmatic);
            if focusable.is_focused() {
                let mut menu_state = state.write_silent();
                menu_state.last_active_view = menu_state.active_view;
            }
        }

        if is_active
            && toggle_enabled
            && (pointer.just_pressed() || focusable.just_activated())
            && let Err(error) = bluetooth_handle(ctx).set_power_enabled(!toggle_value)
        {
            warn!("failed to toggle bluetooth power: {error:?}");
        }

        let control = QuickSettingsControlState {
            is_hovered: pointer.is_hovering(),
            is_focused: focusable.is_focused(),
        };

        SubmenuButton {
            tag: BLUETOOTH_TOGGLE_TAG.to_string(),
            label: "Bluetooth".to_string(),
            label_color: None,
            control,
            surface: SubmenuButtonSurface::Standard,
            state: if toggle_enabled {
                SubmenuButtonState::Enabled
            } else {
                SubmenuButtonState::Disabled
            },
            leading: submenu_button_glyph(
                QuickSettingsGlyph::Asset(BLUETOOTH_ICON),
                settings_text_color(),
            ),
            trailing: Some(submenu_toggle_switch(ctx, toggle_value)),
        }
        .to_element(ctx)
    }
}

#[derive(Clone, Copy)]
struct BluetoothSettingsButton;

impl Component for BluetoothSettingsButton {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let mut pointer = ctx.pointer();
        let focusable = ctx.focusable();

        if pointer.just_pressed() {
            focusable.request_focus(FocusOrigin::Pointer);
        }

        let control = QuickSettingsControlState {
            is_hovered: pointer.is_hovering(),
            is_focused: focusable.is_focused(),
        };

        SubmenuButton {
            tag: BLUETOOTH_SETTINGS_BUTTON_TAG.to_string(),
            label: "Settings".to_string(),
            label_color: None,
            control,
            surface: SubmenuButtonSurface::Emphasized,
            state: SubmenuButtonState::Enabled,
            leading: submenu_button_surface_glyph(
                QuickSettingsGlyph::Asset(GEAR_ICON),
                SubmenuButtonSurface::Emphasized,
                SubmenuButtonState::Enabled,
            ),
            trailing: None,
        }
        .to_element(ctx)
    }
}

#[derive(Clone)]
struct BluetoothDeviceRow {
    bluetooth_device: BluetoothDeviceState,
    is_bluetooth_enabled: bool,
}

impl Component for BluetoothDeviceRow {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let mut pointer = ctx.pointer();
        let focusable = ctx.focusable();
        let availability =
            effective_device_availability(&self.bluetooth_device, self.is_bluetooth_enabled);

        if pointer.just_pressed() {
            focusable.request_focus(FocusOrigin::Pointer);
        }

        if pointer.just_pressed() || focusable.just_activated() {
            match self.bluetooth_device.connection_state {
                BluetoothConnectionState::Connected => {
                    if let Err(error) = bluetooth_handle(ctx)
                        .disconnect_device(self.bluetooth_device.device_identifier.clone())
                    {
                        warn!("failed to disconnect bluetooth device: {error:?}");
                    }
                }
                BluetoothConnectionState::Disconnected
                    if self.is_bluetooth_enabled
                        && matches!(availability, DeviceRowAvailability::Available) =>
                {
                    if let Err(error) = bluetooth_handle(ctx)
                        .connect_device(self.bluetooth_device.device_identifier.clone())
                    {
                        warn!("failed to connect bluetooth device: {error:?}");
                    }
                }
                BluetoothConnectionState::Connecting { .. }
                | BluetoothConnectionState::Disconnecting { .. }
                | BluetoothConnectionState::Disconnected => {}
            }
        }

        let control = QuickSettingsControlState {
            is_hovered: pointer.is_hovering(),
            is_focused: focusable.is_focused(),
        };

        SubmenuButton {
            tag: self.bluetooth_device.tag.clone(),
            label: self.bluetooth_device.display_name.clone(),
            label_color: Some(submenu_device_label_color(availability)),
            control,
            surface: SubmenuButtonSurface::Standard,
            state: button_state_for_device(self.is_bluetooth_enabled),
            leading: submenu_button_leading_slot(
                Element::new()
                    .with_style(submenu_device_icon_ring_style(availability, ctx))
                    .with_content(glyph_element(
                        glyph_for_device(&self.bluetooth_device),
                        SETTINGS_ICON_SIZE,
                        SETTINGS_ICON_FRAME_SIZE,
                        submenu_device_icon_color(availability),
                    )),
            ),
            trailing: None,
        }
        .to_element(ctx)
    }
}

fn device_section(
    title: &'static str,
    devices: &BluetoothDeviceSection,
    is_bluetooth_enabled: bool,
    empty_label: &'static str,
) -> Element {
    let mut section = Element::new()
        .with_style(submenu_section_style())
        .with_content(
            Element::new()
                .with_style(submenu_section_label_style())
                .with_content(Text::new(title).with_style(submenu_section_title_style())),
        );

    match devices {
        BluetoothDeviceSection::Loading => {
            section.add_content(BluetoothPlaceholderRow {
                tag: match title {
                    "Recent" => "header-settings-bluetooth-recent-loading",
                    "Nearby" => "header-settings-bluetooth-nearby-loading",
                    _ => "header-settings-bluetooth-loading",
                },
                label: "Loading devices...",
            });
        }
        BluetoothDeviceSection::Unavailable => {
            section.add_content(BluetoothPlaceholderRow {
                tag: match title {
                    "Recent" => "header-settings-bluetooth-recent-unavailable",
                    "Nearby" => "header-settings-bluetooth-nearby-unavailable",
                    _ => "header-settings-bluetooth-unavailable",
                },
                label: "Bluetooth unavailable",
            });
        }
        BluetoothDeviceSection::Ready(devices) if devices.is_empty() => {
            section.add_content(BluetoothPlaceholderRow {
                tag: match title {
                    "Recent" => "header-settings-bluetooth-recent-empty",
                    "Nearby" => "header-settings-bluetooth-nearby-empty",
                    _ => "header-settings-bluetooth-empty",
                },
                label: empty_label,
            });
        }
        BluetoothDeviceSection::Ready(devices) => {
            for device in devices {
                section.add_content(BluetoothDeviceRow {
                    bluetooth_device: device.clone(),
                    is_bluetooth_enabled,
                });
            }
        }
    }

    section
}

#[derive(Clone, Copy)]
struct BluetoothPlaceholderRow {
    tag: &'static str,
    label: &'static str,
}

impl Component for BluetoothPlaceholderRow {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        SubmenuButton {
            tag: self.tag.to_string(),
            label: self.label.to_string(),
            label_color: None,
            control: QuickSettingsControlState {
                is_hovered: false,
                is_focused: false,
            },
            surface: SubmenuButtonSurface::Standard,
            state: SubmenuButtonState::Disabled,
            leading: submenu_button_surface_glyph(
                QuickSettingsGlyph::Asset(BLUETOOTH_ICON),
                SubmenuButtonSurface::Standard,
                SubmenuButtonState::Disabled,
            ),
            trailing: None,
        }
        .to_element(ctx)
    }
}

fn effective_device_availability(
    bluetooth_device: &BluetoothDeviceState,
    is_bluetooth_enabled: bool,
) -> DeviceRowAvailability {
    if !is_bluetooth_enabled {
        DeviceRowAvailability::Unavailable
    } else {
        match bluetooth_device.connection_state {
            BluetoothConnectionState::Connected => DeviceRowAvailability::Connected,
            BluetoothConnectionState::Connecting { .. } => DeviceRowAvailability::Connecting,
            BluetoothConnectionState::Disconnecting { .. } => DeviceRowAvailability::Disconnecting,
            BluetoothConnectionState::Disconnected => DeviceRowAvailability::Available,
        }
    }
}

fn button_state_for_device(is_bluetooth_enabled: bool) -> SubmenuButtonState {
    if is_bluetooth_enabled {
        SubmenuButtonState::Enabled
    } else {
        SubmenuButtonState::Disabled
    }
}

fn glyph_for_device(device: &BluetoothDeviceState) -> QuickSettingsGlyph {
    match device.category {
        BluetoothDeviceCategory::Audio => QuickSettingsGlyph::Asset(AUDIO_FILE_ICON),
        BluetoothDeviceCategory::CarAudio => QuickSettingsGlyph::Asset(AUDIO_ICON),
        BluetoothDeviceCategory::Camera => QuickSettingsGlyph::Asset(CAMERA_ICON),
        BluetoothDeviceCategory::Computer => QuickSettingsGlyph::Asset(COMPUTER_ICON),
        BluetoothDeviceCategory::Display => QuickSettingsGlyph::Asset(DISPLAY_ICON),
        BluetoothDeviceCategory::GameController => QuickSettingsGlyph::Asset(GAME_CONTROLLER_ICON),
        BluetoothDeviceCategory::Health => QuickSettingsGlyph::Asset(HEALTH_ICON),
        BluetoothDeviceCategory::Headphones => QuickSettingsGlyph::Asset(HEADPHONES_ICON),
        BluetoothDeviceCategory::Headset => QuickSettingsGlyph::Asset(HEADSET_ICON),
        BluetoothDeviceCategory::Input => QuickSettingsGlyph::Asset(KEYBOARD_ICON),
        BluetoothDeviceCategory::Keyboard => QuickSettingsGlyph::Asset(KEYBOARD_ICON),
        BluetoothDeviceCategory::MediaPlayer => QuickSettingsGlyph::Asset(MEDIA_PLAYER_ICON),
        BluetoothDeviceCategory::Microphone => QuickSettingsGlyph::Asset(MICROPHONE_ICON),
        BluetoothDeviceCategory::Mouse => QuickSettingsGlyph::Asset(MOUSE_ICON),
        BluetoothDeviceCategory::Network => QuickSettingsGlyph::Asset(NETWORK_ICON),
        BluetoothDeviceCategory::Phone => QuickSettingsGlyph::Asset(PHONE_ICON),
        BluetoothDeviceCategory::Printer => QuickSettingsGlyph::Asset(PRINTER_ICON),
        BluetoothDeviceCategory::Sensor => QuickSettingsGlyph::Asset(SENSOR_ICON),
        BluetoothDeviceCategory::Speaker => QuickSettingsGlyph::Asset(AUDIO_ICON),
        BluetoothDeviceCategory::Tablet => QuickSettingsGlyph::Asset(TABLET_ICON),
        BluetoothDeviceCategory::Peripheral
        | BluetoothDeviceCategory::Scanner
        | BluetoothDeviceCategory::Wearable
        | BluetoothDeviceCategory::Unknown => QuickSettingsGlyph::Asset(BLUETOOTH_ICON),
    }
}
