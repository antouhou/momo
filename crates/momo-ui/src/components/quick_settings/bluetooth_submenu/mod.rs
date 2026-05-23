mod style;

use self::style::{
    DeviceRowAvailability, bluetooth_submenu_body_style, bluetooth_submenu_style,
    submenu_device_icon_color, submenu_device_icon_ring_style, submenu_section_label_style,
    submenu_section_style, submenu_section_title_style,
};
use super::common::{QuickSettingsControlState, QuickSettingsGlyph, glyph_element};
use super::state::{SETTINGS_MENU_STATE_ID, SettingsMenuState, SettingsMenuView};
use super::style::{SETTINGS_ICON_FRAME_SIZE, SETTINGS_ICON_SIZE, settings_text_color};
use super::submenu_button::{
    SubmenuButton, SubmenuButtonState, SubmenuButtonSurface, submenu_button_glyph,
    submenu_button_leading_slot, submenu_button_surface_glyph, submenu_toggle_switch,
};
use daiko::component::{Component, ComponentContext};
use daiko::navigation::{FocusEntryPolicy, FocusOrigin};
use daiko::widgets::text::Text;
use daiko::{Element, Id};

const BLUETOOTH_ICON: &[u8] = include_bytes!("../../../../assets/bluetooth-b.svg");
const CHEVRON_LEFT_ICON: &[u8] = include_bytes!("../../../../assets/chevron-left.svg");
const KEYBOARD_ICON: &[u8] = include_bytes!("../../../../assets/keyboard.svg");
const AUDIO_ICON: &[u8] = include_bytes!("../../../../assets/volume.svg");
const GEAR_ICON: &[u8] = include_bytes!("../../../../assets/gear-solid-full.svg");

pub(super) const BLUETOOTH_BACK_BUTTON_TAG: &str = "header-settings-bluetooth-back-button";
pub(super) const BLUETOOTH_TOGGLE_TAG: &str = "header-settings-bluetooth-toggle";
pub(super) const BLUETOOTH_SETTINGS_BUTTON_TAG: &str = "header-settings-bluetooth-settings-button";

#[derive(Clone, Copy)]
pub(super) struct BluetoothSubmenu;

impl Component for BluetoothSubmenu {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        ctx.focus_scope()
            .set_entry_policy(FocusEntryPolicy::Remembered);

        Element::new()
            .with_tag("header-settings-bluetooth-submenu")
            .with_style(bluetooth_submenu_style())
            .with_content(BluetoothBackButton)
            .with_content(BluetoothSubmenuBody)
    }
}

#[derive(Clone, Copy)]
struct BluetoothSubmenuBody;

impl Component for BluetoothSubmenuBody {
    fn to_element(&self, _ctx: &mut ComponentContext) -> Element {
        Element::new()
            .with_style(bluetooth_submenu_body_style())
            .with_content(BluetoothToggleRow)
            .with_content(device_section(
                "Recent",
                &[
                    DeviceRowSpec {
                        tag: "header-settings-bluetooth-device-pods",
                        label: "Pods",
                        glyph: QuickSettingsGlyph::Asset(BLUETOOTH_ICON),
                        availability: DeviceRowAvailability::Connected,
                    },
                    DeviceRowSpec {
                        tag: "header-settings-bluetooth-device-keyboard",
                        label: "Keyboard",
                        glyph: QuickSettingsGlyph::Asset(KEYBOARD_ICON),
                        availability: DeviceRowAvailability::Available,
                    },
                ],
            ))
            .with_content(device_section(
                "Nearby",
                &[
                    DeviceRowSpec {
                        tag: "header-settings-bluetooth-nearby-speaker",
                        label: "Speaker",
                        glyph: QuickSettingsGlyph::Asset(AUDIO_ICON),
                        availability: DeviceRowAvailability::Available,
                    },
                    DeviceRowSpec {
                        tag: "header-settings-bluetooth-nearby-keyboard",
                        label: "Keyboard",
                        glyph: QuickSettingsGlyph::Asset(KEYBOARD_ICON),
                        availability: DeviceRowAvailability::Unavailable,
                    },
                ],
            ))
            .with_content(BluetoothSettingsButton)
    }
}

#[derive(Clone, Copy)]
struct BluetoothBackButton;

impl Component for BluetoothBackButton {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let mut pointer = ctx.pointer();
        let focusable = ctx.focusable();
        let state =
            ctx.use_shared_state(Id::new(SETTINGS_MENU_STATE_ID), SettingsMenuState::default);
        let snapshot = *state.read();
        let is_active = snapshot.active_view == SettingsMenuView::Bluetooth;

        if pointer.just_entered() || pointer.just_pressed() {
            focusable.request_focus(FocusOrigin::Pointer);
        }

