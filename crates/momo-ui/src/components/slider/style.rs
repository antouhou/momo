use std::time::Duration;
use daiko::{
    Vec2,
    animation::{AnimationParameters, easing::EasingFunction, transition},
    component::ComponentContext,
    layout::{AlignItems, FlexDirection, ItemSize},
    style::{Border, BorderRadius, Stroke, Style},
};
use super::Slider;

const SLIDER_BORDER_TRANSITION_MS: u64 = 120;

pub(super) fn slider_root_style(slider: &Slider) -> Style {
    let root_height = slider_root_height(slider);
    let style = Style::new()
        .with_fixed_height(ItemSize::Points(root_height))
        .with_direction(FlexDirection::Row)
        .with_align_items(AlignItems::Center);

    if let Some(track_width) = slider.track_width {
        style.with_fixed_size(track_width, root_height)
    } else {
        style.with_grow(1.0)
    }
}

pub(super) fn slider_track_style(track_width: f32, slider: &Slider) -> Style {
    Style::new()
        .with_fixed_size(track_width, slider.track_height)
        .with_absolute_position(Vec2::new(0.0, slider_track_y(slider)))
        .with_background_color(slider.track_color)
        .with_border_radius(BorderRadius::all(slider.track_height / 2.0))
}

pub(super) fn slider_fill_style(track_width: f32, slider: &Slider, width: f32) -> Style {
    Style::new()
        .with_absolute_position(Vec2::new(0.0, 0.0))
        .with_fixed_size(width.min(track_width), slider.track_height)
        .with_background_color(slider.fill_color)
        .with_border_radius(BorderRadius::all(slider.track_height / 2.0))
}

pub(super) fn slider_thumb_style(
    slider: &Slider,
    offset: f32,
    ctx: &mut ComponentContext,
) -> Style {
    let border_color = if slider.is_highlighted {
        slider.highlighted_thumb_border_color
    } else {
        slider.thumb_border_color
    };
    let y = (slider_root_height(slider) - slider.thumb_size) * 0.5;

    Style::new()
        .with_absolute_position(Vec2::new(offset, y))
        .with_fixed_size(slider.thumb_size, slider.thumb_size)
        .with_background_color(slider.thumb_color)
        .with_border(Border::uniform(Stroke::new(
            1.0,
            transition(
                border_color,
                AnimationParameters::default()
                    .with_duration(Duration::from_millis(SLIDER_BORDER_TRANSITION_MS))
                    .with_easing(EasingFunction::EaseOut)
                    .to_transition_options(),
                ctx,
            ),
        )))
        .with_border_radius(BorderRadius::all(slider.thumb_size / 2.0))
}

fn slider_root_height(slider: &Slider) -> f32 {
    slider.thumb_size.max(slider.track_height)
}

fn slider_track_y(slider: &Slider) -> f32 {
    (slider_root_height(slider) - slider.track_height) * 0.5
}
