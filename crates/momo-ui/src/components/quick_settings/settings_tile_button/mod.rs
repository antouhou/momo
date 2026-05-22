mod style;

use self::style::{
    settings_tile_button_style, settings_tile_content_style, settings_tile_icon_style,
    settings_tile_text_column_style, tile_title_style,
};
use super::common::{QuickSettingsGlyph, control_state, glyph_element};
use super::style::{SETTINGS_ICON_FRAME_SIZE, SETTINGS_ICON_SIZE, settings_tile_icon_color};
use daiko::Element;
use daiko::component::{Component, ComponentContext};
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

pub(super) const TILE_ROWS: [[SettingsTileSpec; 2]; 4] = [
    [
        SettingsTileSpec {
            tag: NETWORK_TILE_TAG,
            label: "Network",
            glyph: QuickSettingsGlyph::Asset(CLOUD_ICON),
            is_active: true,
            is_preferred_focus: true,
        },
        SettingsTileSpec {
            tag: "header-settings-tile-bluetooth",
            label: "Bluetooth",
            glyph: QuickSettingsGlyph::Asset(BLUETOOTH_ICON),
            is_active: false,
            is_preferred_focus: false,
        },
    ],
    [
        SettingsTileSpec {
            tag: "header-settings-tile-focus-mode",
            label: "Focus mode",
            glyph: QuickSettingsGlyph::Asset(EYE_ICON),
            is_active: false,
            is_preferred_focus: false,
        },
        SettingsTileSpec {
            tag: "header-settings-tile-night-mode",
            label: "Night mode",
            glyph: QuickSettingsGlyph::Asset(MOON_ICON),
            is_active: false,
            is_preferred_focus: false,
        },
    ],
    [
        SettingsTileSpec {
            tag: "header-settings-tile-dark-style",
            label: "Dark style",
            glyph: QuickSettingsGlyph::Asset(CIRCLE_DOT_ICON),
            is_active: true,
            is_preferred_focus: false,
        },
        SettingsTileSpec {
            tag: "header-settings-tile-keyboard",
            label: "Keyboard",
            glyph: QuickSettingsGlyph::Asset(KEYBOARD_ICON),
            is_active: true,
            is_preferred_focus: false,
        },
    ],
    [
        SettingsTileSpec {
            tag: "header-settings-tile-security",
            label: "Security",
            glyph: QuickSettingsGlyph::Asset(CHECK_ICON),
            is_active: false,
            is_preferred_focus: false,
        },
        SettingsTileSpec {
            tag: "header-settings-tile-travel-mode",
            label: "Travel mode",
            glyph: QuickSettingsGlyph::Asset(MAP_ICON),
            is_active: false,
            is_preferred_focus: false,
        },
    ],
];

#[derive(Clone, Copy)]
pub(super) struct SettingsTileSpec {
    pub(super) tag: &'static str,
    pub(super) label: &'static str,
    pub(super) glyph: QuickSettingsGlyph,
    pub(super) is_active: bool,
    pub(super) is_preferred_focus: bool,
}

#[derive(Clone, Copy)]
pub(super) struct SettingsTileButton {
    pub(super) spec: SettingsTileSpec,
}

impl Component for SettingsTileButton {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        ctx.focusable()
            .set_preferred_focus(self.spec.is_preferred_focus);
        let state = control_state(ctx);

        Element::new()
            .with_tag(self.spec.tag)
            .with_style(settings_tile_button_style(state, ctx, self.spec.is_active))
            .with_content(settings_tile_content(self.spec))
    }
}

fn settings_tile_content(spec: SettingsTileSpec) -> Element {
    Element::new()
        .with_style(settings_tile_content_style())
        .with_content(
            Element::new()
                .with_style(settings_tile_icon_style(spec.is_active))
                .with_content(glyph_element(
                    spec.glyph,
                    SETTINGS_ICON_SIZE,
                    SETTINGS_ICON_FRAME_SIZE,
                    settings_tile_icon_color(spec.is_active),
                )),
        )
        .with_content(
            Element::new()
                .with_style(settings_tile_text_column_style())
                .with_content(Text::new(spec.label).with_style(tile_title_style(spec.is_active))),
        )
}
