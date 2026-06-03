mod style;

use self::style::settings_round_button_style;
use super::common::{
    QuickSettingsControlState, QuickSettingsGlyph, control_state, glyph_element,
    is_menu_view_active,
};
use super::state::SettingsMenuViewType;
use super::style::{
    SETTINGS_ICON_FRAME_SIZE, SETTINGS_ICON_SIZE, settings_danger_text_color,
    settings_inverse_text_color, settings_text_color,
};
use daiko::Element;
use daiko::component::{Component, ComponentContext};

const MOON_ICON: &[u8] = include_bytes!("../../../../assets/moon.svg");
const GEAR_ICON: &[u8] = include_bytes!("../../../../assets/gear-solid-full.svg");
const EYE_ICON: &[u8] = include_bytes!("../../../../assets/eye.svg");
const POWER_ICON: &[u8] = include_bytes!("../../../../assets/power.svg");

pub(super) const FOCUS_ACTION: QuickActionSpec = QuickActionSpec {
    tag: None,
    glyph: QuickSettingsGlyph::Asset(EYE_ICON),
    is_active: true,
    is_danger: false,
};
pub(super) const NIGHT_ACTION: QuickActionSpec = QuickActionSpec {
    tag: None,
    glyph: QuickSettingsGlyph::Asset(MOON_ICON),
    is_active: false,
    is_danger: false,
};
pub(super) const TOOLS_ACTION: QuickActionSpec = QuickActionSpec {
    tag: None,
    glyph: QuickSettingsGlyph::Asset(GEAR_ICON),
    is_active: false,
    is_danger: false,
};
pub(super) const EXIT_ACTION: QuickActionSpec = QuickActionSpec {
    tag: Some("header-settings-exit-button"),
    glyph: QuickSettingsGlyph::Asset(POWER_ICON),
    is_active: false,
    is_danger: true,
};

#[derive(Clone, Copy)]
pub(super) struct QuickActionSpec {
    pub(super) tag: Option<&'static str>,
    pub(super) glyph: QuickSettingsGlyph,
    pub(super) is_active: bool,
    pub(super) is_danger: bool,
}

#[derive(Clone, Copy)]
pub(super) struct QuickActionButton {
    pub(super) spec: QuickActionSpec,
}

impl Component for QuickActionButton {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        ctx.focusable()
            .set_navigation_enabled(is_menu_view_active(ctx, SettingsMenuViewType::Main));
        let state = control_state(ctx);
        let mut element = Element::new()
            .with_style(settings_round_button_style(
                state,
                ctx,
                self.spec.is_active,
                self.spec.is_danger,
            ))
            .with_content(quick_action_content(self.spec, state));

        if let Some(tag) = self.spec.tag {
            element.set_tag(tag);
        }

        element
    }
}

fn quick_action_content(spec: QuickActionSpec, state: QuickSettingsControlState) -> Element {
    let is_highlighted = state.is_hovered || state.is_focused;

    glyph_element(
        spec.glyph,
        SETTINGS_ICON_SIZE,
        SETTINGS_ICON_FRAME_SIZE,
        if spec.is_danger {
            settings_danger_text_color()
        } else if is_highlighted {
            settings_inverse_text_color()
        } else {
            settings_text_color()
        },
    )
}
