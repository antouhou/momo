use super::{
    common::{QuickSettingsGlyph, is_menu_view_active},
    state::{SETTINGS_MENU_STATE_ID, SettingsMenuState, SettingsMenuViewType},
};
use daiko::{
    Element, Id,
    channel::Channel,
    component::{Component, ComponentContext},
    navigation::FocusOrigin,
};
use momo_kit::{
    assets::POWER_ICON,
    components::{RoundIconButton, RoundIconButtonVariant},
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
    should_request_focus: bool,
    should_complete_focus_handoff_when_focused: bool,
    button: RoundIconButton,
    focused_channel: Channel<()>,
}

impl QuickActionButton {
    pub(super) fn new(ctx: &mut ComponentContext, spec: QuickActionSpec) -> Self {
        let mut button =
            RoundIconButton::new(ctx, spec.glyph.svg()).with_variant(if spec.is_danger {
                RoundIconButtonVariant::Danger
            } else if spec.is_active {
                RoundIconButtonVariant::Accent
            } else {
                RoundIconButtonVariant::Standard
            });
        if let Some(tag) = spec.tag {
            button = button.with_tag(tag);
        }

        Self {
            should_request_focus: false,
            should_complete_focus_handoff_when_focused: false,
            button,
            focused_channel: ctx.create_channel(),
        }
    }

    pub(super) fn request_focus(&mut self) {
        self.should_request_focus = true;
    }

    pub(super) fn complete_focus_handoff_when_focused(&mut self) {
        self.should_complete_focus_handoff_when_focused = true;
    }

    pub(super) fn activated(&self) -> bool {
        self.button.activated()
    }
}

impl Component for QuickActionButton {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let is_enabled = is_menu_view_active(ctx, SettingsMenuViewType::Main);
        if self.should_complete_focus_handoff_when_focused
            && self.focused_channel.iter().next().is_some()
        {
            let state =
                ctx.use_shared_state(Id::new(SETTINGS_MENU_STATE_ID), SettingsMenuState::default);
            state.write_silent().complete_view_focus_handoff();
        }

        let button = self
            .button
            .clone()
            .with_focused_channel(self.focused_channel.clone())
            .with_enabled(is_enabled)
            .with_requested_focus(
                self.should_request_focus
                    .then_some(FocusOrigin::Programmatic),
            );
        button.to_element(ctx)
    }
}
