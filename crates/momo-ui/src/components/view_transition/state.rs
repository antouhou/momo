use super::ViewTransitionDirection;
use daiko::{Id, Vec2};

#[derive(Clone, Default)]
pub(super) struct ViewTransitionState {
    pub(super) current_key: Option<Id>,
    pub(super) previous_key: Option<Id>,
    pub(super) viewport_size: Option<Vec2>,
    pub(super) from_size: Option<Vec2>,
    pub(super) target_size: Option<Vec2>,
    pub(super) settling_size: Option<Vec2>,
    pub(super) current_motion: ViewTransitionSlotMotion,
    pub(super) previous_motion: ViewTransitionSlotMotion,
}

#[derive(Clone, Copy, Default)]
pub(super) struct ViewTransitionMeasurements {
    pub(super) stable_size: Option<Vec2>,
    pub(super) incoming_key: Option<Id>,
    pub(super) incoming_size: Option<Vec2>,
}

#[derive(Clone, Copy, Debug, Default)]
pub(super) struct ViewTransitionSlotMotion {
    from_offset: f32,
    to_offset: f32,
}

impl ViewTransitionSlotMotion {
    pub(super) fn new(from_offset: f32, to_offset: f32) -> Self {
        Self {
            from_offset,
            to_offset,
        }
    }
}

pub(super) fn stable_view_transition_slot_motion() -> ViewTransitionSlotMotion {
    ViewTransitionSlotMotion::new(0.0, 0.0)
}

pub(super) fn incoming_view_transition_slot_motion(
    direction: ViewTransitionDirection,
    slide_distance: f32,
) -> ViewTransitionSlotMotion {
    ViewTransitionSlotMotion::new(incoming_offset(direction, slide_distance), 0.0)
}

pub(super) fn outgoing_view_transition_slot_motion(
    direction: ViewTransitionDirection,
    slide_distance: f32,
) -> ViewTransitionSlotMotion {
    ViewTransitionSlotMotion::new(0.0, outgoing_offset(direction, slide_distance))
}

pub(super) fn outgoing_view_transition_slot_target_offset(
    direction: ViewTransitionDirection,
    slide_distance: f32,
) -> f32 {
    outgoing_offset(direction, slide_distance)
}

pub(super) fn view_transition_slot_motion_offset(
    motion: ViewTransitionSlotMotion,
    progress: f32,
) -> f32 {
    motion.from_offset + (motion.to_offset - motion.from_offset) * progress
}

fn incoming_offset(direction: ViewTransitionDirection, slide_distance: f32) -> f32 {
    direction_sign(direction) * slide_distance
}

fn outgoing_offset(direction: ViewTransitionDirection, slide_distance: f32) -> f32 {
    -direction_sign(direction) * slide_distance
}

fn direction_sign(direction: ViewTransitionDirection) -> f32 {
    match direction {
        ViewTransitionDirection::Forward => 1.0,
        ViewTransitionDirection::Backward => -1.0,
    }
}
