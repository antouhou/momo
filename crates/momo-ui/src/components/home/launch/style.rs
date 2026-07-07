use daiko::{
    Vec2,
    layout::{AlignItems, FlexDirection},
    style::{Border, BorderRadius, Color, Overflow, Stroke, Style},
    widgets::text::TextStyle,
};

use crate::components::home::{
    launch::{HOME_SHARED_CONTENT_HEIGHT, HOME_SHARED_CONTENT_WIDTH},
    model::TILE_ICON_SIZE,
};

pub(super) const BACKDROP_MAX_OPACITY: f32 = 0.78;
pub(super) const SURFACE_BORDER_WIDTH: f32 = 2.0;
pub(super) const LAUNCH_SURFACE_BACKGROUND: Color = Color::from_rgb(14, 18, 27);
pub(super) const LAUNCH_TILE_META_GAP: f32 = 4.0;
pub(super) const DESTINATION_LABELS_GAP: f32 = 18.0;
pub(super) const DESTINATION_LABELS_TOP_GAP: f32 = 18.0;
const LAUNCH_TILE_PADDING: f32 = 16.0;
const LAUNCH_TILE_GAP: f32 = 12.0;
const DESTINATION_TITLE_SIZE: f32 = 34.0;
const DESTINATION_SUBTITLE_SIZE: f32 = 16.0;

pub(super) struct RadiusTransition {
    pub source: f32,
    pub target: f32,
}

pub(super) const DESTINATION_ICON_RADIUS: RadiusTransition = RadiusTransition {
    source: 14.0,
    target: 24.0,
};

pub(super) fn with_opacity(color: Color, opacity: f32) -> Color {
    let alpha = ((color.a() as f32) * opacity.clamp(0.0, 1.0)).round() as u8;
    Color::from_rgba_premultiplied(color.r(), color.g(), color.b(), alpha)
}

pub(super) fn backdrop_style(viewport_size: Vec2, opacity: f32) -> Style {
    Style::new()
        .with_fixed_position(Vec2::zero())
        .with_fixed_size(viewport_size.x, viewport_size.y)
        .with_background_color(Color::from_rgba_premultiplied(
            0,
            0,
            0,
            (255.0 * opacity).round() as u8,
        ))
        .with_order(10)
}

pub(super) fn launch_surface_style(
    position: Vec2,
    size: Vec2,
    background: Color,
    border_color: Color,
    border_width: f32,
    border_radius: f32,
) -> Style {
    Style::new()
        .with_fixed_position(position)
        .with_fixed_size(size.x, size.y)
        .with_background_color(background)
        .with_border(Border::uniform(Stroke::new(border_width, border_color)))
        .with_border_radius(BorderRadius::all(border_radius))
        .with_order(11)
}

pub(super) fn launch_tile_icon_style(background: Color) -> Style {
    Style::new()
        .with_fixed_size(TILE_ICON_SIZE, TILE_ICON_SIZE)
        .with_centered_content()
        .with_background_color(background)
        .with_border_radius(BorderRadius::all(DESTINATION_ICON_RADIUS.source))
}

pub(super) fn launch_tile_icon_placeholder_style() -> Style {
    Style::new().with_fixed_size(TILE_ICON_SIZE, TILE_ICON_SIZE)
}

pub(super) fn launch_tile_content_style(size: Vec2) -> Style {
    Style::new()
        .with_fixed_size(size.x, size.y)
        .with_direction(FlexDirection::Column)
        .with_align_items(AlignItems::FlexStart)
        .with_padding(LAUNCH_TILE_PADDING)
        .with_spacing((LAUNCH_TILE_GAP, LAUNCH_TILE_GAP))
}

pub(super) fn destination_icon_style(
    position: Vec2,
    size: f32,
    background: Color,
    radius: f32,
) -> Style {
    Style::new()
        .with_absolute_position(position)
        .with_fixed_size(size, size)
        .with_centered_content()
        .with_background_color(background)
        .with_border_radius(BorderRadius::all(radius))
}

pub(super) fn destination_icon_glyph_style(size: f32) -> Style {
    Style::new().with_fixed_size(size, size)
}

pub(super) fn destination_title_style(opacity: f32) -> TextStyle {
    TextStyle::default()
        .with_font_size(DESTINATION_TITLE_SIZE)
        .with_font_color(with_opacity(Color::from_rgb(247, 250, 255), opacity))
        .with_center_alignment()
}

pub(super) fn destination_subtitle_style(opacity: f32) -> TextStyle {
    TextStyle::default()
        .with_font_size(DESTINATION_SUBTITLE_SIZE)
        .with_font_color(with_opacity(Color::from_rgb(154, 171, 196), opacity))
        .with_center_alignment()
}

pub(super) fn shared_content_style(position: Vec2) -> Style {
    Style::new()
        .with_fixed_position(position)
        .with_fixed_size(HOME_SHARED_CONTENT_WIDTH, HOME_SHARED_CONTENT_HEIGHT)
        .with_order(12)
        .with_direction(FlexDirection::Column)
        .with_align_items(AlignItems::FlexStart)
        .with_overflow(Overflow::Hidden)
}

pub(super) fn labels_offset_style(offset: f32) -> Style {
    Style::new().with_absolute_position(Vec2::new(0.0, offset))
}
