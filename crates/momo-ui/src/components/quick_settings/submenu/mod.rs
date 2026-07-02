mod style;

pub(super) use self::style::{
    submenu_body_style, submenu_section_label_style, submenu_section_style,
    submenu_section_title_style,
};

use super::common::{QuickSettingsControlState, QuickSettingsGlyph};
use super::state::{SETTINGS_MENU_STATE_ID, SettingsMenuState, SettingsMenuViewType};
use super::style::settings_text_color;
use super::submenu_button::{
    SubmenuButton, SubmenuButtonState, SubmenuButtonSurface, submenu_button_glyph,
};
use daiko::component::{Component, ComponentContext};
use daiko::navigation::{FocusEntryPolicy, FocusOrigin, NavigationInputAction};
use daiko::{Element, Id};

const BACK_ICON: &[u8] = include_bytes!("../../../../assets/chevron-left.svg");

#[derive(Clone, Copy)]
pub(super) struct SubmenuBackButton {
    pub(super) tag: &'static str,
    pub(super) current_view: SettingsMenuViewType,
}

impl Component for SubmenuBackButton {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let mut pointer = ctx.pointer();
        let focusable = ctx.focusable();
        let state =
            ctx.use_shared_state(Id::new(SETTINGS_MENU_STATE_ID), SettingsMenuState::default);
        let (is_active, should_receive_handoff_focus) = {
            let state = state.read();
            (
                state.active_view == self.current_view,
                state.last_active_view == SettingsMenuViewType::Main
                    && state.active_view == self.current_view,
            )
        };

        if pointer.just_pressed() {
            focusable.request_focus(FocusOrigin::Pointer);
        }

        if should_receive_handoff_focus {
            focusable.request_focus(FocusOrigin::Programmatic);
            if focusable.is_focused() {
                state.write_silent().complete_view_focus_handoff();
            }
        }

        if is_active && (pointer.just_pressed() || focusable.just_activated()) {
            state.write().set_active_view(SettingsMenuViewType::Main);
        }

        SubmenuButton {
            tag: self.tag.to_string(),
            label: "Back".to_string(),
            label_color: None,
            control: QuickSettingsControlState {
                is_hovered: pointer.is_hovering(),
                is_focused: focusable.is_focused(),
            },
            surface: SubmenuButtonSurface::Standard,
            state: SubmenuButtonState::Enabled,
            leading: submenu_button_glyph(
                QuickSettingsGlyph::Asset(BACK_ICON),
                settings_text_color(),
            ),
            trailing: None,
        }
        .to_element(ctx)
    }
}

pub(super) fn handle_submenu_back_navigation(
    ctx: &mut ComponentContext,
    submenu_view: SettingsMenuViewType,
) {
    let focus_scope = ctx.focus_scope();
    focus_scope.set_entry_policy(FocusEntryPolicy::Remembered);
    focus_scope
        .capture_when_contains_focus(&[NavigationInputAction::Cancel, NavigationInputAction::Back]);

    let state = ctx.use_shared_state(Id::new(SETTINGS_MENU_STATE_ID), SettingsMenuState::default);
    let should_go_back = focus_scope.drain_captured_actions().any(|action| {
        matches!(
            action,
            NavigationInputAction::Cancel | NavigationInputAction::Back
        )
    });

    if should_go_back && state.read().active_view == submenu_view {
        state.write().set_active_view(SettingsMenuViewType::Main);
    }
}
