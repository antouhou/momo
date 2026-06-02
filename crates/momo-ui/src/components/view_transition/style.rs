use super::{ViewTransitionDirection, ViewTransitionPhase};
use daiko::Vec2;
use daiko::layout::{FlexDirection, SizeConstraint};
use daiko::style::{Overflow, Style, Transform};

pub(super) const VIEW_TRANSITION_DURATION_MS: u64 = 360;
pub(super) const DEFAULT_VIEW_TRANSITION_SLIDE_DISTANCE: f32 = 40.0;

pub(super) fn view_transition_style(width: f32, fixed_size: Option<Vec2>) -> Style {
    let style = Style::new()
        .with_direction(FlexDirection::Column)
        .with_overflow(Overflow::Hidden);

    if let Some(fixed_size) = fixed_size {
        style.with_fixed_size(fixed_size.x, fixed_size.y)
    } else {
        style.with_size_constraint(SizeConstraint::exact_content_height().with_exact_width(width))
    }
}

pub(super) fn view_transition_slot_style(
    phase: ViewTransitionPhase,
    progress: f32,
    motion: ViewTransitionSlotMotion,
    slide_distance: f32,
) -> Style {
    let offset = view_transition_slot_motion_offset(motion, progress);
    let mut style = Style::new()
        .with_size_constraint(
            SizeConstraint::exact_content_height().with_exact_width(slide_distance),
        )
        .with_transform(Some(Transform::new().then_translate_x(offset)))
        .with_order(match phase {
            ViewTransitionPhase::Stable | ViewTransitionPhase::Incoming => 1,
            ViewTransitionPhase::Outgoing => 0,
        });

    if matches!(
        phase,
        ViewTransitionPhase::Incoming | ViewTransitionPhase::Outgoing
    ) {
        style = style.with_absolute_position(Vec2::new(0.0, 0.0));
    }

    style
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