        if snapshot.last_active_view == SettingsMenuView::Main
            && snapshot.active_view == SettingsMenuView::Bluetooth
        {
            focusable.request_focus(FocusOrigin::Programmatic);
            let mut menu_state = state.write_silent();
            menu_state.last_active_view = menu_state.active_view;
        }

        if is_active && (pointer.just_pressed() || focusable.just_activated()) {
            *state.write() = SettingsMenuState {
                last_active_view: snapshot.active_view,
                active_view: SettingsMenuView::Main,
                ..snapshot
            };
        }

        let control = QuickSettingsControlState {
            is_hovered: pointer.is_hovering(),
            is_focused: focusable.is_focused(),
        };

        SubmenuButton {
            tag: BLUETOOTH_BACK_BUTTON_TAG,
            label: "Bluetooth",
            control,
            surface: SubmenuButtonSurface::Standard,
            state: SubmenuButtonState::Enabled,
            leading: submenu_button_glyph(
                QuickSettingsGlyph::Asset(CHEVRON_LEFT_ICON),
                settings_text_color(),
            ),
            trailing: None,
        }
        .to_element(ctx)
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

        if pointer.just_entered() || pointer.just_pressed() {
            focusable.request_focus(FocusOrigin::Pointer);
        }

        if is_active && (pointer.just_pressed() || focusable.just_activated()) {
            *state.write() = SettingsMenuState {
                bluetooth_enabled: !snapshot.bluetooth_enabled,
                ..snapshot
            };
        }

        let control = QuickSettingsControlState {
            is_hovered: pointer.is_hovering(),
            is_focused: focusable.is_focused(),
        };

        SubmenuButton {
            tag: BLUETOOTH_TOGGLE_TAG,
            label: "Bluetooth",
            control,
            surface: SubmenuButtonSurface::Standard,
            state: SubmenuButtonState::Enabled,
            leading: submenu_button_glyph(
                QuickSettingsGlyph::Asset(BLUETOOTH_ICON),
                settings_text_color(),
            ),
            trailing: Some(submenu_toggle_switch(ctx, snapshot.bluetooth_enabled)),
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

        if pointer.just_entered() || pointer.just_pressed() {
            focusable.request_focus(FocusOrigin::Pointer);
        }

        let control = QuickSettingsControlState {
            is_hovered: pointer.is_hovering(),
            is_focused: focusable.is_focused(),
        };

        SubmenuButton {
            tag: BLUETOOTH_SETTINGS_BUTTON_TAG,
            label: "Settings",
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

#[derive(Clone, Copy)]
struct DeviceRowSpec {
    tag: &'static str,
    label: &'static str,
    glyph: QuickSettingsGlyph,
    availability: DeviceRowAvailability,
}

#[derive(Clone, Copy)]
struct BluetoothDeviceRow {
    spec: DeviceRowSpec,
}

impl Component for BluetoothDeviceRow {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let mut pointer = ctx.pointer();
        let focusable = ctx.focusable();
        let state =
            ctx.use_shared_state(Id::new(SETTINGS_MENU_STATE_ID), SettingsMenuState::default);
        let snapshot = *state.read();
        let availability = effective_device_availability(self.spec, snapshot.bluetooth_enabled);

        if pointer.just_entered() || pointer.just_pressed() {
            focusable.request_focus(FocusOrigin::Pointer);
        }

        let control = QuickSettingsControlState {
            is_hovered: pointer.is_hovering(),
            is_focused: focusable.is_focused(),
        };

        SubmenuButton {
            tag: self.spec.tag,
            label: self.spec.label,
            control,
            surface: SubmenuButtonSurface::Standard,
            state: button_state_for_device(availability),
            leading: submenu_button_leading_slot(
                Element::new()
                    .with_style(submenu_device_icon_ring_style(availability, ctx))
                    .with_content(glyph_element(
                        self.spec.glyph,
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

fn device_section(title: &'static str, rows: &[DeviceRowSpec]) -> Element {
    let mut section = Element::new()
        .with_style(submenu_section_style())
        .with_content(
            Element::new()
                .with_style(submenu_section_label_style())
                .with_content(Text::new(title).with_style(submenu_section_title_style())),
        );

    for row in rows {
        section.add_content(BluetoothDeviceRow { spec: *row });
    }

    section
}

fn effective_device_availability(
    spec: DeviceRowSpec,
    bluetooth_enabled: bool,
) -> DeviceRowAvailability {
    if bluetooth_enabled {
        spec.availability
    } else {
        DeviceRowAvailability::Unavailable
    }
}

fn button_state_for_device(availability: DeviceRowAvailability) -> SubmenuButtonState {
    match availability {
        DeviceRowAvailability::Connected | DeviceRowAvailability::Available => {
            SubmenuButtonState::Enabled
        }
        DeviceRowAvailability::Unavailable => SubmenuButtonState::Disabled,
    }
}
