use super::{
    state::{SETTINGS_MENU_STATE_ID, SettingsMenuState, SettingsMenuViewType},
    style::{SETTINGS_MENU_VERTICAL_PADDING, settings_inset_section_style},
};
use daiko::{
    Element, Id,
    component::{Component, ComponentContext},
    style::Color,
    widgets::image::Image,
};
use momo_kit::components::svg_icon;

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

impl QuickSettingsGlyph {
    pub(super) fn svg(self) -> &'static [u8] {
        match self {
            Self::Asset(svg) => svg,
        }
    }
}

pub(super) fn glyph_image(glyph: QuickSettingsGlyph, icon_size: usize, icon_color: Color) -> Image {
    svg_icon(glyph.svg(), icon_size, icon_color)
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
