use crate::components::home::model::{
    TILE_BORDER_RADIUS, TILE_BORDER_WIDTH, TILE_CONTENT_GAP, TILE_FOCUS_ANIMATION_DURATION_MS,
    TILE_HEIGHT, TILE_PADDING, TILE_WIDTH,
};
use daiko::animation::easing::EasingFunction;
use daiko::animation::{AnimationParameters, transition};
use daiko::component::ComponentContext;
use daiko::layout::AlignItems;
use daiko::style::{Border, BorderRadius, Color, Stroke, Style, Transform};
use std::time::Duration;

pub fn tile_style(
    ctx: &mut ComponentContext,
    accent: Color,
    transform: &Transform,
    paint_decorations: bool,
) -> Style {
    let background = Color::from_rgb(14, 18, 27);
    // let background = if is_pressed {
    //     Color::from_rgb(38, 47, 68)
    // } else if is_focus_visible {
    //     Color::from_rgb(30, 41, 60)
    // } else {
    //     Color::from_rgb(20, 26, 38)
    // };

    let border_color = if paint_decorations {
        accent
        // Color::from_hex("#4fc3f7").unwrap_or(accent)
    } else {
        Color::from_rgb(52, 65, 89)
        // Color::from_rgba_unmultiplied(79, 195, 247, 76)
    };

    Style::new()
        .with_fixed_size(TILE_WIDTH, TILE_HEIGHT)
        .with_direction(daiko::layout::FlexDirection::Column)
        .with_align_items(AlignItems::Center)
        .with_padding(TILE_PADDING)
        .with_spacing((TILE_CONTENT_GAP, TILE_CONTENT_GAP))
        .with_background_color(transition(
            background,
            AnimationParameters::default()
                .with_duration(Duration::from_millis(TILE_FOCUS_ANIMATION_DURATION_MS))
                .to_transition_options(),
            ctx,
        ))
        .with_border(Border::uniform(Stroke::new(
            TILE_BORDER_WIDTH,
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
