use crate::components::home::model::{
    TILE_BORDER_RADIUS, TILE_FOCUS_ANIMATION_DURATION_MS, TILE_HEIGHT, TILE_WIDTH,
};
use daiko::animation::{AnimationParameters, transition};
use daiko::component::ComponentContext;
use daiko::style::{Border, BorderRadius, Color, Stroke, Style, Transform};
use std::time::Duration;
use daiko::animation::easing::EasingFunction;

pub fn tile_style(
    ctx: &mut ComponentContext,
    accent: Color,
    transform: &Transform,
    paint_decorations: bool,
) -> Style {
    let background = if paint_decorations {
        Color::from_rgb(30, 41, 60)
    } else {
        Color::from_rgba_unmultiplied(20, 26, 38, 220)
    };
    // let background = if is_pressed {
    //     Color::from_rgb(38, 47, 68)
    // } else if is_focus_visible {
    //     Color::from_rgb(30, 41, 60)
    // } else {
    //     Color::from_rgb(20, 26, 38)
    // };

    let border_color = if paint_decorations {
        accent;
        Color::from_hex("#4fc3f7").unwrap_or(accent)
    } else {
        Color::from_rgb(52, 65, 89);
        Color::from_rgba_unmultiplied(79, 195, 247, 76)
    };

    let hehe = if paint_decorations { 0 } else { TILE_FOCUS_ANIMATION_DURATION_MS };

    Style::new()
        .with_fixed_size(TILE_WIDTH, TILE_HEIGHT)
        .with_direction(daiko::layout::FlexDirection::Column)
        .with_align_items(daiko::layout::AlignItems::FlexStart)
        .with_padding(16.0)
        .with_spacing((12.0, 12.0))
        .with_background_color(transition(
            background,
            AnimationParameters::default()
                .with_duration(Duration::from_millis(TILE_FOCUS_ANIMATION_DURATION_MS))
                .to_transition_options(),
            ctx,
        ))
        .with_border(Border::uniform(Stroke::new(
            2.0,
            transition(
                border_color,
                AnimationParameters::default()
                    .with_duration(Duration::from_millis(80))
                    .with_easing(EasingFunction::EaseOut)
                    .to_transition_options(),
                ctx,
            ),
        )))
        .with_border_radius(BorderRadius::all(TILE_BORDER_RADIUS))
        .with_transform(Some(transform.clone()))
}
