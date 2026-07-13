mod style;

use self::style::settings_menu_style;
use super::{
    bluetooth_submenu::BluetoothSubmenu,
    main_menu::MainMenu,
    power_submenu::PowerSubmenu,
    state::{
        SETTINGS_MENU_STATE_ID, SETTINGS_VIEW_TRANSITION_ID, SettingsMenuState,
        SettingsMenuViewType,
    },
    style::{
        SETTINGS_MENU_CONTENT_WIDTH, SETTINGS_MENU_EDGE_MARGIN, SETTINGS_MENU_MIN_HEIGHT,
        SETTINGS_MENU_SLIDE_DISTANCE, SETTINGS_MENU_TOP_OFFSET,
    },
};
use crate::components::{
    home::bluetooth::bluetooth_handle,
    view_transition::{
        ViewTransition, ViewTransitionController, ViewTransitionDirection, ViewTransitionEvent,
    },
};
use daiko::{
    Element, Id, Vec2,
    animation::{AnimationParameters, easing::EasingFunction},
    component::{Child, Component, ComponentContext},
    navigation::{FocusBoundary, FocusEntryPolicy, FocusOrigin, NavigationInputAction},
    widgets::overlay::{Overlay, OverlayPositioning},
};
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
        let (is_open, just_opened, active_view) = {
            let state = state.read();
            (state.is_open, state.just_opened, state.active_view)
        };
        let mut pointer = ctx.pointer();
        let focus_scope = ctx.focus_scope();

        if is_open {
            focus_scope.set_boundary(FocusBoundary::Stop);
            if active_view == SettingsMenuViewType::Main {
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
                if should_close
                    && state.read().active_view == SettingsMenuViewType::Bluetooth
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
        let max_drawer_height = (ctx.viewport().unwrap_or_default().size().height
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
    clear_just_started_closing_next_frame: bool,
}

#[derive(Clone, Copy)]
struct SettingsMenuVisibility {
    progress: f32,
    is_visible: bool,
}

fn settings_view(
    view_type: SettingsMenuViewType,
    show_scroll_bars_when_overflowing: bool,
) -> Child {
    match view_type {
        SettingsMenuViewType::Main => MainMenu {
            show_scroll_bars_when_overflowing,
        }
        .into_child(),
        SettingsMenuViewType::Bluetooth => BluetoothSubmenu {
            show_scroll_bars_when_overflowing,
        }
        .into_child(),
        SettingsMenuViewType::Power => PowerSubmenu.into_child(),
    }
}

#[derive(Clone, Copy)]
struct SettingsMenuContent;

impl Component for SettingsMenuContent {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        ctx.focus_scope()
            .set_entry_policy(FocusEntryPolicy::Remembered);
        let view_transition_controller =
            ViewTransition::use_controller(ctx, SETTINGS_VIEW_TRANSITION_ID);
        stop_bluetooth_discovery_after_submenu_transition(ctx, &view_transition_controller);

        let state =
            ctx.use_shared_state(Id::new(SETTINGS_MENU_STATE_ID), SettingsMenuState::default);
        let active_view_type = state.read().active_view;
        let is_view_transitioning = view_transition_controller.is_transitioning();
        let view_transition_state =
            settings_view_transition_state(ctx, active_view_type, is_view_transitioning);
        let previous_active_view_type = view_transition_state.from_view;
        let show_scroll_bars = !is_view_transitioning && !view_transition_state.active_view_changed;

        let view_transition = ViewTransition::new()
            .from(settings_view(previous_active_view_type, show_scroll_bars))
            .to(settings_view(active_view_type, show_scroll_bars))
            .with_id(SETTINGS_VIEW_TRANSITION_ID)
            .with_transition_key(active_view_type)
            .with_direction(view_transition_state.direction)
            .with_slide_distance(SETTINGS_MENU_CONTENT_WIDTH);

        Element::new().with_content(view_transition)
    }
}

#[derive(Clone, Copy, Default)]
struct SettingsMenuViewTransitionDirectionState {
    observed_active_view: Option<SettingsMenuViewType>,
    from_view: Option<SettingsMenuViewType>,
    direction: Option<ViewTransitionDirection>,
}

#[derive(Clone, Copy)]
struct SettingsMenuViewTransitionState {
    from_view: SettingsMenuViewType,
    direction: ViewTransitionDirection,
    active_view_changed: bool,
}

fn settings_view_transition_state(
    ctx: &mut ComponentContext,
    active_view: SettingsMenuViewType,
    is_transitioning: bool,
) -> SettingsMenuViewTransitionState {
    let state = ctx.use_local_state(SettingsMenuViewTransitionDirectionState::default);
    let mut view_state = *state.read();
    let mut active_view_changed = false;

    match view_state.observed_active_view {
        None => {
            view_state.observed_active_view = Some(active_view);
            *state.write_silent() = view_state;
        }
        Some(previous_active_view) if previous_active_view != active_view => {
            view_state.observed_active_view = Some(active_view);
            view_state.from_view = Some(previous_active_view);
            view_state.direction = Some(settings_menu_view_transition_direction(
                previous_active_view,
                active_view,
            ));
            active_view_changed = true;
            *state.write_silent() = view_state;
        }
        _ => {
            if !is_transitioning && view_state.from_view.is_some() {
                view_state.from_view = None;
                *state.write_silent() = view_state;
            }
        }
    }

    SettingsMenuViewTransitionState {
        from_view: view_state.from_view.unwrap_or(active_view),
        direction: view_state
            .direction
            .unwrap_or(ViewTransitionDirection::Forward),
        active_view_changed,
    }
}

fn settings_menu_view_transition_direction(
    from: SettingsMenuViewType,
    to: SettingsMenuViewType,
) -> ViewTransitionDirection {
    match (from, to) {
        (_, SettingsMenuViewType::Main) => ViewTransitionDirection::Backward,
        _ => ViewTransitionDirection::Forward,
    }
}

fn stop_bluetooth_discovery_after_submenu_transition(
    ctx: &mut ComponentContext,
    transition: &ViewTransitionController,
) {
    for event in transition.events() {
        if matches!(
            event,
            ViewTransitionEvent::Completed {
                outgoing_key
            } if outgoing_key == Id::new(SettingsMenuViewType::Bluetooth)
        ) && let Err(error) = bluetooth_handle(ctx).stop_discovery()
        {
            warn!("failed to stop Bluetooth discovery: {error:?}");
        }
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
    let state = ctx.use_shared_state(Id::new(SETTINGS_MENU_STATE_ID), SettingsMenuState::default);
    if motion.clear_just_started_closing_next_frame {
        state.write_silent().just_started_closing = false;
        motion.clear_just_started_closing_next_frame = false;
        *motion_state.write_silent() = motion;
    }

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
                state.write_silent().just_started_closing = true;
                motion.clear_just_started_closing_next_frame = true;
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
