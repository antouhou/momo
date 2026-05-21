mod style;

use self::style::{
    settings_tile_button_style, settings_tile_content_style, settings_tile_icon_style,
    settings_tile_text_column_style, tile_title_style,
};
use super::common::{QuickSettingsGlyph, control_state, glyph_element};
use daiko::Element;
use daiko::component::{Component, ComponentContext};
use daiko::style::Color;
use daiko::widgets::text::Text;

const CLOUD_ICON: &[u8] = include_bytes!("../../../../assets/cloud.svg");
const MESSAGE_ICON: &[u8] = include_bytes!("../../../../assets/message.svg");
const MOON_ICON: &[u8] = include_bytes!("../../../../assets/moon.svg");
const CIRCLE_DOT_ICON: &[u8] = include_bytes!("../../../../assets/circle-dot.svg");
const KEYBOARD_ICON: &[u8] = include_bytes!("../../../../assets/keyboard.svg");
const MAP_ICON: &[u8] = include_bytes!("../../../../assets/map.svg");
const CHECK_ICON: &[u8] = include_bytes!("../../../../assets/circle-check.svg");
const EYE_ICON: &[u8] = include_bytes!("../../../../assets/eye.svg");

pub(super) const TILE_ROWS: [[SettingsTileSpec; 2]; 4] = [
    [
        SettingsTileSpec {
            label: "Network",
            glyph: QuickSettingsGlyph::Asset(CLOUD_ICON),
            is_active: true,
        },
        SettingsTileSpec {
            label: "Bluetooth",
            glyph: QuickSettingsGlyph::Asset(MESSAGE_ICON),
            is_active: false,
        },
    ],
    [
        SettingsTileSpec {
            label: "Focus mode",
            glyph: QuickSettingsGlyph::Asset(EYE_ICON),
            is_active: false,
        },
        SettingsTileSpec {
            label: "Night mode",
            glyph: QuickSettingsGlyph::Asset(MOON_ICON),
            is_active: false,
        },
    ],
    [
        SettingsTileSpec {
            label: "Dark style",
            glyph: QuickSettingsGlyph::Asset(CIRCLE_DOT_ICON),
            is_active: true,
        },
        SettingsTileSpec {
            label: "Keyboard",
            glyph: QuickSettingsGlyph::Asset(KEYBOARD_ICON),
            is_active: true,
        },
    ],
    [
        SettingsTileSpec {
            label: "Security",
            glyph: QuickSettingsGlyph::Asset(CHECK_ICON),
            is_active: false,
        },
        SettingsTileSpec {
            label: "Travel mode",
            glyph: QuickSettingsGlyph::Asset(MAP_ICON),
            is_active: false,
        },
    ],
];

#[derive(Clone, Copy)]
pub(super) struct SettingsTileSpec {
    pub(super) label: &'static str,
    pub(super) glyph: QuickSettingsGlyph,
    pub(super) is_active: bool,
}

#[derive(Clone, Copy)]
pub(super) struct SettingsTileButton {
    pub(super) spec: SettingsTileSpec,
}

impl Component for SettingsTileButton {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let state = control_state(ctx);

        Element::new()
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
                    16,
                    20.0,
                    if spec.is_active {
                        Color::from_rgb(246, 237, 255)
                    } else {
                        Color::from_rgb(232, 238, 247)
                    },
                )),
        )
        .with_content(
            Element::new()
                .with_style(settings_tile_text_column_style())
                .with_content(Text::new(spec.label).with_style(tile_title_style(spec.is_active))),
        )
}
