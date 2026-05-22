use super::super::quick_action_button::{
    EXIT_ACTION, FOCUS_ACTION, NIGHT_ACTION, QuickActionButton, TOOLS_ACTION,
};
use super::super::status_chip::StatusChip;
use super::style::{settings_top_actions_style, settings_top_row_style};
use daiko::Element;
use daiko::component::{Component, ComponentContext};
use daiko::navigation::FocusEntryPolicy;

#[derive(Clone, Copy)]
pub(super) struct SettingsTopRow;

impl Component for SettingsTopRow {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        ctx.focus_scope()
            .set_entry_policy(FocusEntryPolicy::Remembered);

        Element::new()
            .with_style(settings_top_row_style())
            .with_content(StatusChip)
            .with_content(top_actions())
    }
}

fn top_actions() -> Element {
    Element::new()
        .with_style(settings_top_actions_style())
        .with_content(QuickActionButton { spec: FOCUS_ACTION })
        .with_content(QuickActionButton { spec: NIGHT_ACTION })
        .with_content(QuickActionButton { spec: TOOLS_ACTION })
        .with_content(QuickActionButton { spec: EXIT_ACTION })
}
