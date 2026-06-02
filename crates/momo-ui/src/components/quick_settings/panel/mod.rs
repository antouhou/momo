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
use crate::components::view_transition::{
    ViewTransition, ViewTransitionDirection, ViewTransitionEvent, view_transition_events,
    view_transition_status,
};
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
        let should_render = {
            let state = state.read();
            state.is_open || state.is_animating
        };

        if !should_render {
            return Element::new();
        }

        let (is_open, just_opened, active_view) = {
            let state = state.read();
            (state.is_open, state.just_opened, state.active_view)
        };
        let mut pointer = ctx.pointer();
        let focus_scope = ctx.focus_scope();

        if is_open {
            focus_scope.set_boundary(FocusBoundary::Stop);
            if active_view == SettingsMenuView::Main {
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

        if is_open {
            let is_view_transition_pending = {
                let state = state.read();
                state.last_active_view != state.active_view
            };
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
                let opened_from_trigger_press = state.read().opened_from_trigger_press;
                if close_from_navigation && opened_from_trigger_press {
                    ctx.navigation().request_focus_by_key(
                        crate::components::home::model::home_top_row_settings_focus_key(),
                        FocusOrigin::Navigation,
                    );
                }

                if should_close
                    && state.read().active_view == SettingsMenuView::Bluetooth
                    && let Err(error) = bluetooth_handle(ctx).stop_discovery()
                {
                    warn!("failed to stop Bluetooth discovery: {error:?}");
                }

                let mut state = state.write();
                state.is_open = !should_close;
                state.just_opened = false;
                if should_close {
                    state.opened_from_trigger_press = false;
                }
                state.is_animating = true;
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
        stop_bluetooth_discovery_after_submenu_transition(ctx);

        let state =
            ctx.use_shared_state(Id::new(SETTINGS_MENU_STATE_ID), SettingsMenuState::default);
        let active_view = state.read().active_view;
        let direction = settings_view_transition_direction(ctx, active_view);

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

#[derive(Clone, Copy, Default)]
struct SettingsMenuViewTransitionDirectionState {
    observed_active_view: Option<SettingsMenuView>,
    direction: Option<ViewTransitionDirection>,
}

fn settings_view_transition_direction(
    ctx: &mut ComponentContext,
    active_view: SettingsMenuView,
) -> ViewTransitionDirection {
    let state = ctx.use_local_state(SettingsMenuViewTransitionDirectionState::default);
    let mut view_state = *state.read();

    match view_state.observed_active_view {
        None => {
            view_state.observed_active_view = Some(active_view);
            *state.write_silent() = view_state;
        }
        Some(previous_active_view) if previous_active_view != active_view => {
            view_state.observed_active_view = Some(active_view);
            view_state.direction = Some(settings_menu_view_transition_direction(
                previous_active_view,
                active_view,
            ));
            *state.write_silent() = view_state;
        }
        _ => {}
    }

    view_state
        .direction
        .unwrap_or(ViewTransitionDirection::Forward)
}

fn settings_menu_view_transition_direction(
    from: SettingsMenuView,
    to: SettingsMenuView,
) -> ViewTransitionDirection {
    match (from, to) {
        (SettingsMenuView::Bluetooth, SettingsMenuView::Main) => ViewTransitionDirection::Backward,
        _ => ViewTransitionDirection::Forward,
    }
}

fn stop_bluetooth_discovery_after_submenu_transition(ctx: &mut ComponentContext) {
    for event in view_transition_events(ctx, SETTINGS_VIEW_TRANSITION_ID).iter() {
        if matches!(
            event,
            ViewTransitionEvent::Completed {
                outgoing_key
            } if outgoing_key == Id::new(SettingsMenuView::Bluetooth)
        ) && let Err(error) = bluetooth_handle(ctx).stop_discovery()
        {
            warn!("failed to stop Bluetooth discovery: {error:?}");
        }
    }
}

#[derive(Clone, Copy)]
struct MainSettingsView;

impl Component for MainSettingsView {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let transition_status = view_transition_status(ctx, SETTINGS_VIEW_TRANSITION_ID);

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
    let is_open = state.read().is_open;
    let visibility = settings_menu_visibility(ctx, is_open);

    if !visibility.is_visible && state.read().is_animating {
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
    let mut motion = *motion_state.read();
    let animation = ctx.animation_with_id(
        Id::new(SETTINGS_MENU_ANIMATION_ID),
        AnimationParameters::default()
            .with_duration(Duration::from_millis(SETTINGS_MENU_SLIDE_DURATION_MS))
            .with_easing(EasingFunction::EaseOut),
    );

    match motion.previous_open {
        None => {
            if is_open {
                animation.set_progress(0.0);
                animation.play_forward();
            } else {
                animation.set_progress(0.0);
            }
            motion.previous_open = Some(is_open);
            *motion_state.write_silent() = motion;
        }
        Some(previous_open) if previous_open != is_open => {
            motion.previous_open = Some(is_open);
            if is_open {
                animation.play_forward();
            } else {
                animation.play_backward();
            }
            *motion_state.write_silent() = motion;
        }
        _ => {}
    }

    SettingsMenuVisibility {
        progress: animation.progress(),
        is_visible: is_open || animation.is_running() || animation.progress_linear() > 0.0,
    }
}
