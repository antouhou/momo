use crate::components::login_screen::state::{ProfileAction, UserProfile};
use crate::components::login_screen::style::{text_primary_color, text_secondary_color};
use daiko::animation::easing::EasingFunction;
use daiko::animation::{AnimationParameters, transition};
use daiko::component::ComponentContext;
use daiko::layout::{AlignItems, FlexDirection, JustifyContent, SizeConstraint};
use daiko::style::{Border, BorderRadius, Color, CursorIcon, Stroke, Style};
use daiko::widgets::text::{TextStyle, TextWrap, Weight};
use momo_kit::animation::focus_transform;
use std::time::Duration;

const TILE_WIDTH: f32 = 190.0;
const TILE_HEIGHT: f32 = 226.0;
const AVATAR_SIZE: f32 = 142.0;
const AVATAR_RADIUS: f32 = AVATAR_SIZE / 2.0;
const TILE_GAP: f32 = 16.0;
const BORDER_WIDTH: f32 = 3.0;
const FOCUS_SCALE: f32 = 1.07;
const FOCUS_LIFT_Y: f32 = -8.0;
const TRANSITION_MS: u64 = 140;
const AVATAR_TEXT_SIZE: f32 = 48.0;
const ADD_USER_TEXT_SIZE: f32 = 64.0;
const LABEL_TEXT_SIZE: f32 = 19.0;

pub(super) fn tile_style(ctx: &mut ComponentContext, is_highlighted: bool) -> Style {
    Style::new()
        .with_fixed_size(TILE_WIDTH, TILE_HEIGHT)
        .with_direction(FlexDirection::Column)
        .with_align_items(AlignItems::Center)
        .with_justify_content(JustifyContent::Center)
        .with_spacing((TILE_GAP, TILE_GAP))
        .with_cursor(CursorIcon::PointingHand)
        .with_transform(Some(focus_transform(
            TILE_WIDTH,
            TILE_HEIGHT,
            is_highlighted,
            FOCUS_SCALE,
            FOCUS_LIFT_Y,
            Duration::from_millis(TRANSITION_MS),
            ctx,
        )))
}

pub(super) fn avatar_style(
    ctx: &mut ComponentContext,
    action: ProfileAction,
    is_highlighted: bool,
) -> Style {
    let border_color = transition(
        if is_highlighted {
            Color::from_rgb(238, 247, 255)
        } else {
            Color::from_rgba_unmultiplied(255, 255, 255, 72)
        },
        AnimationParameters::default()
            .with_duration(Duration::from_millis(TRANSITION_MS))
            .with_easing(EasingFunction::EaseOut)
            .to_transition_options(),
        ctx,
    );

    Style::new()
        .with_fixed_size(AVATAR_SIZE, AVATAR_SIZE)
        .with_centered_content()
        .with_background_color(avatar_color(action))
        .with_border(Border::uniform(Stroke::new(BORDER_WIDTH, border_color)))
        .with_border_radius(BorderRadius::all(AVATAR_RADIUS))
}

pub(super) fn avatar_text_style(action: ProfileAction) -> TextStyle {
    TextStyle::default()
        .with_font_size(if matches!(action, ProfileAction::AddUser) {
            ADD_USER_TEXT_SIZE
        } else {
            AVATAR_TEXT_SIZE
        })
        .with_weight(Weight::LIGHT)
        .with_font_color(text_primary_color())
        .with_center_alignment()
        .with_wrap(TextWrap::NoWrap)
        .with_size_constraint(SizeConstraint::fixed(AVATAR_SIZE, AVATAR_SIZE))
}

pub(super) fn label_text_style(is_highlighted: bool) -> TextStyle {
    TextStyle::default()
        .with_font_size(LABEL_TEXT_SIZE)
        .with_font_color(if is_highlighted {
            text_primary_color()
        } else {
            text_secondary_color()
        })
        .with_wrap(TextWrap::NoWrap)
}

fn avatar_color(action: ProfileAction) -> Color {
    match action {
        ProfileAction::Login(UserProfile::Anton) => Color::from_rgb(34, 105, 152),
        ProfileAction::Login(UserProfile::Maya) => Color::from_rgb(118, 65, 135),
        ProfileAction::Login(UserProfile::Guest) => Color::from_rgb(54, 91, 82),
        ProfileAction::AddUser => Color::from_rgba_unmultiplied(255, 255, 255, 28),
    }
}
