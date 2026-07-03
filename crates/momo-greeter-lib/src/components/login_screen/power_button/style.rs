use crate::components::login_screen::style::{text_primary_color, text_secondary_color};
use daiko::component::ComponentContext;
use daiko::style::{Border, BorderRadius, Color, CursorIcon, Stroke, Style};
use daiko::widgets::text::{TextStyle, TextWrap};
use momo_kit::animation::focus_transform;
use std::time::Duration;

const BUTTON_SIZE: f32 = 48.0;
const BUTTON_RADIUS: f32 = BUTTON_SIZE / 2.0;
const ICON_SIZE: f32 = 25.0;
const FOCUS_SCALE: f32 = 1.08;
const FOCUS_LIFT_Y: f32 = -2.0;
const TRANSITION_MS: u64 = 110;

pub(super) fn power_button_style(ctx: &mut ComponentContext, is_highlighted: bool) -> Style {
    Style::new()
        .with_fixed_size(BUTTON_SIZE, BUTTON_SIZE)
        .with_centered_content()
        .with_background_color(if is_highlighted {
            Color::from_rgba_unmultiplied(255, 255, 255, 48)
        } else {
            Color::from_rgba_unmultiplied(255, 255, 255, 22)
        })
        .with_border(Border::uniform(Stroke::new(
            1.0,
            Color::from_rgba_unmultiplied(255, 255, 255, 72),
        )))
        .with_border_radius(BorderRadius::all(BUTTON_RADIUS))
        .with_cursor(CursorIcon::PointingHand)
        .with_transform(Some(focus_transform(
            BUTTON_SIZE,
            BUTTON_SIZE,
            is_highlighted,
            FOCUS_SCALE,
            FOCUS_LIFT_Y,
            Duration::from_millis(TRANSITION_MS),
            ctx,
        )))
}

pub(super) fn power_text_style(is_highlighted: bool) -> TextStyle {
    TextStyle::default()
        .with_font_size(ICON_SIZE)
        .with_font_color(if is_highlighted {
            text_primary_color()
        } else {
            text_secondary_color()
        })
        .with_center_alignment()
        .with_wrap(TextWrap::NoWrap)
}
