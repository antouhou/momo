mod style;
mod tile_grid;
mod top_row;

use self::style::{settings_content_style, settings_menu_style};
use self::tile_grid::SettingsTileGrid;
use self::top_row::SettingsTopRow;
use super::bluetooth_submenu::BluetoothSubmenu;
use super::common::{settings_middle_row, settings_row};
use super::state::{
    SETTINGS_MENU_STATE_ID, SETTINGS_VIEW_TRANSITION_ID, SettingsMenuState, SettingsMenuView,
};
use super::style::{
    SETTINGS_MENU_CONTENT_WIDTH, SETTINGS_MENU_EDGE_MARGIN, SETTINGS_MENU_MIN_HEIGHT,
    SETTINGS_MENU_SLIDE_DISTANCE, SETTINGS_MENU_TOP_OFFSET,
};
use super::volume_control::VolumeControl;
use crate::components::home::bluetooth::bluetooth_handle;
use crate::components::view_transition::{ViewTransition, ViewTransitionDirection};
use daiko::animation::AnimationParameters;
use daiko::animation::easing::EasingFunction;
use daiko::component::{Component, ComponentContext};
use daiko::navigation::{FocusBoundary, FocusEntryPolicy, FocusOrigin, NavigationInputAction};
use daiko::widgets::overlay::{Overlay, OverlayPositioning};
use daiko::widgets::scrollable::Scrollable;
use daiko::{Element, Id, Vec2};
use std::time::Duration;
use tracing::warn;

const SETTINGS_MENU_ANIMATION_ID: &str = "momo_home_settings_menu_animation";
const SETTINGS_MENU_SLIDE_DURATION_MS: u64 = 280;
#[derive(Clone, Copy)]
pub(super) struct SettingsMenuPanel {}

impl Component for SettingsMenuPanel {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let state =
            ctx.use_shared_state(Id::new(SETTINGS_MENU_STATE_ID), SettingsMenuState::default);
        let state_snapshot = *state.read();

        if !state_snapshot.is_open && !state_snapshot.is_animating {
            return Element::new();
        }

        let just_opened = state_snapshot.just_opened;
        let mut pointer = ctx.pointer();
        let focus_scope = ctx.focus_scope();

        if state_snapshot.is_open {
            focus_scope.set_boundary(FocusBoundary::Stop);
            if state_snapshot.active_view == SettingsMenuView::Main {
                focus_scope.capture_when_contains_focus(&[
                    NavigationInputAction::Cancel,
                    NavigationInputAction::Back,
                ]);
            } else {
                focus_scope.capture_when_contains_focus(&[NavigationInputAction::Cancel]);
            }

            if just_opened {
                focus_scope.request_focus(FocusOrigin::Navigation);
            }
        }

        if state_snapshot.is_open {
            let is_view_transition_pending =
                state_snapshot.last_active_view != state_snapshot.active_view;
            let close_from_navigation = focus_scope.drain_captured_actions().any(|action| {
                matches!(
                    action,
                    NavigationInputAction::Cancel | NavigationInputAction::Back
                )
            });
            let close_from_focus_leave = !is_view_transition_pending
                && !just_opened
                && focus_scope.just_left()
                && !pointer.is_pressed_anywhere();
            let should_close = close_from_navigation
                || (!just_opened && pointer.just_clicked_outside())
                || close_from_focus_leave;

            if should_close || just_opened {
                if close_from_navigation && state_snapshot.opened_from_trigger_press {
                    ctx.navigation().request_focus_by_key(
                        crate::components::home::model::home_top_row_settings_focus_key(),
                        FocusOrigin::Navigation,
                    );
                }

                if should_close
                    && state_snapshot.active_view == SettingsMenuView::Bluetooth
                    && let Err(error) = bluetooth_handle(ctx).stop_discovery()
                {
                    warn!("failed to stop Bluetooth discovery: {error:?}");
                }

                *state.write() = SettingsMenuState {
                    is_open: !should_close,
                    just_opened: false,
                    opened_from_trigger_press: if should_close {
                        false
                    } else {
                        state_snapshot.opened_from_trigger_press
                    },
                    is_animating: true,
                    last_active_view: state_snapshot.last_active_view,
                    active_view: state_snapshot.active_view,
                };
            }
        }

        // TODO: verify those values
        let max_drawer_height = (ctx.app_context.viewport().size().height
            - SETTINGS_MENU_TOP_OFFSET
            - SETTINGS_MENU_EDGE_MARGIN)
            .max(SETTINGS_MENU_MIN_HEIGHT);

