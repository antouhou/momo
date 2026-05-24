mod style;

use self::style::{
    submenu_button_foreground_color, submenu_button_label_style, submenu_button_style,
    submenu_label_group_style, submenu_leading_slot_style, submenu_toggle_knob_style,
    submenu_toggle_switch_style,
};
use super::common::{QuickSettingsControlState, QuickSettingsGlyph, glyph_element};
use super::style::{SETTINGS_ICON_FRAME_SIZE, SETTINGS_ICON_SIZE};
use daiko::Element;
use daiko::component::{Component, ComponentContext};
use daiko::style::Color;
use daiko::widgets::text::Text;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum SubmenuButtonState {
    Enabled,
    Disabled,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum SubmenuButtonSurface {
    Standard,
    Emphasized,
}

pub(super) struct SubmenuButton {
    pub(super) tag: String,
    pub(super) label: String,
    pub(super) control: QuickSettingsControlState,
    pub(super) surface: SubmenuButtonSurface,
    pub(super) state: SubmenuButtonState,
    pub(super) leading: Element,
    pub(super) trailing: Option<Element>,
}

pub(super) fn submenu_button_glyph(glyph: QuickSettingsGlyph, icon_color: Color) -> Element {
    submenu_button_leading_slot(glyph_element(
        glyph,
        SETTINGS_ICON_SIZE,
        SETTINGS_ICON_FRAME_SIZE,
        icon_color,
    ))
}

pub(super) fn submenu_button_surface_glyph(
    glyph: QuickSettingsGlyph,
    surface: SubmenuButtonSurface,
    state: SubmenuButtonState,
) -> Element {
    submenu_button_glyph(glyph, submenu_button_foreground_color(surface, state))
}

pub(super) fn submenu_button_leading_slot(content: Element) -> Element {
    Element::new()
        .with_style(submenu_leading_slot_style())
        .with_content(content)
}

pub(super) fn submenu_toggle_switch(ctx: &mut ComponentContext, is_enabled: bool) -> Element {
    Element::new()
        .with_style(submenu_toggle_switch_style(ctx, is_enabled))
        .with_content(Element::new().with_style(submenu_toggle_knob_style(ctx, is_enabled)))
}

impl Component for SubmenuButton {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let mut button = Element::new()
            .with_tag(self.tag.clone())
            .with_style(submenu_button_style(
                self.control,
                ctx,
                self.surface,
                self.trailing.is_some(),
            ))
            .with_content(
                Element::new()
                    .with_style(submenu_label_group_style())
                    .with_content(self.leading.clone())
                    .with_content(
                        Text::new(self.label.clone())
                            .with_style(submenu_button_label_style(self.surface, self.state)),
                    ),
            );

        if let Some(trailing) = &self.trailing {
            button.add_content(trailing.clone());
        }

        button
    }
}
