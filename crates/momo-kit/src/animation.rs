use daiko::{
    animation::{AnimationParameters, transition},
    component::ComponentContext,
    style::Transform,
};
use std::time::Duration;

/// Builds the short scale-and-lift transition used by selectable shell surfaces.
pub fn focus_transform(
    width: f32,
    height: f32,
    is_focused: bool,
    focused_scale: f32,
    focused_lift_y: f32,
    duration: Duration,
    ctx: &mut ComponentContext,
) -> Transform {
    let scale = transition(
        if is_focused { focused_scale } else { 1.0 },
        AnimationParameters::default()
            .with_duration(duration)
            .to_transition_options(),
        ctx,
    );
    let lift_y = transition(
        if is_focused { focused_lift_y } else { 0.0 },
        AnimationParameters::default()
            .with_duration(duration)
            .to_transition_options(),
        ctx,
    );

    Transform::new()
        .with_origin(width * 0.5, height * 0.5)
        .then_scale(scale, scale)
        .then_translate(0.0, lift_y)
}
