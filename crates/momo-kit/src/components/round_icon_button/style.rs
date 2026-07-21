use super::{ROUND_ICON_BUTTON_BORDER_WIDTH, ROUND_ICON_BUTTON_SIZE, RoundIconButtonVariant};
use crate::interaction::ButtonInteractionState;
use daiko::{
    animation::{AnimationParameters, TransitionOptions, easing::EasingFunction, transition},
    component::ComponentContext,
    layout::{AlignItems, FlexDirection, JustifyContent},
    style::{Border, BorderRadius, Color, CursorIcon, Stroke, Style, Transform},
};
use std::time::Duration;

const ROUND_ICON_BUTTON_RADIUS: f32 = ROUND_ICON_BUTTON_SIZE * 0.5;
const ROUND_ICON_BUTTON_TRANSITION_DURATION: Duration = Duration::from_millis(120);
const ROUND_ICON_BUTTON_FOCUS_SCALE: f32 = 1.015;
const ROUND_ICON_BUTTON_FOCUS_LIFT_Y: f32 = -1.0;

pub(super) fn round_icon_button_style(
    context: &mut ComponentContext,
    interaction: &ButtonInteractionState,
    variant: RoundIconButtonVariant,
) -> Style {
    let background_color = background_color(interaction, variant);
    let border_color = border_color(interaction, variant);

    Style::new()
        .with_fixed_size(ROUND_ICON_BUTTON_SIZE, ROUND_ICON_BUTTON_SIZE)
        .with_direction(FlexDirection::Row)
        .with_align_items(AlignItems::Center)
        .with_justify_content(JustifyContent::Center)
        .with_transform(Some(focus_transform(interaction.is_focused, context)))
        .with_background_color(transition(background_color, transition_options(), context))
        .with_border(Border::uniform(Stroke::new(
            ROUND_ICON_BUTTON_BORDER_WIDTH,
            transition(border_color, transition_options(), context),
        )))
        .with_border_radius(BorderRadius::all(ROUND_ICON_BUTTON_RADIUS))
        .with_cursor(CursorIcon::PointingHand)
}

pub(super) fn round_icon_button_foreground_color(
    interaction: &ButtonInteractionState,
    variant: RoundIconButtonVariant,
) -> Color {
    match variant {
        RoundIconButtonVariant::Danger => Color::from_rgb(255, 231, 235),
        RoundIconButtonVariant::Standard | RoundIconButtonVariant::Accent
            if interaction.is_hovering || interaction.is_focused =>
        {
            Color::from_rgb(12, 16, 20)
        }
        RoundIconButtonVariant::Standard | RoundIconButtonVariant::Accent => {
            Color::from_rgb(235, 240, 247)
        }
    }
}

fn background_color(
    interaction: &ButtonInteractionState,
    variant: RoundIconButtonVariant,
) -> Color {
    match (variant, interaction.is_focused, interaction.is_hovering) {
        (RoundIconButtonVariant::Danger, true, _) => Color::from_rgb(108, 37, 50),
        (RoundIconButtonVariant::Danger, false, true) => Color::from_rgb(92, 32, 43),
        (RoundIconButtonVariant::Danger, false, false) => Color::from_rgb(74, 28, 36),
        (RoundIconButtonVariant::Standard | RoundIconButtonVariant::Accent, true, _) => {
            Color::from_rgb(244, 246, 249)
        }
        (RoundIconButtonVariant::Standard | RoundIconButtonVariant::Accent, false, true) => {
            Color::from_rgb(236, 240, 243)
        }
        (RoundIconButtonVariant::Accent, false, false) => Color::from_rgb(104, 79, 140),
        (RoundIconButtonVariant::Standard, false, false) => Color::from_rgb(24, 28, 31),
    }
}

fn border_color(interaction: &ButtonInteractionState, variant: RoundIconButtonVariant) -> Color {
    match (variant, interaction.is_focused, interaction.is_hovering) {
        (RoundIconButtonVariant::Danger, true, _) => {
            Color::from_rgba_unmultiplied(255, 189, 198, 220)
        }
        (RoundIconButtonVariant::Danger, false, true) => {
            Color::from_rgba_unmultiplied(255, 189, 198, 184)
        }
        (RoundIconButtonVariant::Danger, false, false) => {
            Color::from_rgba_unmultiplied(255, 160, 174, 72)
        }
        (RoundIconButtonVariant::Standard | RoundIconButtonVariant::Accent, true, _) => {
            Color::from_rgba_unmultiplied(255, 255, 255, 196)
        }
        (RoundIconButtonVariant::Standard | RoundIconButtonVariant::Accent, false, true) => {
            Color::from_rgba_unmultiplied(255, 255, 255, 138)
        }
        (RoundIconButtonVariant::Standard | RoundIconButtonVariant::Accent, false, false) => {
            Color::from_rgba_unmultiplied(255, 255, 255, 28)
        }
    }
}

fn focus_transform(is_focused: bool, context: &mut ComponentContext) -> Transform {
    let scale = transition(
        if is_focused {
            ROUND_ICON_BUTTON_FOCUS_SCALE
        } else {
            1.0
        },
        transition_options(),
        context,
    );
    let lift_y = transition(
        if is_focused {
            ROUND_ICON_BUTTON_FOCUS_LIFT_Y
        } else {
            0.0
        },
        transition_options(),
        context,
    );

    Transform::new()
        .with_origin(ROUND_ICON_BUTTON_SIZE * 0.5, ROUND_ICON_BUTTON_SIZE * 0.5)
        .then_scale(scale, scale)
        .then_translate(0.0, lift_y)
}

fn transition_options() -> Option<[TransitionOptions<()>; 1]> {
    AnimationParameters::default()
        .with_duration(ROUND_ICON_BUTTON_TRANSITION_DURATION)
        .with_easing(EasingFunction::EaseOut)
        .to_transition_options()
}
