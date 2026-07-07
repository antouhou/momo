use daiko::{
    Vec2,
    layout::{FlexDirection, SizeConstraint},
    style::{Overflow, Style, Transform},
};
use super::{
    ViewTransitionPhase,
    state::{ViewTransitionSlotMotion, view_transition_slot_motion_offset},
};

pub(super) const VIEW_TRANSITION_DURATION_MS: u64 = 240;
pub(super) const DEFAULT_VIEW_TRANSITION_SLIDE_DISTANCE: f32 = 40.0;

pub(super) fn view_transition_style(width: f32, fixed_size: Option<Vec2>) -> Style {
    let _ = width;

    let style = Style::new()
        .with_direction(FlexDirection::Column)
        .with_grow(1.0);

    if let Some(fixed_size) = fixed_size {
        style
            .with_size_constraint(SizeConstraint::fixed(fixed_size.x, fixed_size.y))
            .with_overflow(Overflow::Hidden)
    } else {
        style
    }
}

pub(super) fn view_transition_slot_style(
    phase: ViewTransitionPhase,
    progress: f32,
    motion: ViewTransitionSlotMotion,
    slide_distance: f32,
    fixed_size: Option<Vec2>,
) -> Style {
    let _ = slide_distance;
    let offset = view_transition_slot_motion_offset(motion, progress);
    let mut style = Style::new()
        .with_grow(1.0)
        .with_transform(Some(Transform::new().then_translate_x(offset)))
        .with_order(match phase {
            ViewTransitionPhase::Stable | ViewTransitionPhase::Incoming => 1,
            ViewTransitionPhase::Outgoing => 0,
        });

    if let Some(fixed_size) = fixed_size {
        style = style.with_size_constraint(SizeConstraint::fixed(fixed_size.x, fixed_size.y));
    }

    if matches!(phase, ViewTransitionPhase::Outgoing) {
        style = style.with_absolute_position(Vec2::new(0.0, 0.0));
    }

    style
}
