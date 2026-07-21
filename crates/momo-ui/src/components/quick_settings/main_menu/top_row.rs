use super::{
    super::{
        common::{QuickSettingsGlyph, is_menu_view_active},
        state::{SETTINGS_MENU_STATE_ID, SettingsMenuState, SettingsMenuViewType},
        status_chip::StatusChip,
    },
    style::{settings_top_actions_style, settings_top_row_style},
};
use daiko::{
    Element, Id,
    component::{Component, ComponentContext},
    navigation::{FocusEntryPolicy, FocusOrigin},
};
use momo_kit::{
    assets::POWER_ICON,
    components::{RoundIconButton, RoundIconButtonVariant},
};

const MOON_ICON: &[u8] = include_bytes!("../../../../assets/moon.svg");
const GEAR_ICON: &[u8] = include_bytes!("../../../../assets/gear-solid-full.svg");
const EYE_ICON: &[u8] = include_bytes!("../../../../assets/eye.svg");
const POWER_ACTION_TAG: &str = "header-settings-power-button";
const QUICK_ACTIONS: [QuickActionSpec; 4] = [
    QuickActionSpec {
        tag: None,
        glyph: QuickSettingsGlyph::Asset(EYE_ICON),
        variant: RoundIconButtonVariant::Accent,
    },
    QuickActionSpec {
        tag: None,
        glyph: QuickSettingsGlyph::Asset(MOON_ICON),
        variant: RoundIconButtonVariant::Standard,
    },
    QuickActionSpec {
        tag: None,
        glyph: QuickSettingsGlyph::Asset(GEAR_ICON),
        variant: RoundIconButtonVariant::Standard,
    },
    QuickActionSpec {
        tag: Some(POWER_ACTION_TAG),
        glyph: QuickSettingsGlyph::Asset(POWER_ICON),
        variant: RoundIconButtonVariant::Danger,
    },
];

#[derive(Clone, Copy)]
struct QuickActionSpec {
    tag: Option<&'static str>,
    glyph: QuickSettingsGlyph,
    variant: RoundIconButtonVariant,
}

impl QuickActionSpec {
    fn is_power_action(self) -> bool {
        self.tag == Some(POWER_ACTION_TAG)
    }
}

#[derive(Clone, Copy)]
pub(super) struct SettingsTopRow;

impl Component for SettingsTopRow {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        ctx.focus_scope()
            .set_entry_policy(FocusEntryPolicy::Remembered);

        Element::new()
            .with_style(settings_top_row_style())
            .with_content(StatusChip)
            .with_content(top_actions(ctx))
    }
}

fn top_actions(ctx: &mut ComponentContext) -> Element {
    let should_restore_power_focus = should_restore_power_button_focus(ctx);
    let is_enabled = is_menu_view_active(ctx, SettingsMenuViewType::Main);
    let mut actions = Element::new().with_style(settings_top_actions_style());

    for spec in QUICK_ACTIONS {
        let should_restore_focus = should_restore_power_focus && spec.is_power_action();
        let mut button = RoundIconButton::new(ctx, spec.glyph.svg())
            .with_variant(spec.variant)
            .with_enabled(is_enabled)
            .with_requested_focus(should_restore_focus.then_some(FocusOrigin::Programmatic));
        if let Some(tag) = spec.tag {
            button = button.with_tag(tag);
        }
        handle_quick_action_activation(ctx, spec, button.activated());
        if should_restore_focus && button.has_been_focused() {
            complete_power_focus_handoff(ctx);
        }
        actions.add_content(button);
    }

    actions
}

fn handle_quick_action_activation(
    ctx: &mut ComponentContext,
    spec: QuickActionSpec,
    was_activated: bool,
) {
    if !was_activated {
        return;
    }

    if spec.is_power_action() {
        let state =
            ctx.use_shared_state(Id::new(SETTINGS_MENU_STATE_ID), SettingsMenuState::default);
        if state.read().active_view == SettingsMenuViewType::Main {
            state.write().set_active_view(SettingsMenuViewType::Power);
        }
    }
}

fn complete_power_focus_handoff(ctx: &mut ComponentContext) {
    let state = ctx.use_shared_state(Id::new(SETTINGS_MENU_STATE_ID), SettingsMenuState::default);
    state.write_silent().complete_view_focus_handoff();
}

fn should_restore_power_button_focus(ctx: &mut ComponentContext) -> bool {
    let state = ctx.use_shared_state(Id::new(SETTINGS_MENU_STATE_ID), SettingsMenuState::default);
    {
        let state = state.read();
        state.last_active_view == SettingsMenuViewType::Power
            && state.active_view == SettingsMenuViewType::Main
    }
}
