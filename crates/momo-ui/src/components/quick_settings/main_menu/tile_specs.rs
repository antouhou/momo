use super::super::common::QuickSettingsGlyph;
use super::super::settings_tile_button::SettingsTileSpec;

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
pub(super) const BLUETOOTH_TILE_FOCUS_KEY_ID: &str = "header-settings-tile-bluetooth-focus";

pub(super) const TILE_ROWS: [[SettingsTileSpec; 2]; 4] = [
    [
        SettingsTileSpec {
            tag: NETWORK_TILE_TAG,
            focus_key_id: "header-settings-tile-network-focus",
            label: "Network",
            glyph: QuickSettingsGlyph::Asset(CLOUD_ICON),
            is_active: true,
            is_preferred_focus: true,
        },
        SettingsTileSpec {
            tag: BLUETOOTH_TILE_TAG,
            focus_key_id: BLUETOOTH_TILE_FOCUS_KEY_ID,
            label: "Bluetooth",
            glyph: QuickSettingsGlyph::Asset(BLUETOOTH_ICON),
            is_active: false,
            is_preferred_focus: false,
        },
    ],
    [
        SettingsTileSpec {
            tag: "header-settings-tile-focus-mode",
            focus_key_id: "header-settings-tile-focus-mode-focus",
            label: "Focus mode",
            glyph: QuickSettingsGlyph::Asset(EYE_ICON),
            is_active: false,
            is_preferred_focus: false,
        },
        SettingsTileSpec {
            tag: "header-settings-tile-night-mode",
            focus_key_id: "header-settings-tile-night-mode-focus",
            label: "Night mode",
            glyph: QuickSettingsGlyph::Asset(MOON_ICON),
            is_active: false,
            is_preferred_focus: false,
        },
    ],
    [
        SettingsTileSpec {
            tag: "header-settings-tile-dark-style",
            focus_key_id: "header-settings-tile-dark-style-focus",
            label: "Dark style",
            glyph: QuickSettingsGlyph::Asset(CIRCLE_DOT_ICON),
            is_active: true,
            is_preferred_focus: false,
        },
        SettingsTileSpec {
            tag: "header-settings-tile-keyboard",
            focus_key_id: "header-settings-tile-keyboard-focus",
            label: "Keyboard",
            glyph: QuickSettingsGlyph::Asset(KEYBOARD_ICON),
            is_active: true,
            is_preferred_focus: false,
        },
    ],
    [
        SettingsTileSpec {
            tag: "header-settings-tile-security",
            focus_key_id: "header-settings-tile-security-focus",
            label: "Security",
            glyph: QuickSettingsGlyph::Asset(CHECK_ICON),
            is_active: false,
            is_preferred_focus: false,
        },
        SettingsTileSpec {
            tag: "header-settings-tile-travel-mode",
            focus_key_id: "header-settings-tile-travel-mode-focus",
            label: "Travel mode",
            glyph: QuickSettingsGlyph::Asset(MAP_ICON),
            is_active: false,
            is_preferred_focus: false,
        },
    ],
];
