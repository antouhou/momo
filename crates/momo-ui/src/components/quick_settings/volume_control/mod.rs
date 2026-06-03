mod style;

use self::style::{
    volume_control_style, volume_label_container_style, volume_label_style,
    volume_slider_row_style, volume_slider_track_style,
};
use super::common::is_menu_view_active;
use super::common::{QuickSettingsControlState, QuickSettingsGlyph, glyph_element};
use super::state::SettingsMenuViewType;
use super::style::{
    SETTINGS_ICON_FRAME_SIZE, SETTINGS_ICON_SIZE, SETTINGS_VOLUME_THUMB_SIZE,
    SETTINGS_VOLUME_TRACK_HEIGHT, settings_accent_color, settings_bright_surface_border_color,
    settings_bright_surface_color, settings_surface_muted_color, settings_text_color,
    settings_volume_thumb_border_color,
};
use crate::components::home::system_status::{volume_handle, volume_state};
use crate::components::slider::{Slider, clamp_slider_value};
use daiko::component::{Component, ComponentContext};
use daiko::navigation::{FocusOrigin, NavigationDirection, NavigationInputAction};
use daiko::widgets::text::Text;
use daiko::{Element, Id};
use tracing::warn;

const AUDIO_ICON: &[u8] = include_bytes!("../../../../assets/volume.svg");
const VOLUME_STEP: i16 = 10;
const VOLUME_MIN: u8 = 0;
const VOLUME_MAX: u8 = 100;

pub(super) const SETTINGS_VOLUME_CONTROL_TAG: &str = "header-settings-volume-control";
pub(crate) const SETTINGS_VOLUME_STATE_ID: &str = "header-settings-volume-value";
pub(crate) const SETTINGS_VOLUME_TRACK_TAG: &str = "header-settings-volume-track";

#[derive(Clone, Copy)]
pub(super) struct VolumeControl;

impl Component for VolumeControl {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let volume = volume_state(ctx).read().output_percentage;
        let slider_volume = ctx.use_shared_state(Id::new(SETTINGS_VOLUME_STATE_ID), move || volume);
        let current_slider_volume = *slider_volume.read();
        let mut pointer = ctx.pointer();
        let focusable = ctx.focusable();
        let is_active = is_menu_view_active(ctx, SettingsMenuViewType::Main);

        if current_slider_volume != volume {
            *slider_volume.write_silent() = volume;
        }

        focusable.set_navigation_enabled(is_active);

        if pointer.just_pressed() {
            focusable.request_focus(FocusOrigin::Pointer);
        }

        if is_active && focusable.just_focused() {
            focusable.engage();
        }

        focusable.capture_when_engaged(&[
            NavigationInputAction::Move(NavigationDirection::Left),
            NavigationInputAction::Move(NavigationDirection::Right),
        ]);

        let volume_delta =
            focusable
                .drain_captured_actions()
                .fold(0, |delta, action| match action {
                    NavigationInputAction::Move(NavigationDirection::Left) => delta - VOLUME_STEP,
                    NavigationInputAction::Move(NavigationDirection::Right) => delta + VOLUME_STEP,
                    _ => delta,
                });
        if volume_delta != 0 {
            let next_volume =
                clamp_slider_value(i16::from(volume) + volume_delta, VOLUME_MIN, VOLUME_MAX);
            request_volume_change(ctx, next_volume);
        }

        let state = QuickSettingsControlState {
            is_hovered: pointer.is_hovering(),
            is_focused: focusable.is_focused(),
        };

        Element::new()
            .with_tag(SETTINGS_VOLUME_CONTROL_TAG)
            .with_style(volume_control_style(state, ctx))
            .with_content(
                Element::new()
                    .with_style(volume_label_container_style())
                    .with_content(Text::new("Sound").with_style(volume_label_style())),
            )
            .with_content(volume_slider_row(state, volume))
    }
}

fn request_volume_change(ctx: &mut ComponentContext, volume: u8) {
    if let Err(error) = volume_handle(ctx).set_output_volume_percentage(volume) {
        warn!("failed to set output volume: {error:?}");
    }
}

fn volume_slider_row(state: QuickSettingsControlState, volume: u8) -> Element {
    Element::new()
        .with_style(volume_slider_row_style())
        .with_content(glyph_element(
            QuickSettingsGlyph::Asset(AUDIO_ICON),
            SETTINGS_ICON_SIZE,
            SETTINGS_ICON_FRAME_SIZE,
            settings_text_color(),
        ))
        .with_content(
            Element::new()
                .with_tag(SETTINGS_VOLUME_TRACK_TAG)
                .with_style(volume_slider_track_style())
                .with_content(
                    Slider::new(SETTINGS_VOLUME_STATE_ID)
                        .default_value(volume)
                        .on_change(request_volume_change)
                        .range(VOLUME_MIN, VOLUME_MAX)
                        .track_height(SETTINGS_VOLUME_TRACK_HEIGHT)
                        .thumb_size(SETTINGS_VOLUME_THUMB_SIZE)
                        .track_color(settings_surface_muted_color())
                        .fill_color(settings_accent_color())
                        .thumb_color(settings_bright_surface_color())
                        .thumb_border_colors(
                            settings_volume_thumb_border_color(),
                            settings_bright_surface_border_color(),
                        )
                        .highlighted(state.is_highlighted()),
                ),
        )
}
