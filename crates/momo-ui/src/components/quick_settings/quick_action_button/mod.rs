mod style;

use daiko::{
    Element, Id,
    channel::Channel,
    component::{Component, ComponentContext},
    navigation::FocusOrigin,
};
use momo_kit::{assets::POWER_ICON, interaction::ButtonBehavior};
use self::style::settings_round_button_style;
use super::{
    common::{QuickSettingsControlState, QuickSettingsGlyph, glyph_element, is_menu_view_active},
    state::{SETTINGS_MENU_STATE_ID, SettingsMenuState, SettingsMenuViewType},
    style::{
        SETTINGS_ICON_FRAME_SIZE, SETTINGS_ICON_SIZE, settings_danger_text_color,
        settings_inverse_text_color, settings_text_color,
    },
};

const MOON_ICON: &[u8] = include_bytes!("../../../../assets/moon.svg");
const GEAR_ICON: &[u8] = include_bytes!("../../../../assets/gear-solid-full.svg");
const EYE_ICON: &[u8] = include_bytes!("../../../../assets/eye.svg");
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
pub(super) const POWER_ACTION: QuickActionSpec = QuickActionSpec {
    tag: Some(POWER_ACTION_TAG),
    glyph: QuickSettingsGlyph::Asset(POWER_ICON),
    is_active: false,
    is_danger: true,
};

pub(super) const POWER_ACTION_TAG: &str = "header-settings-power-button";

#[derive(Clone, Copy)]
pub(super) struct QuickActionSpec {
    pub(super) tag: Option<&'static str>,
    pub(super) glyph: QuickSettingsGlyph,
    pub(super) is_active: bool,
    pub(super) is_danger: bool,
}

pub(super) struct QuickActionButton {
    pub(super) spec: QuickActionSpec,
    should_request_focus: bool,
    should_complete_focus_handoff_when_focused: bool,
    activation_channel: Option<Channel<()>>,
}

impl QuickActionButton {
    pub(super) fn new(ctx: &mut ComponentContext, spec: QuickActionSpec) -> Self {
        Self {
            spec,
            should_request_focus: false,
            should_complete_focus_handoff_when_focused: false,
            activation_channel: Some(ctx.create_channel()),
        }
    }

    pub(super) fn request_focus(&mut self) {
        self.should_request_focus = true;
    }

    pub(super) fn complete_focus_handoff_when_focused(&mut self) {
        self.should_complete_focus_handoff_when_focused = true;
    }

    pub(super) fn activated(&self) -> bool {
        match &self.activation_channel {
            Some(channel) => channel.iter().count() > 0,
            None => false,
        }
    }
}

impl Component for QuickActionButton {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let is_enabled = is_menu_view_active(ctx, SettingsMenuViewType::Main);
        let button = ButtonBehavior::new(ctx)
            .with_enabled(is_enabled)
            .with_requested_focus(
                self.should_request_focus
                    .then_some(FocusOrigin::Programmatic),
            )
            .apply();

        if self.should_complete_focus_handoff_when_focused && button.is_focused {
            let state =
                ctx.use_shared_state(Id::new(SETTINGS_MENU_STATE_ID), SettingsMenuState::default);
            state.write_silent().complete_view_focus_handoff();
        }

        let state = QuickSettingsControlState {
            is_hovered: button.is_hovering,
            is_focused: button.is_focused,
        };

        if button.just_activated
            && let Some(channel) = &self.activation_channel
        {
            let _ = channel.send(());
        }

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
