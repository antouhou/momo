use daiko::layout::{AlignItems, FlexDirection, ItemSize, JustifyContent, SizeConstraint};
use daiko::style::{Color, Indent, Style};
use daiko::widgets::text::{TextStyle, TextWrap};
use momo_kit::style::shell_background_gradient;

pub(super) const SCREEN_PADDING: f32 = 42.0;
pub(super) const SECTION_GAP: f32 = 28.0;
pub(super) const CONTROL_GAP: f32 = 18.0;
pub(super) const FOOTER_HEIGHT: f32 = 88.0;

const TITLE_SIZE: f32 = 44.0;
const SUBTITLE_SIZE: f32 = 22.0;

pub(super) fn text_primary_color() -> Color {
    Color::from_rgb(244, 247, 255)
}

pub(super) fn text_secondary_color() -> Color {
    Color::from_rgb(185, 195, 214)
}

pub(super) fn accent_color() -> Color {
    Color::from_rgb(111, 195, 255)
}

pub(super) fn root_style() -> Style {
    Style::new()
        .with_background(shell_background_gradient())
        .with_direction(FlexDirection::Column)
}

pub(super) fn header_style() -> Style {
    Style::new()
        .with_size_constraint(SizeConstraint::exact_content_height())
        .with_direction(FlexDirection::Row)
        .with_align_items(AlignItems::FlexEnd)
        .with_justify_content(JustifyContent::FlexEnd)
        .with_padding(Indent::new(
            SCREEN_PADDING,
            SCREEN_PADDING,
            SCREEN_PADDING,
            0.0,
        ))
}

pub(super) fn main_content_style() -> Style {
    Style::new()
        .with_grow(1.0)
        .with_direction(FlexDirection::Column)
        .with_align_items(AlignItems::Center)
        .with_justify_content(JustifyContent::Center)
        .with_spacing((SECTION_GAP, SECTION_GAP))
        .with_padding(Indent::uniform_horizontal(SCREEN_PADDING))
}

pub(super) fn title_block_style() -> Style {
    Style::new()
        .with_direction(FlexDirection::Column)
        .with_align_items(AlignItems::Center)
        .with_spacing((8.0, 8.0))
        .with_size_constraint(SizeConstraint::exact_content_size())
}

pub(super) fn profile_row_style() -> Style {
    Style::new()
        .with_direction(FlexDirection::Row)
        .with_align_items(AlignItems::Center)
        .with_justify_content(JustifyContent::Center)
        .with_spacing((CONTROL_GAP, CONTROL_GAP))
        .with_size_constraint(SizeConstraint::exact_content_size())
}

pub(super) fn footer_style() -> Style {
    Style::new()
        .with_fixed_height(ItemSize::Points(FOOTER_HEIGHT))
        .with_direction(FlexDirection::Row)
        .with_align_items(AlignItems::Center)
        .with_justify_content(JustifyContent::Center)
}

pub(super) fn title_text_style() -> TextStyle {
    TextStyle::default().with_font_size(TITLE_SIZE)
}

pub(super) fn subtitle_text_style() -> TextStyle {
    TextStyle::default()
        .with_font_size(SUBTITLE_SIZE)
        .with_font_color(text_secondary_color())
        .with_wrap(TextWrap::NoWrap)
}
