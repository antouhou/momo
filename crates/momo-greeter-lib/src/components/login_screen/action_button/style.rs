use std::time::Duration;
use daiko::{
    animation::{AnimationParameters, easing::EasingFunction, transition},
    component::ComponentContext,
    style::{Border, BorderRadius, Color, CursorIcon, Indent, Stroke, Style},
    widgets::text::TextStyle,
};
use momo_kit::animation::focus_transform;

use crate::components::login_screen::style::accent_color;

const BUTTON_WIDTH: f32 = 154.0;
const BUTTON_HEIGHT: f32 = 52.0;
const BUTTON_RADIUS: f32 = BUTTON_HEIGHT / 2.0;
const BUTTON_PADDING: f32 = 18.0;
const BUTTON_TEXT_SIZE: f32 = 17.0;
const FOCUS_SCALE: f32 = 1.04;
const FOCUS_LIFT_Y: f32 = -2.0;
const TRANSITION_MS: u64 = 110;

pub(super) fn action_button_style(
    ctx: &mut ComponentContext,
    is_primary: bool,
    is_highlighted: bool,
) -> Style {
    let background = if is_primary {
        if is_highlighted {
            Color::from_rgb(148, 218, 255)
        } else {
            accent_color()
        }
    } else if is_highlighted {
        Color::from_rgba_unmultiplied(255, 255, 255, 42)
    } else {
        Color::from_rgba_unmultiplied(255, 255, 255, 18)
    };
    let border_color = if is_highlighted {
        Color::from_rgb(239, 248, 255)
    } else {
        Color::from_rgba_unmultiplied(255, 255, 255, 64)
    };

    Style::new()
        .with_fixed_size(BUTTON_WIDTH, BUTTON_HEIGHT)
        .with_centered_content()
        .with_padding(Indent::uniform_horizontal(BUTTON_PADDING))
        .with_background_color(transition(
            background,
            AnimationParameters::default()
                .with_duration(Duration::from_millis(TRANSITION_MS))
                .with_easing(EasingFunction::EaseOut)
                .to_transition_options(),
            ctx,
        ))
        .with_border(Border::uniform(Stroke::new(1.0, border_color)))
        .with_border_radius(BorderRadius::all(BUTTON_RADIUS))
        .with_cursor(CursorIcon::PointingHand)
        .with_transform(Some(focus_transform(
            BUTTON_WIDTH,
            BUTTON_HEIGHT,
            is_highlighted,
            FOCUS_SCALE,
            FOCUS_LIFT_Y,
            Duration::from_millis(TRANSITION_MS),
            ctx,
        )))
}

pub(super) fn action_text_style() -> TextStyle {
    TextStyle::default().with_font_size(BUTTON_TEXT_SIZE)
}
