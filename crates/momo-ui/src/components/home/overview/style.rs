use super::OverviewCardPosition;
use crate::components::home::model::SECTION_GAP;
use daiko::{
    Vec2,
    animation::{AnimationParameters, easing::EasingFunction, transition},
    component::ComponentContext,
    layout::{AlignItems, FlexDirection, ItemSize, JustifyContent},
    style::{Border, BorderRadius, Color, CursorIcon, Overflow, Stroke, Style},
};
use momo_kit::components::{ROUND_ICON_BUTTON_BORDER_WIDTH, ROUND_ICON_BUTTON_SIZE};
use std::time::Duration;

const OVERVIEW_CLOSE_BUTTON_GAP: f32 = 18.0;
const OVERVIEW_CARD_ACTIVE_HEIGHT_RATIO: f32 = 0.58;
const OVERVIEW_CARD_ACTIVE_WIDTH_RATIO: f32 = 0.5;
const OVERVIEW_CARD_ACTIVE_ASPECT_RATIO: f32 = 480.0 / 278.0;
const OVERVIEW_CARD_ACTIVE_MIN_HEIGHT: f32 = 160.0;
const OVERVIEW_CARD_ACTIVE_MAX_HEIGHT: f32 = 520.0;
const OVERVIEW_CARD_SIDE_WIDTH_RATIO: f32 = 340.0 / 480.0;
const OVERVIEW_CARD_SIDE_HEIGHT_RATIO: f32 = 218.0 / 278.0;
const OVERVIEW_CARD_OVERLAP_RATIO: f32 = 112.0 / 480.0;
const OVERVIEW_CARD_RADIUS: f32 = 18.0;
const OVERVIEW_SIDE_CARD_ORDER: u16 = 0;
const OVERVIEW_ACTIVE_CARD_ORDER: u16 = 1000;
const OVERVIEW_CARD_TRANSITION_MS: u64 = 140;

pub(super) fn overview_style() -> Style {
    Style::new()
        .with_direction(FlexDirection::Column)
        .with_align_items(AlignItems::Center)
        .with_justify_content(JustifyContent::Center)
        .with_spacing((SECTION_GAP, SECTION_GAP))
        .with_fixed_width(ItemSize::Percent(1.0))
        .with_min_height(ItemSize::Points(0.0))
        .with_grow(1.0)
        .with_overflow(Overflow::Visible)
}

pub(super) fn overview_window_close_target_position(active_card_frame: OverviewCardFrame) -> Vec2 {
    let close_button_outer_size = ROUND_ICON_BUTTON_SIZE + ROUND_ICON_BUTTON_BORDER_WIDTH * 2.0;
    Vec2::new(
        active_card_frame.position.x + (active_card_frame.size.x - close_button_outer_size) * 0.5,
        active_card_frame.position.y - close_button_outer_size - OVERVIEW_CLOSE_BUTTON_GAP,
    )
}

pub(super) fn overview_window_close_position_style(rendered_position: Vec2) -> Style {
    Style::new()
        .with_absolute_position(rendered_position)
        .with_fixed_size(ROUND_ICON_BUTTON_SIZE, ROUND_ICON_BUTTON_SIZE)
}

pub(super) fn overview_carousel_style() -> Style {
    Style::new()
        .with_align_items(AlignItems::Center)
        .with_justify_content(JustifyContent::Center)
        .with_fixed_width(ItemSize::Percent(1.0))
        .with_grow(1.0)
        .with_overflow(Overflow::Hidden)
}

#[derive(Clone, Copy)]
pub(super) struct OverviewCardFrame {
    pub(super) position: Vec2,
    pub(super) size: Vec2,
}

pub(super) fn overview_card_stage_style() -> Style {
    Style::new()
        .with_fixed_width(ItemSize::Percent(1.0))
        .with_fixed_height(ItemSize::Percent(1.0))
        .with_overflow(Overflow::Visible)
}

pub(super) fn overview_card_target_frame(
    viewport_size: Vec2,
    card_index: usize,
    active_card_index: usize,
) -> OverviewCardFrame {
    let metrics = overview_card_metrics(viewport_size);
    let active_left = (viewport_size.x - metrics.active_size.x) * 0.5;
    let active_top = (viewport_size.y - metrics.active_size.y) * 0.5;
    let side_top = (viewport_size.y - metrics.side_size.y) * 0.5;
    let position = if card_index == active_card_index {
        Vec2::new(active_left, active_top)
    } else if card_index + 1 == active_card_index {
        Vec2::new(
            active_left - metrics.side_size.x + metrics.overlap,
            side_top,
        )
    } else if card_index == active_card_index + 1 {
        Vec2::new(
            active_left + metrics.active_size.x - metrics.overlap,
            side_top,
        )
    } else if card_index < active_card_index {
        Vec2::new(-metrics.side_size.x, side_top)
    } else {
        Vec2::new(viewport_size.x, side_top)
    };
    let size = if card_index == active_card_index {
        metrics.active_size
    } else {
        metrics.side_size
    };

    OverviewCardFrame { position, size }
}

