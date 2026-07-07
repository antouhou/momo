use std::time::Duration;
use daiko::{
    animation::{AnimationParameters, easing::EasingFunction, transition},
    component::ComponentContext,
    layout::{AlignItems, FlexDirection, ItemSize, JustifyContent, SizeConstraint},
    style::{Border, BorderRadius, Color, CursorIcon, Indent, Overflow, Stroke, Style},
};
use crate::components::home::model::SCREEN_PADDING;

const HEADER_MENU_HEIGHT: f32 = 44.0;
pub(in crate::components::home) const HEADER_BUTTON_HEIGHT: f32 = 38.0;
pub(in crate::components::home) const HEADER_BUTTON_RADIUS: f32 = 19.0;
pub(in crate::components::home) const HEADER_CLOCK_WIDTH: f32 = 104.0;
const HEADER_TRANSITION_MS: u64 = 100;
const HEADER_BORDER_TRANSITION_MS: u64 = 80;
const HEADER_SIDE_WIDTH: f32 = 0.3;

#[derive(Clone, Copy)]
pub(in crate::components::home) struct HeaderButtonMetrics {
    pub width: f32,
    pub height: f32,
    pub radius: f32,
}

#[derive(Clone, Copy)]
pub(in crate::components::home) struct HeaderButtonState {
    pub is_active: bool,
    pub is_pressed: bool,
    pub is_hovered: bool,
    pub is_focused: bool,
}

pub(super) fn header_style() -> Style {
    Style::new()
        .with_justify_content(JustifyContent::Center)
        .with_background_color(Color::from_rgba_premultiplied(12, 16, 18, 178))
        .with_overflow(Overflow::Visible)
        .with_direction(FlexDirection::Column)
        .with_size_constraint(SizeConstraint::exact_content_height())
        .with_padding(Indent::new(
            SCREEN_PADDING,
            SCREEN_PADDING,
            SCREEN_PADDING,
            0.0,
        ))
}

pub(super) fn header_row_style() -> Style {
    Style::new()
        .with_overflow(Overflow::Visible)
        .with_fixed_height(ItemSize::Points(HEADER_MENU_HEIGHT))
        .with_direction(FlexDirection::Row)
        .with_justify_content(JustifyContent::SpaceBetween)
        .with_align_items(AlignItems::Center)
}

pub(super) fn header_side_style(align_end: bool) -> Style {
    let mut style = Style::new()
        .with_align_items(AlignItems::Center)
        .with_size_constraint(SizeConstraint {
            min_width: Some(ItemSize::Percent(HEADER_SIDE_WIDTH)),
            max_width: Some(ItemSize::Percent(HEADER_SIDE_WIDTH)),
            ..SizeConstraint::default()
        })
        .with_grow(0.0);
    if align_end {
        style = style.with_justify_content(JustifyContent::FlexEnd);
    }
    style
}

pub(in crate::components::home) fn header_button_style(
    ctx: &mut ComponentContext,
    metrics: HeaderButtonMetrics,
    state: HeaderButtonState,
    paint_surface: bool,
) -> Style {
    let is_lifted = state.is_hovered || state.is_focused;
    let background = if !paint_surface {
        Color::TRANSPARENT
    } else if state.is_pressed {
        Color::from_rgb(204, 210, 216)
    } else if state.is_active || is_lifted {
        Color::from_rgb(236, 240, 243)
    } else {
        Color::TRANSPARENT
    };
    let border_color = if paint_surface && (state.is_active || is_lifted) {
        Color::from_rgba_unmultiplied(255, 255, 255, 172)
    } else {
        Color::TRANSPARENT
    };

    Style::new()
        .with_overflow(Overflow::Visible)
        .with_fixed_size(metrics.width, metrics.height)
        .with_direction(FlexDirection::Row)
        .with_align_items(AlignItems::Center)
        .with_justify_content(JustifyContent::Center)
        .with_background_color(transition(
            background,
            AnimationParameters::default()
                .with_duration(Duration::from_millis(HEADER_TRANSITION_MS))
                .with_easing(EasingFunction::EaseOut)
                .to_transition_options(),
            ctx,
        ))
        .with_border(Border::uniform(Stroke::new(
            1.0,
            transition(
                border_color,
                AnimationParameters::default()
                    .with_duration(Duration::from_millis(HEADER_BORDER_TRANSITION_MS))
                    .with_easing(EasingFunction::EaseOut)
                    .to_transition_options(),
                ctx,
            ),
        )))
        .with_border_radius(BorderRadius::all(metrics.radius))
        .with_cursor(CursorIcon::PointingHand)
        .with_order(1)
}

pub(super) fn central_container_style() -> Style {
    Style::new()
        .with_justify_content(JustifyContent::Center)
        .with_align_items(AlignItems::Center)
        .with_grow(1.0)
}
