use crate::components::home::model::{
    TILE_BORDER_RADIUS, TILE_BORDER_WIDTH, TILE_CONTENT_GAP, TILE_FOCUS_ANIMATION_DURATION_MS,
    TILE_HEIGHT, TILE_PADDING, TILE_WIDTH,
};
use daiko::{
    animation::{AnimationParameters, easing::EasingFunction, transition},
    component::ComponentContext,
    layout::AlignItems,
    style::{Border, BorderRadius, Color, Stroke, Style, Transform},
    widgets::text::{TextStyle, TextWrap},
};
use momo_kit::style::SYSTEM_TEXT_SIZE;
use std::time::Duration;

#[derive(Clone, Copy)]
pub(crate) struct AppButtonSurfaceMetrics {
    pub(crate) width: f32,
    pub(crate) height: f32,
    pub(crate) border_radius: f32,
    pub(crate) border_width: f32,
}

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
        .with_font_size(SYSTEM_TEXT_SIZE)
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
    app_button_surface_style(
        ctx,
        AppButtonSurfaceMetrics {
            width: TILE_WIDTH,
            height: TILE_HEIGHT,
            border_radius: TILE_BORDER_RADIUS,
            border_width: TILE_BORDER_WIDTH,
        },
        accent,
        transform,
        is_hovering,
        is_focus_visible,
    )
    .with_direction(daiko::layout::FlexDirection::Column)
    .with_align_items(AlignItems::Center)
    .with_padding(TILE_PADDING)
    .with_spacing((TILE_CONTENT_GAP, TILE_CONTENT_GAP))
}

pub(crate) fn app_button_surface_style(
    ctx: &mut ComponentContext,
    metrics: AppButtonSurfaceMetrics,
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
        .with_fixed_size(metrics.width, metrics.height)
        .with_background_color(transition(
            background,
            AnimationParameters::default()
                .with_duration(Duration::from_millis(TILE_FOCUS_ANIMATION_DURATION_MS))
                .to_transition_options(),
            ctx,
        ))
        .with_border(Border::uniform(Stroke::new(
            metrics.border_width,
            transition(
                border_color,
                AnimationParameters::default()
                    .with_duration(Duration::from_millis(80))
                    .with_easing(EasingFunction::EaseOut)
                    .to_transition_options(),
                ctx,
            ),
        )))
        .with_border_radius(BorderRadius::all(metrics.border_radius))
        .with_transform(Some(transform.clone()))
}
