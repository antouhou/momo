use super::{ViewTransitionDirection, ViewTransitionPhase};
use daiko::Vec2;
use daiko::layout::{FlexDirection, SizeConstraint};
use daiko::style::{Overflow, Style, Transform};

pub(super) const VIEW_TRANSITION_DURATION_MS: u64 = 400;
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
    direction: ViewTransitionDirection,
    slide_distance: f32,
) -> Style {
    let offset = slot_offset(phase, progress, direction, slide_distance);
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

fn slot_offset(
    phase: ViewTransitionPhase,
    progress: f32,
    direction: ViewTransitionDirection,
    slide_distance: f32,
) -> f32 {
    let direction_sign = match direction {
        ViewTransitionDirection::Forward => 1.0,
        ViewTransitionDirection::Backward => -1.0,
    };

    match phase {
        ViewTransitionPhase::Stable => 0.0,
        ViewTransitionPhase::Incoming => direction_sign * (1.0 - progress) * slide_distance,
        ViewTransitionPhase::Outgoing => -direction_sign * progress * slide_distance,
    }
}