        Element::new()
            .with_tag("header-settings-menu")
            .with_style(settings_menu_style(max_drawer_height))
            .with_content(SettingsMenuContent)
    }
}

#[derive(Clone, Copy, Default)]
struct SettingsMenuMotionState {
    previous_open: Option<bool>,
}

#[derive(Clone, Copy)]
struct SettingsMenuVisibility {
    progress: f32,
    is_visible: bool,
}

#[derive(Clone, Copy)]
struct SettingsMenuContent;

impl Component for SettingsMenuContent {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        ctx.focus_scope()
            .set_entry_policy(FocusEntryPolicy::Remembered);

        let state =
            ctx.use_shared_state(Id::new(SETTINGS_MENU_STATE_ID), SettingsMenuState::default);
        let snapshot = *state.read();
        let active_view = snapshot.active_view;
        let previous_active_view = snapshot.last_active_view;

        let direction = match (previous_active_view, active_view) {
            (SettingsMenuView::Bluetooth, SettingsMenuView::Main) => {
                ViewTransitionDirection::Backward
            }
            _ => ViewTransitionDirection::Forward,
        };

        let view_transition = ViewTransition::new(match active_view {
            SettingsMenuView::Main => MainSettingsView.into_child(),
            SettingsMenuView::Bluetooth => BluetoothSubmenu.into_child(),
        })
        .with_id(SETTINGS_VIEW_TRANSITION_ID)
        .with_transition_key(active_view)
        .with_direction(direction)
        .with_slide_distance(SETTINGS_MENU_CONTENT_WIDTH);

        Element::new().with_content(view_transition)
    }
}

#[derive(Clone, Copy)]
struct MainSettingsView;

impl Component for MainSettingsView {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let transition_status = crate::components::view_transition::view_transition_status(
            ctx,
            SETTINGS_VIEW_TRANSITION_ID,
        );

        Element::new()
            .with_style(settings_content_style())
            .with_content(settings_row(SettingsTopRow))
            .with_content(settings_middle_row(VolumeControl))
            .with_content(
                Scrollable::new(SettingsTileGrid, "quick_settings_scrollable")
                    .size_to_content_with_clamp(Vec2::new(f32::INFINITY, f32::INFINITY))
                    .with_visible_scroll_bars(!transition_status.is_transitioning),
            )
    }
}

pub(crate) fn settings_overlay(ctx: &mut ComponentContext) -> Overlay {
    let state = ctx.use_shared_state(Id::new(SETTINGS_MENU_STATE_ID), SettingsMenuState::default);
    let state_snapshot = *state.read();
    let visibility = settings_menu_visibility(ctx, state_snapshot.is_open);

    if !visibility.is_visible && state_snapshot.is_animating {
        *state.write() = SettingsMenuState::default();
    }

    let slide_x = (1.0 - visibility.progress) * SETTINGS_MENU_SLIDE_DISTANCE;

    Overlay::new_content_sized(SettingsMenuPanel {})
        .with_positioning(OverlayPositioning::RelativeToTopRightWindowCorner)
        .add_offset(Vec2::new(
            -SETTINGS_MENU_EDGE_MARGIN + slide_x,
            SETTINGS_MENU_TOP_OFFSET,
        ))
}

fn settings_menu_visibility(ctx: &mut ComponentContext, is_open: bool) -> SettingsMenuVisibility {
    let motion_state = ctx.use_local_state(SettingsMenuMotionState::default);
    let mut snapshot = *motion_state.read();
    let animation = ctx.animation_with_id(
        Id::new(SETTINGS_MENU_ANIMATION_ID),
        AnimationParameters::default()
            .with_duration(Duration::from_millis(SETTINGS_MENU_SLIDE_DURATION_MS))
            .with_easing(EasingFunction::EaseOut),
    );

    match snapshot.previous_open {
        None => {
            if is_open {
                animation.set_progress(0.0);
                animation.play_forward();
            } else {
                animation.set_progress(0.0);
            }
            snapshot.previous_open = Some(is_open);
            *motion_state.write_silent() = snapshot;
        }
        Some(previous_open) if previous_open != is_open => {
            snapshot.previous_open = Some(is_open);
            if is_open {
                animation.play_forward();
            } else {
                animation.play_backward();
            }
            *motion_state.write_silent() = snapshot;
        }
        _ => {}
    }

    SettingsMenuVisibility {
        progress: animation.progress(),
        is_visible: is_open || animation.is_running() || animation.progress_linear() > 0.0,
    }
}
