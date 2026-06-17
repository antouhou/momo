use crate::components::home::model::{
    TILE_BORDER_RADIUS, TILE_BORDER_WIDTH, TILE_CONTENT_GAP, TILE_FOCUS_ANIMATION_DURATION_MS,
    TILE_HEIGHT, TILE_PADDING, TILE_WIDTH,
};
use daiko::animation::easing::EasingFunction;
use daiko::animation::{AnimationParameters, transition};
use daiko::component::ComponentContext;
use daiko::layout::AlignItems;
use daiko::style::{Border, BorderRadius, Color, Stroke, Style, Transform};
use daiko::widgets::text::{TextStyle, TextWrap};
use std::time::Duration;

const TILE_TITLE_TEXT_SIZE: f32 = 18.0;

pub fn tile_surface_hover_color() -> Color {
    Color::from_rgb(18, 23, 32)
}

pub fn tile_surface_focus_color() -> Color {
    Color::from_rgb(20, 25, 35)
}

pub fn tile_border_hover_color() -> Color {
    Color::from_rgb(98, 112, 140)
}

pub fn tile_title_color() -> Color {
    Color::from_rgb(240, 245, 255)
}

pub fn tile_title_style() -> TextStyle {
    TextStyle::default()
        .with_font_size(TILE_TITLE_TEXT_SIZE)
        .with_font_color(tile_title_color())
        .with_center_alignment()
        .with_wrap(TextWrap::NoWrap)
}

pub fn tile_style(
    ctx: &mut ComponentContext,
    accent: Color,
    transform: &Transform,
    is_hovering: bool,
    is_focus_visible: bool,
) -> Style {
    let background = if is_focus_visible {
        tile_surface_focus_color()
    } else if is_hovering {
        tile_surface_hover_color()
    } else {
        Color::from_rgba_premultiplied(0, 0, 0, 70)
        // tile_surface_color()
    };

    let border_color = if is_focus_visible {
        accent
    } else if is_hovering {
        tile_border_hover_color()
    } else {
        Color::TRANSPARENT
        //tile_border_color()
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
