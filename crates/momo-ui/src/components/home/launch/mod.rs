pub(super) mod controller;
pub(super) mod overlay;
mod style;

use std::sync::Arc;
use daiko::Vec2;
use crate::components::home::model::LaunchRequest;

pub(super) const HOME_LAUNCH_ANIMATION_ID: &str = "momo_home_launch_animation";
pub(super) const HOME_LAUNCH_OVERLAY_EVENT_CHANNEL_ID: &str =
    "momo_home_launch_overlay_event_channel";
pub(super) const HOME_LAUNCH_BACKDROP_TAG: &str = "launch-overlay";
pub(super) const HOME_LAUNCH_SURFACE_TAG: &str = "launch-overlay-surface";
pub(super) const HOME_LAUNCH_SURFACE_RADIUS: f32 = 0.0;
// pub(super) const HOME_LAUNCH_PRESS_SCALE: f32 = 0.972;
// pub(super) const HOME_LAUNCH_REBOUND_SCALE: f32 = 1.018;
pub(super) const HOME_HERO_ICON_SIZE: f32 = 128.0;
pub(super) const HOME_HERO_ICON_GLYPH_SIZE: usize = 128;
pub(super) const HOME_SHARED_CONTENT_WIDTH: f32 = 480.0;
pub(super) const HOME_SHARED_CONTENT_HEIGHT: f32 = 360.0;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum LaunchPhase {
    Expanding,
    WaitingForSurface,
    Contracting,
}

#[derive(Clone)]
pub(super) struct LaunchTransitionState {
    request: LaunchRequest,
    phase: LaunchPhase,
}

#[derive(Clone)]
pub(super) enum LaunchOverlayEvent {
    Expanded { app_id: Arc<String> },
    Contracted { app_id: Arc<String> },
}

#[derive(Clone, Copy)]
pub(super) struct AnimatedRect {
    position: Vec2,
    size: Vec2,
}
