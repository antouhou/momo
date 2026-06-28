use crate::components::home::app_tile::AppInfo;
use daiko::Vec2;
use daiko::animation::{AnimationParameters, transition};
use daiko::component::ComponentContext;
use daiko::navigation::FocusKey;
use daiko::style::Transform;
use std::time::Duration;

pub(super) const HOME_APP_GRID_FOCUSED_KEY_ID: &str = "momo_home_app_grid_focused_key";
pub(super) const HOME_APP_GRID_SCROLL_ACCUMULATOR_ID: &str =
    "momo_home_app_grid_scroll_accumulator";
pub(super) const HOME_APP_GRID_SMOOTH_OFFSET_ID: &str = "momo_home_app_grid_smooth_offset";
pub(super) const HOME_CLOCK_THREAD_ID: &str = "momo_home_clock_thread_started";
pub(super) const HOME_CLOCK_STATE_ID: &str = "momo_home_clock_text";
pub(super) const HOME_BLUETOOTH_HANDLE_ID: &str = "momo_home_bluetooth_handle";
pub(super) const HOME_BLUETOOTH_OBSERVATION_ID: &str = "momo_home_bluetooth_observation";
pub(super) const HOME_BLUETOOTH_STATE_ID: &str = "momo_home_bluetooth_state";
pub(super) const HOME_BATTERY_HANDLE_ID: &str = "momo_home_battery_handle";
pub(super) const HOME_BATTERY_OBSERVATION_ID: &str = "momo_home_battery_observation";
pub(super) const HOME_BATTERY_STATE_ID: &str = "momo_home_battery_state";
pub(super) const HOME_VOLUME_HANDLE_ID: &str = "momo_home_volume_handle";
pub(super) const HOME_VOLUME_OBSERVATION_ID: &str = "momo_home_volume_observation";
pub(super) const HOME_VOLUME_STATE_ID: &str = "momo_home_volume_state";
pub(super) const HOME_LAUNCH_CHANNEL_ID: &str = "momo_home_launch_channel";
// pub(super) const HOME_TOP_ROW_APPS_FOCUS_KEY_ID: &str = "momo_home_top_row_apps";
// pub(super) const HOME_TOP_ROW_SETTINGS_FOCUS_KEY_ID: &str = "momo_home_top_row_settings";

pub(super) const SCREEN_PADDING: f32 = 40.0;
pub(super) const SECTION_GAP: f32 = 24.0;
pub(super) const GRID_GAP: f32 = 18.0;
pub(super) const TILE_WIDTH: f32 = 248.0;
pub(super) const TILE_HEIGHT: f32 = 176.0;
pub(super) const TILE_BORDER_RADIUS: f32 = 18.0;
pub(crate) const TILE_BORDER_WIDTH: f32 = 2.0;
pub(super) const TILE_FOCUS_SCALE: f32 = 1.05;
pub(super) const TILE_FOCUS_LIFT_Y: f32 = -3.0;
pub(super) const TILE_FOCUS_ANIMATION_DURATION_MS: u64 = 100;
pub(super) const TILE_ICON_SIZE: f32 = 96.0;
pub(super) const TILE_ICON_GLYPH_SIZE: usize = 96;
pub(super) const TILE_PADDING: f32 = 16.0;
pub(super) const TILE_CONTENT_GAP: f32 = 12.0;

// pub fn home_top_row_apps_focus_key() -> FocusKey {
//     FocusKey::new(HOME_TOP_ROW_APPS_FOCUS_KEY_ID)
// }

// pub fn home_top_row_settings_focus_key() -> FocusKey {
//     FocusKey::new(HOME_TOP_ROW_SETTINGS_FOCUS_KEY_ID)
// }

#[derive(Clone)]
pub(super) struct LaunchRequest {
    pub app: AppInfo,
    pub restore_focus: LaunchRestoreFocus,
    pub position: Vec2,
    pub size: Vec2,
    pub icon_position: Vec2,
    pub icon_size: Vec2,
}

#[derive(Clone, Copy)]
pub(crate) enum LaunchRestoreFocus {
    AppGrid,
    Dock(FocusKey),
}

pub(super) fn columns_for_width(width: f32) -> usize {
    let slot = TILE_WIDTH + GRID_GAP;
    ((width + GRID_GAP) / slot).floor().max(1.0) as usize
}

pub(super) fn rows_for_height(height: f32) -> usize {
    let slot = TILE_HEIGHT + GRID_GAP;
    ((height + GRID_GAP) / slot).floor().max(1.0) as usize
}

pub(super) fn tile_icon_origin() -> Vec2 {
    Vec2::new(
        (TILE_WIDTH - TILE_ICON_SIZE) / 2.0,
        TILE_PADDING + TILE_BORDER_WIDTH,
    )
}

pub(crate) fn tile_focus_transform(
    size: Vec2,
    is_focused: bool,
    ctx: &mut ComponentContext,
) -> Transform {
    let scale = transition(
        if is_focused { TILE_FOCUS_SCALE } else { 1.0 },
        AnimationParameters::default()
            .with_duration(Duration::from_millis(TILE_FOCUS_ANIMATION_DURATION_MS))
            .to_transition_options(),
        ctx,
    );
    let lift_y = transition(
        if is_focused { TILE_FOCUS_LIFT_Y } else { 0.0 },
        AnimationParameters::default()
            .with_duration(Duration::from_millis(TILE_FOCUS_ANIMATION_DURATION_MS))
            .to_transition_options(),
        ctx,
    );

    Transform::new()
        .with_origin(size.x * 0.5, size.y * 0.5)
        .then_scale(scale, scale)
        .then_translate(0.0, lift_y)
}

pub(super) fn transformed_local_rect(
    position: Vec2,
    transform: &Transform,
    local_position: Vec2,
    size: Vec2,
) -> (Vec2, Vec2) {
    let effective_transform = transform
        .clone()
        .with_position_relative_to_parent(position.x, position.y)
        .compose_2(&Transform::new());
    let corners = [
        effective_transform.transform_local_point2d_to_world(local_position.x, local_position.y),
        effective_transform
            .transform_local_point2d_to_world(local_position.x + size.x, local_position.y),
        effective_transform
            .transform_local_point2d_to_world(local_position.x, local_position.y + size.y),
        effective_transform
            .transform_local_point2d_to_world(local_position.x + size.x, local_position.y + size.y),
    ];
    let (min_x, max_x) = corners
        .iter()
        .map(|(x, _)| *x)
        .fold((f32::INFINITY, f32::NEG_INFINITY), |(min_x, max_x), x| {
            (min_x.min(x), max_x.max(x))
        });
    let (min_y, max_y) = corners
        .iter()
        .map(|(_, y)| *y)
        .fold((f32::INFINITY, f32::NEG_INFINITY), |(min_y, max_y), y| {
            (min_y.min(y), max_y.max(y))
        });

    (
        Vec2::new(min_x, min_y),
        Vec2::new(max_x - min_x, max_y - min_y),
    )
}
