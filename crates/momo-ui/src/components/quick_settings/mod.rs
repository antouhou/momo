pub mod state;
mod style;

use daiko::component::{Component, ComponentContext};
use daiko::{Element, Id};
use daiko::navigation::{FocusBoundary, FocusOrigin, NavigationInputAction};
use daiko::widgets::button::Button;
use daiko::widgets::container::{Container, Fit};
pub(crate) use state::{SettingsMenuState, SETTINGS_MENU_STATE_ID, is_settings_menu_open};
use style::{menu_heading, settings_exit_button_style, settings_menu_style};

#[derive(Clone, Copy)]
pub struct SettingsMenuPanel;

impl Component for SettingsMenuPanel {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let mut pointer = ctx.pointer();
        let focus_scope = ctx.focus_scope();
        let state =
            ctx.use_shared_state(Id::new(SETTINGS_MENU_STATE_ID), SettingsMenuState::default);
        let state_snapshot = *state.read();
        let just_opened = state_snapshot.just_opened;

        focus_scope.set_boundary(FocusBoundary::Escape);
        focus_scope.capture_when_contains_focus(&[
            NavigationInputAction::Cancel,
            NavigationInputAction::Back,
        ]);

        if just_opened {
            focus_scope.request_focus(FocusOrigin::Navigation);
        }

        let exit_button = Button::new(ctx, "Exit").with_style(settings_exit_button_style);
        let exit_clicked = exit_button.clicked();
        if exit_clicked {
            ctx.app_context.close_app();
        }
        let close_from_navigation = focus_scope.drain_captured_actions().any(|action| {
            matches!(
                action,
                NavigationInputAction::Cancel | NavigationInputAction::Back
            )
        });
        let close_from_focus_leave =
            !just_opened && focus_scope.just_left() && !pointer.is_pressed_anywhere();
        let should_close = exit_clicked
            || close_from_navigation
            || (!just_opened && pointer.just_clicked_outside())
            || close_from_focus_leave;

        if should_close || just_opened {
            if close_from_navigation && state_snapshot.opened_from_trigger_press {
                ctx.navigation().request_focus_by_key(
                    crate::components::home::model::home_top_row_settings_focus_key(),
                    FocusOrigin::Navigation,
                );
            }

            *state.write() = SettingsMenuState {
                is_open: !should_close,
                just_opened: false,
                opened_from_trigger_press: if should_close {
                    false
                } else {
                    state_snapshot.opened_from_trigger_press
                },
            };
        }

        Element::new()
            .with_tag("header-settings-menu")
            .with_style(settings_menu_style())
            .with_content(
                Container::vertical()
                    .with_fit(Fit::new().at_least_parent_width().at_least_content_height())
                    .with_spacing((12.0, 12.0))
                    .build()
                    .with_content(menu_heading())
                    .with_content(
                        Element::new()
                            .with_tag("header-settings-exit-button")
                            .with_content(exit_button),
                    ),
            )
    }
}