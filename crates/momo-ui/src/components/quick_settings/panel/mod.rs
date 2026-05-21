mod style;

use self::style::{
    settings_menu_style, settings_scrollable_style, settings_tile_grid_style,
    settings_tile_row_style, settings_top_actions_style, settings_top_row_style,
};
use super::quick_action_button::{
    EXIT_ACTION, FOCUS_ACTION, NIGHT_ACTION, QuickActionButton, TOOLS_ACTION,
};
use super::settings_tile_button::{SettingsTileButton, TILE_ROWS};
use super::state::{SETTINGS_MENU_STATE_ID, SettingsMenuState};
use super::status_chip::StatusChip;
use super::style::{
    SETTINGS_MENU_EDGE_MARGIN, SETTINGS_MENU_INNER_WIDTH, SETTINGS_MENU_SLIDE_DISTANCE,
    SETTINGS_MENU_TOP_OFFSET, SETTINGS_MENU_VERTICAL_PADDING,
};
use daiko::animation::AnimationParameters;
use daiko::animation::easing::EasingFunction;
use daiko::component::{Component, ComponentContext};
use daiko::navigation::{FocusBoundary, FocusOrigin, NavigationInputAction};
use daiko::widgets::overlay::{Overlay, OverlayPositioning};
use daiko::widgets::scrollable::Scrollable;
use daiko::{Element, Id, Vec2};
use std::time::Duration;

const SETTINGS_MENU_ANIMATION_ID: &str = "momo_home_settings_menu_animation";
const SETTINGS_MENU_SCROLLABLE_ID: &str = "momo_home_settings_menu_scrollable";
const SETTINGS_MENU_SLIDE_DURATION_MS: u64 = 280;
const SETTINGS_MENU_CHROME_HEIGHT: f32 = 56.0;

#[derive(Clone, Copy)]
pub(super) struct SettingsMenuPanel {}

#[derive(Clone, Copy, Default)]
struct SettingsMenuMotionState {
    previous_open: Option<bool>,
}

#[derive(Clone, Copy)]
struct SettingsMenuVisibility {
    progress: f32,
    is_visible: bool,
}

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
            focus_scope.capture_when_contains_focus(&[
                NavigationInputAction::Cancel,
                NavigationInputAction::Back,
            ]);

            if just_opened {
                focus_scope.request_focus(FocusOrigin::Navigation);
            }
        }

        if state_snapshot.is_open {
            let close_from_navigation = focus_scope.drain_captured_actions().any(|action| {
                matches!(
                    action,
                    NavigationInputAction::Cancel | NavigationInputAction::Back
                )
            });
            let close_from_focus_leave =
                !just_opened && focus_scope.just_left() && !pointer.is_pressed_anywhere();
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

                *state.write() = SettingsMenuState {
                    is_open: !should_close,
                    just_opened: false,
                    opened_from_trigger_press: if should_close {
                        false
                    } else {
                        state_snapshot.opened_from_trigger_press
                    },
                    is_animating: true,
                };
            }
        }

        // TODO: verify those values
        let max_drawer_height = (ctx.app_context.viewport().size().height
            - SETTINGS_MENU_TOP_OFFSET
            - SETTINGS_MENU_EDGE_MARGIN)
            .max(160.0);

        let max_scroll_height = (max_drawer_height
            - SETTINGS_MENU_VERTICAL_PADDING * 2.0
            - SETTINGS_MENU_CHROME_HEIGHT)
            .max(120.0);

        Element::new()
            .with_tag("header-settings-menu")
            .with_style(settings_menu_style(max_drawer_height))
            .with_content(top_row())
            .with_content(
                Scrollable::new(settings_tile_grid(), Id::new(SETTINGS_MENU_SCROLLABLE_ID))
                    .size_to_content_with_clamp(Vec2::new(
                        SETTINGS_MENU_INNER_WIDTH,
                        max_scroll_height,
                    ))
                    .with_style(settings_scrollable_style()),
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

fn top_row() -> Element {
    Element::new()
        .with_style(settings_top_row_style())
        .with_content(StatusChip)
        .with_content(top_actions())
}

fn top_actions() -> Element {
    Element::new()
        .with_style(settings_top_actions_style())
        .with_content(QuickActionButton { spec: FOCUS_ACTION })
        .with_content(QuickActionButton { spec: NIGHT_ACTION })
        .with_content(QuickActionButton { spec: TOOLS_ACTION })
        .with_content(QuickActionButton { spec: EXIT_ACTION })
}

fn settings_tile_grid() -> Element {
    let mut grid = Element::new().with_style(settings_tile_grid_style());

    for row in TILE_ROWS {
        let mut row_element = Element::new().with_style(settings_tile_row_style());
        for tile in row {
            row_element.add_content(SettingsTileButton { spec: tile });
        }
        grid.add_content(row_element);
    }

    grid
}
