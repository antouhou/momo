use daiko::{
    Element, Id,
    component::{Component, ComponentContext},
    navigation::FocusEntryPolicy,
};
use super::{
    super::{
        quick_action_button::{
            FOCUS_ACTION, NIGHT_ACTION, POWER_ACTION, POWER_ACTION_TAG, QuickActionButton,
            QuickActionSpec, TOOLS_ACTION,
        },
        state::{SETTINGS_MENU_STATE_ID, SettingsMenuState, SettingsMenuViewType},
        status_chip::StatusChip,
    },
    style::{settings_top_actions_style, settings_top_row_style},
};

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
    let mut actions = Element::new().with_style(settings_top_actions_style());

    for spec in [FOCUS_ACTION, NIGHT_ACTION, TOOLS_ACTION, POWER_ACTION] {
        let mut button = QuickActionButton::new(ctx, spec);
        if should_restore_power_focus && spec.tag == Some(POWER_ACTION_TAG) {
            button.request_focus();
            button.complete_focus_handoff_when_focused();
        }
        handle_quick_action_activation(ctx, spec, button.activated());
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

    if spec.tag == Some(POWER_ACTION_TAG) {
        let state =
            ctx.use_shared_state(Id::new(SETTINGS_MENU_STATE_ID), SettingsMenuState::default);
        if state.read().active_view == SettingsMenuViewType::Main {
            state.write().set_active_view(SettingsMenuViewType::Power);
        }
    }
}

fn should_restore_power_button_focus(ctx: &mut ComponentContext) -> bool {
    let state = ctx.use_shared_state(Id::new(SETTINGS_MENU_STATE_ID), SettingsMenuState::default);
    {
        let state = state.read();
        state.last_active_view == SettingsMenuViewType::Power
            && state.active_view == SettingsMenuViewType::Main
    }
}
