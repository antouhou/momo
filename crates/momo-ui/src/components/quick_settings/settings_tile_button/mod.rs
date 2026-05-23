mod style;

use self::style::{
    settings_tile_button_style, settings_tile_content_style, settings_tile_icon_style,
    settings_tile_text_column_style, tile_title_style,
};
use super::common::{
    QuickSettingsControlState, QuickSettingsGlyph, glyph_element, is_menu_view_active,
};
use super::state::{SETTINGS_MENU_STATE_ID, SettingsMenuState, SettingsMenuView};
use super::style::{SETTINGS_ICON_FRAME_SIZE, SETTINGS_ICON_SIZE, settings_tile_icon_color};
use daiko::Element;
use daiko::Id;
use daiko::component::{Component, ComponentContext};
use daiko::navigation::FocusOrigin;
use daiko::widgets::text::Text;

const CLOUD_ICON: &[u8] = include_bytes!("../../../../assets/cloud.svg");
const BLUETOOTH_ICON: &[u8] = include_bytes!("../../../../assets/bluetooth-b.svg");
const MOON_ICON: &[u8] = include_bytes!("../../../../assets/moon.svg");
const CIRCLE_DOT_ICON: &[u8] = include_bytes!("../../../../assets/circle-dot.svg");
const KEYBOARD_ICON: &[u8] = include_bytes!("../../../../assets/keyboard.svg");
const MAP_ICON: &[u8] = include_bytes!("../../../../assets/map.svg");
const CHECK_ICON: &[u8] = include_bytes!("../../../../assets/circle-check.svg");
const EYE_ICON: &[u8] = include_bytes!("../../../../assets/eye.svg");
pub(super) const NETWORK_TILE_TAG: &str = "header-settings-tile-network";
pub(super) const BLUETOOTH_TILE_TAG: &str = "header-settings-tile-bluetooth";

pub(super) const TILE_ROWS: [[SettingsTileSpec; 2]; 4] = [
    [
        SettingsTileSpec {
            tag: NETWORK_TILE_TAG,
            label: "Network",
            glyph: QuickSettingsGlyph::Asset(CLOUD_ICON),
            is_active: true,
            is_preferred_focus: true,
            action: SettingsTileAction::None,
        },
        SettingsTileSpec {
            tag: BLUETOOTH_TILE_TAG,
            label: "Bluetooth",
            glyph: QuickSettingsGlyph::Asset(BLUETOOTH_ICON),
            is_active: false,
            is_preferred_focus: false,
            action: SettingsTileAction::OpenBluetoothSubmenu,
        },
    ],
    [
        SettingsTileSpec {
            tag: "header-settings-tile-focus-mode",
            label: "Focus mode",
            glyph: QuickSettingsGlyph::Asset(EYE_ICON),
            is_active: false,
            is_preferred_focus: false,
            action: SettingsTileAction::None,
        },
        SettingsTileSpec {
            tag: "header-settings-tile-night-mode",
            label: "Night mode",
            glyph: QuickSettingsGlyph::Asset(MOON_ICON),
            is_active: false,
            is_preferred_focus: false,
            action: SettingsTileAction::None,
        },
    ],
    [
        SettingsTileSpec {
            tag: "header-settings-tile-dark-style",
            label: "Dark style",
            glyph: QuickSettingsGlyph::Asset(CIRCLE_DOT_ICON),
            is_active: true,
            is_preferred_focus: false,
            action: SettingsTileAction::None,
        },
        SettingsTileSpec {
            tag: "header-settings-tile-keyboard",
            label: "Keyboard",
            glyph: QuickSettingsGlyph::Asset(KEYBOARD_ICON),
            is_active: true,
            is_preferred_focus: false,
            action: SettingsTileAction::None,
        },
    ],
    [
        SettingsTileSpec {
            tag: "header-settings-tile-security",
            label: "Security",
            glyph: QuickSettingsGlyph::Asset(CHECK_ICON),
            is_active: false,
            is_preferred_focus: false,
            action: SettingsTileAction::None,
        },
        SettingsTileSpec {
            tag: "header-settings-tile-travel-mode",
            label: "Travel mode",
            glyph: QuickSettingsGlyph::Asset(MAP_ICON),
            is_active: false,
            is_preferred_focus: false,
            action: SettingsTileAction::None,
        },
    ],
];

#[derive(Clone, Copy)]
pub(super) enum SettingsTileAction {
    None,
    OpenBluetoothSubmenu,
}

#[derive(Clone, Copy)]
pub(super) struct SettingsTileSpec {
    pub(super) tag: &'static str,
    pub(super) label: &'static str,
    pub(super) glyph: QuickSettingsGlyph,
    pub(super) is_active: bool,
    pub(super) is_preferred_focus: bool,
    pub(super) action: SettingsTileAction,
}

#[derive(Clone, Copy)]
pub(super) struct SettingsTileButton {
    pub(super) spec: SettingsTileSpec,
}

impl Component for SettingsTileButton {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let mut pointer = ctx.pointer();
        let focusable = ctx.focusable();
        let is_main_view = is_menu_view_active(ctx, SettingsMenuView::Main);
        let shared_state =
            ctx.use_shared_state(Id::new(SETTINGS_MENU_STATE_ID), SettingsMenuState::default);
        let menu_state = *shared_state.read();

        focusable.set_preferred_focus(self.spec.is_preferred_focus);
        focusable.set_navigation_enabled(is_main_view);

        if pointer.just_entered() || pointer.just_pressed() {
            focusable.request_focus(FocusOrigin::Pointer);
        }

        if matches!(self.spec.action, SettingsTileAction::OpenBluetoothSubmenu)
            && menu_state.last_active_view == SettingsMenuView::Bluetooth
            && menu_state.active_view == SettingsMenuView::Main
            && is_main_view
        {
            focusable.request_focus(FocusOrigin::Programmatic);
            let mut state = shared_state.write_silent();
            state.last_active_view = state.active_view;
        }

        let state = QuickSettingsControlState {
            is_hovered: pointer.is_hovering(),
            is_focused: focusable.is_focused(),
        };
        let is_active = is_tile_active(self.spec, menu_state);

        if is_main_view && (pointer.just_clicked() || focusable.just_activated()) {
            match self.spec.action {
                SettingsTileAction::OpenBluetoothSubmenu => {
                    *shared_state.write() = SettingsMenuState {
                        last_active_view: menu_state.active_view,
                        active_view: SettingsMenuView::Bluetooth,
                        ..menu_state
                    };
                }
                SettingsTileAction::None => {}
            }
        }

        Element::new()
            .with_tag(self.spec.tag)
            .with_style(settings_tile_button_style(state, ctx, is_active))
            .with_content(settings_tile_content(self.spec, is_active))
    }
}

fn settings_tile_content(spec: SettingsTileSpec, is_active: bool) -> Element {
    Element::new()
        .with_style(settings_tile_content_style())
        .with_content(
            Element::new()
                .with_style(settings_tile_icon_style(is_active))
                .with_content(glyph_element(
                    spec.glyph,
                    SETTINGS_ICON_SIZE,
                    SETTINGS_ICON_FRAME_SIZE,
                    settings_tile_icon_color(is_active),
                )),
        )
        .with_content(
            Element::new()
                .with_style(settings_tile_text_column_style())
                .with_content(Text::new(spec.label).with_style(tile_title_style(is_active))),
        )
}

fn is_tile_active(spec: SettingsTileSpec, state: SettingsMenuState) -> bool {
    match spec.action {
        SettingsTileAction::OpenBluetoothSubmenu => state.bluetooth_enabled,
        SettingsTileAction::None => spec.is_active,
    }
}
