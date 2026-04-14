pub(super) mod controller;
pub(super) mod overlay;

use crate::components::home::model::LaunchRequest;
use daiko::Vec2;

pub(super) const HOME_LAUNCH_ANIMATION_ID: &str = "momo_home_launch_animation";
pub(super) const HOME_LAUNCH_BACKDROP_TAG: &str = "launch-overlay";
pub(super) const HOME_LAUNCH_SURFACE_TAG: &str = "launch-overlay-surface";
pub(super) const HOME_LAUNCH_SURFACE_RADIUS: f32 = 0.0;
pub(super) const HOME_LAUNCH_PRESS_SCALE: f32 = 0.972;
pub(super) const HOME_LAUNCH_REBOUND_SCALE: f32 = 1.018;
pub(super) const HOME_TILE_BORDER_WIDTH: f32 = 2.0;
pub(super) const HOME_TILE_ICON_SIZE: f32 = 72.0;
pub(super) const HOME_TILE_ICON_OFFSET: f32 = 16.0;
pub(super) const HOME_HERO_ICON_SIZE: f32 = 112.0;
pub(super) const HOME_SHARED_CONTENT_WIDTH: f32 = 360.0;
pub(super) const HOME_SHARED_CONTENT_HEIGHT: f32 = 230.0;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum LaunchPhase {
    Expanding,
    WaitingForSurface,
    Contracting,
}

#[derive(Clone, Copy)]
pub(super) struct LaunchTransitionState {
    request: LaunchRequest,
    phase: LaunchPhase,
}

#[derive(Clone, Copy)]
pub(super) struct AnimatedRect {
    position: Vec2,
    size: Vec2,
}
