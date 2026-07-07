pub mod state;
mod style;

use daiko::{
    Element, Id,
    component::{Component, ComponentContext},
    navigation::FocusOrigin,
    widgets::text::Text,
};
use momo_kit::interaction::ButtonBehavior;
use tracing::warn;
use self::style::{clock_button_style, clock_text_style};
use crate::components::{
    home::{
        bluetooth::bluetooth_handle, clock_chip::state::ClockButtonLocalState,
        header::HeaderButtonState, model::HOME_CLOCK_STATE_ID, time::read_system_time,
    },
    quick_settings::{
        SETTINGS_MENU_STATE_ID,
        state::{SettingsMenuState, SettingsMenuViewType},
    },
};

#[derive(Clone, Copy)]
pub(super) struct ClockChip;

impl Component for ClockChip {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let local_state = ctx.use_local_state(ClockButtonLocalState::default);
        let lost_focus_due_to_settings_menu_open =
            local_state.read().lost_focus_due_to_settings_menu_open;
        let clock_text = ctx.use_global_state(Id::new(HOME_CLOCK_STATE_ID), read_system_time);
        let settings_overlay_state =
            ctx.use_shared_state(Id::new(SETTINGS_MENU_STATE_ID), SettingsMenuState::default);
        let settings_overlay_started_to_close = settings_overlay_state.read().just_started_closing;
        let should_restore_focus =
            settings_overlay_started_to_close && lost_focus_due_to_settings_menu_open;
        let button = ButtonBehavior::new(ctx)
            .with_requested_focus(should_restore_focus.then_some(FocusOrigin::Programmatic))
            .apply();

        if should_restore_focus {
            local_state.write().lost_focus_due_to_settings_menu_open = false;
        }

        if button.just_activated {
            local_state.write().lost_focus_due_to_settings_menu_open = true;
            let (is_open, is_animating) = {
                let state = settings_overlay_state.read();
                (state.is_open, state.is_animating)
            };
            if is_open || !is_animating {
                // TODO: move bluetooth discovery shutdown into the overlay itself
                let next_is_open = !is_open;
                if !next_is_open
                    && settings_overlay_state.read().active_view == SettingsMenuViewType::Bluetooth
                    && let Err(error) = bluetooth_handle(ctx).stop_discovery()
                {
                    warn!("failed to stop Bluetooth discovery: {error:?}");
                }
                let mut state = settings_overlay_state.write();
                state.is_open = next_is_open;
                state.just_opened = next_is_open;
                state.opened_from_trigger_press = next_is_open;
                state.is_animating = true;
                state.reset_active_view_to_main();
            }
        }

        let state = HeaderButtonState {
            is_active: false,
            is_pressed: button.is_pressed,
            is_hovered: button.is_hovering,
            is_focused: button.is_focused,
        };
        Element::new()
            .with_tag("clock-chip")
            .with_style(clock_button_style(ctx, state))
            .with_content(Text::new(clock_text.read().clone()).with_style(clock_text_style(state)))
    }
}
