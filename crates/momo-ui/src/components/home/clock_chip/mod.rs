pub mod state;

use crate::components::home::bluetooth::bluetooth_handle;
use crate::components::home::clock_chip::state::ClockButtonLocalState;
use crate::components::home::header::{
    HEADER_BUTTON_HEIGHT, HEADER_BUTTON_RADIUS, HEADER_CLOCK_WIDTH, HeaderButtonMetrics,
    HeaderButtonState, header_button_style,
};
use crate::components::home::model::HOME_CLOCK_STATE_ID;
use crate::components::home::time::read_system_time;
use crate::components::quick_settings::SETTINGS_MENU_STATE_ID;
use crate::components::quick_settings::state::{SettingsMenuState, SettingsMenuViewType};
use daiko::component::{Component, ComponentContext};
use daiko::navigation::FocusOrigin;
use daiko::style::{Color, Style};
use daiko::widgets::text::{Text, TextStyle, TextWrap};
use daiko::{Element, Id};
use tracing::warn;

#[derive(Clone, Copy)]
pub(super) struct ClockChip;

impl Component for ClockChip {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let local_state = ctx.use_local_state(ClockButtonLocalState::default);
        let lost_focus_due_to_settings_menu_open =
            local_state.read().lost_focus_due_to_settings_menu_open;
        let mut pointer = ctx.pointer();
        let focusable = ctx.focusable();
        let clock_text = ctx.use_global_state(Id::new(HOME_CLOCK_STATE_ID), read_system_time);
        let just_pressed = pointer.just_pressed();
        let settings_overlay_state =
            ctx.use_shared_state(Id::new(SETTINGS_MENU_STATE_ID), SettingsMenuState::default);
        let settings_overlay_started_to_close = settings_overlay_state.read().just_started_closing;

        if just_pressed {
            focusable.request_focus(FocusOrigin::Pointer);
        }

        if settings_overlay_started_to_close && lost_focus_due_to_settings_menu_open {
            local_state.write().lost_focus_due_to_settings_menu_open = false;
            focusable.request_focus(FocusOrigin::Programmatic);
        }

        let just_activated = just_pressed || focusable.just_activated();

        if just_activated {
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
            is_pressed: pointer.is_pressed(),
            is_hovered: pointer.is_hovering(),
            is_focused: focusable.is_focused(),
        };
        let text_color = if state.is_pressed || state.is_hovered || state.is_focused {
            Color::from_rgb(10, 13, 18)
        } else {
            Color::from_rgb(232, 238, 250)
        };

        Element::new()
            .with_tag("clock-chip")
            .with_style(clock_button_style(ctx, state))
            .with_content(
                Text::new(clock_text.read().clone()).with_style(
                    TextStyle::default()
                        .with_font_size(22.0)
                        .with_line_height(1.0)
                        .with_font_color(text_color)
                        .with_wrap(TextWrap::NoWrap),
                ),
            )
    }
}

fn clock_button_style(ctx: &mut ComponentContext, state: HeaderButtonState) -> Style {
    header_button_style(
        ctx,
        HeaderButtonMetrics {
            width: HEADER_CLOCK_WIDTH,
            height: HEADER_BUTTON_HEIGHT,
            radius: HEADER_BUTTON_RADIUS,
        },
        state,
        true,
    )
}
