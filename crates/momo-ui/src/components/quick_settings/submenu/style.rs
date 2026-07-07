use daiko::{
    layout::{AlignItems, FlexDirection, ItemSize, JustifyContent, SizeConstraint},
    style::{Indent, Style},
    widgets::text::TextStyle,
};

use super::super::style::{
    SETTINGS_COMPACT_CONTENT_GAP, SETTINGS_MENU_CONTENT_WIDTH, SETTINGS_MENU_GAP,
    SETTINGS_MENU_HORIZONTAL_PADDING, SETTINGS_SCROLLABLE_FOCUS_PADDING,
    SETTINGS_SUBMENU_SECTION_LABEL_HEIGHT, SETTINGS_SUBMENU_SECTION_PADDING,
    SETTINGS_SUBMENU_SECTION_TITLE_TEXT_SIZE, settings_label_text_style,
    settings_surface_muted_color,
};

pub(in crate::components::quick_settings) fn submenu_body_style() -> Style {
    Style::new()
        .with_size_constraint(
            SizeConstraint::exact_content_height().with_exact_width(SETTINGS_MENU_CONTENT_WIDTH),
        )
        .with_direction(FlexDirection::Column)
        .with_padding(Indent::new(
            SETTINGS_MENU_HORIZONTAL_PADDING,
            SETTINGS_SCROLLABLE_FOCUS_PADDING,
            SETTINGS_MENU_HORIZONTAL_PADDING,
            SETTINGS_SCROLLABLE_FOCUS_PADDING,
        ))
        .with_spacing((SETTINGS_MENU_GAP, SETTINGS_MENU_GAP))
}

pub(in crate::components::quick_settings) fn submenu_section_style() -> Style {
    Style::new()
        .with_direction(FlexDirection::Column)
        .with_spacing((SETTINGS_COMPACT_CONTENT_GAP, SETTINGS_COMPACT_CONTENT_GAP))
}

pub(in crate::components::quick_settings) fn submenu_section_label_style() -> Style {
    Style::new()
        .with_padding(SETTINGS_SUBMENU_SECTION_PADDING)
        .with_fixed_height(ItemSize::Points(SETTINGS_SUBMENU_SECTION_LABEL_HEIGHT))
        .with_direction(FlexDirection::Row)
        .with_align_items(AlignItems::Center)
        .with_justify_content(JustifyContent::FlexStart)
}

pub(in crate::components::quick_settings) fn submenu_section_title_style() -> TextStyle {
    settings_label_text_style(settings_surface_muted_color())
        .with_font_size(SETTINGS_SUBMENU_SECTION_TITLE_TEXT_SIZE)
}
