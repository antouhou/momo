use super::state::{SETTINGS_MENU_STATE_ID, SettingsMenuState, SettingsMenuViewType};
use super::style::{SETTINGS_MENU_VERTICAL_PADDING, settings_inset_section_style};
use daiko::Element;
use daiko::Id;
use daiko::component::{Component, ComponentContext};
use daiko::style::{Color, Style};
use daiko::widgets::image::{Image, ImageParams, ImageSource, ImageType};

#[derive(Clone, Copy)]
pub(super) struct QuickSettingsControlState {
    pub(super) is_hovered: bool,
    pub(super) is_focused: bool,
}

impl QuickSettingsControlState {
    pub(super) fn is_highlighted(self) -> bool {
        self.is_hovered || self.is_focused
    }
}

#[derive(Clone, Copy)]
pub(super) enum QuickSettingsGlyph {
    Asset(&'static [u8]),
}

pub(super) fn glyph_element(
    glyph: QuickSettingsGlyph,
    icon_size: usize,
    frame_size: f32,
    icon_color: Color,
) -> Element {
    match glyph {
        QuickSettingsGlyph::Asset(svg) => centered_glyph_frame(frame_size).with_content(
            Image::new(ImageParams {
                max_width: icon_size,
                max_height: icon_size,
                image_type: Some(ImageType::Svg),
                source: ImageSource::BytesSlice(svg),
            })
            .fill_color(Some(icon_color)),
        ),
    }
}

pub(super) fn settings_row(content: impl Component) -> Element {
    settings_row_with_padding(content, SETTINGS_MENU_VERTICAL_PADDING, 0.0)
}

pub(super) fn settings_middle_row(content: impl Component) -> Element {
    settings_row_with_padding(content, 0.0, 0.0)
}

pub(super) fn settings_bottom_row(content: impl Component) -> Element {
    settings_row_with_padding(content, 0.0, SETTINGS_MENU_VERTICAL_PADDING)
}

fn settings_row_with_padding(
    content: impl Component,
    top_padding: f32,
    bottom_padding: f32,
) -> Element {
    Element::new()
        .with_style(settings_inset_section_style(top_padding, bottom_padding))
        .with_content(content)
}

pub(super) fn is_menu_view_active(ctx: &mut ComponentContext, view: SettingsMenuViewType) -> bool {
    let state = ctx.use_shared_state(Id::new(SETTINGS_MENU_STATE_ID), SettingsMenuState::default);
    state.read().active_view == view
}

fn centered_glyph_frame(size: f32) -> Element {
    Element::new().with_style(
        Style::new()
            .with_fixed_size(size, size)
            .with_direction(daiko::layout::FlexDirection::Row)
            .with_align_items(daiko::layout::AlignItems::Center)
            .with_justify_content(daiko::layout::JustifyContent::Center),
    )
}