struct OverviewCardMetrics {
    active_size: Vec2,
    side_size: Vec2,
    overlap: f32,
}

fn overview_card_metrics(viewport_size: Vec2) -> OverviewCardMetrics {
    let height_from_available_height = viewport_size.y * OVERVIEW_CARD_ACTIVE_HEIGHT_RATIO;
    let height_from_available_width =
        viewport_size.x * OVERVIEW_CARD_ACTIVE_WIDTH_RATIO / OVERVIEW_CARD_ACTIVE_ASPECT_RATIO;
    let active_height = height_from_available_height
        .min(height_from_available_width)
        .clamp(
            OVERVIEW_CARD_ACTIVE_MIN_HEIGHT,
            OVERVIEW_CARD_ACTIVE_MAX_HEIGHT,
        );
    let active_width = active_height * OVERVIEW_CARD_ACTIVE_ASPECT_RATIO;
    let active_size = Vec2::new(active_width, active_height);
    let side_size = Vec2::new(
        active_width * OVERVIEW_CARD_SIDE_WIDTH_RATIO,
        active_height * OVERVIEW_CARD_SIDE_HEIGHT_RATIO,
    );

    OverviewCardMetrics {
        active_size,
        side_size,
        overlap: active_width * OVERVIEW_CARD_OVERLAP_RATIO,
    }
}

pub(super) fn overview_card_layout_style(
    position: OverviewCardPosition,
    rendered_frame: OverviewCardFrame,
) -> Style {
    let mut style = Style::new()
        .with_absolute_position(rendered_frame.position)
        .with_fixed_size(rendered_frame.size.x, rendered_frame.size.y)
        .with_direction(FlexDirection::Row)
        .with_align_items(AlignItems::Center)
        .with_justify_content(JustifyContent::Center)
        .with_overflow(Overflow::Visible)
        .with_order(match position {
            OverviewCardPosition::Active => OVERVIEW_ACTIVE_CARD_ORDER,
            OverviewCardPosition::Previous
            | OverviewCardPosition::Next
            | OverviewCardPosition::Hidden => OVERVIEW_SIDE_CARD_ORDER,
        });

    if !matches!(position, OverviewCardPosition::Hidden) {
        style = style.with_cursor(CursorIcon::PointingHand);
    }

    style
}

pub(super) fn overview_card_surface_style(
    ctx: &mut ComponentContext,
    card_index: usize,
    position: OverviewCardPosition,
    is_pressed: bool,
    is_hovered: bool,
    is_focused: bool,
) -> Style {
    let is_hidden = matches!(position, OverviewCardPosition::Hidden);
    let background = if is_hidden {
        Color::TRANSPARENT
    } else {
        placeholder_card_color(card_index, position, is_pressed, is_hovered || is_focused)
    };
    let border_color = if is_hidden {
        Color::TRANSPARENT
    } else if is_focused {
        Color::from_rgb(238, 246, 255)
    } else if is_hovered {
        Color::from_rgba_unmultiplied(238, 246, 255, 174)
    } else {
        Color::from_rgba_unmultiplied(255, 255, 255, 82)
    };

    Style::new()
        .with_grow(1.0)
        .with_background_color(transition(
            background,
            AnimationParameters::default()
                .with_duration(Duration::from_millis(OVERVIEW_CARD_TRANSITION_MS))
                .with_easing(EasingFunction::EaseOut)
                .to_transition_options(),
            ctx,
        ))
        .with_border(Border::uniform(Stroke::new(2.0, border_color)))
        .with_border_radius(BorderRadius::all(OVERVIEW_CARD_RADIUS))
        .with_overflow(Overflow::Hidden)
}

fn placeholder_card_color(
    card_index: usize,
    position: OverviewCardPosition,
    is_pressed: bool,
    is_highlighted: bool,
) -> Color {
    if is_pressed {
        return Color::from_rgb(190, 199, 206);
    }

    if is_highlighted {
        return Color::from_rgb(235, 241, 246);
    }

    match (card_index, position) {
        (0, OverviewCardPosition::Active) => Color::from_rgb(220, 225, 230),
        (1, OverviewCardPosition::Active) => Color::from_rgb(205, 216, 224),
        (_, OverviewCardPosition::Active) => Color::from_rgb(224, 216, 205),
        (0, OverviewCardPosition::Previous | OverviewCardPosition::Next) => {
            Color::from_rgb(194, 200, 206)
        }
        (1, OverviewCardPosition::Previous | OverviewCardPosition::Next) => {
            Color::from_rgb(183, 194, 202)
        }
        (_, OverviewCardPosition::Previous | OverviewCardPosition::Next) => {
            Color::from_rgb(201, 194, 185)
        }
        (_, OverviewCardPosition::Hidden) => Color::TRANSPARENT,
    }
}
