use super::super::style::{
    PANEL_RADIUS, SETTINGS_MENU_GAP, SETTINGS_MENU_PADDING, SETTINGS_MENU_WIDTH,
    SETTINGS_TOP_ACTIONS_GAP,
};
use daiko::layout::{AlignItems, FlexDirection, ItemSize, JustifyContent, SizeConstraint};
use daiko::style::{Border, BorderRadius, Color, Stroke, Style};

pub(crate) fn settings_menu_style(max_height: f32) -> Style {
    Style::new()
        .with_size_constraint(SizeConstraint::exact_content_height().with_max_height(max_height))
        .with_fixed_width(ItemSize::Points(SETTINGS_MENU_WIDTH))
        .with_padding(SETTINGS_MENU_PADDING)
        .with_direction(FlexDirection::Column)
        .with_spacing((SETTINGS_MENU_GAP, SETTINGS_MENU_GAP))
        .with_background_color(Color::from_rgb(12, 16, 18))
        .with_border(Border::uniform(Stroke::new(
            1.0,
            Color::from_rgba_unmultiplied(255, 255, 255, 42),
        )))
        .with_border_radius(BorderRadius::all(PANEL_RADIUS))
}

pub(crate) fn settings_scrollable_style() -> Style {
    Style::new()
}

pub(crate) fn settings_top_row_style() -> Style {
    Style::new()
        .with_size_constraint(SizeConstraint::exact_content_height())
        .with_fixed_width(ItemSize::Percent(1.0))
        .with_direction(FlexDirection::Row)
        .with_align_items(AlignItems::Center)
        .with_justify_content(JustifyContent::SpaceBetween)
}

pub(crate) fn settings_top_actions_style() -> Style {
    Style::new()
        .with_direction(FlexDirection::Row)
        .with_align_items(AlignItems::Center)
        .with_justify_content(JustifyContent::FlexEnd)
        .with_spacing((SETTINGS_TOP_ACTIONS_GAP, SETTINGS_TOP_ACTIONS_GAP))
}

pub(crate) fn settings_tile_grid_style() -> Style {
    Style::new()
        .with_size_constraint(SizeConstraint::exact_content_height())
        .with_direction(FlexDirection::Column)
        .with_spacing((SETTINGS_MENU_GAP, SETTINGS_MENU_GAP))
}

pub(crate) fn settings_tile_row_style() -> Style {
    Style::new()
        .with_size_constraint(SizeConstraint::exact_content_height())
        .with_direction(FlexDirection::Row)
        .with_justify_content(JustifyContent::SpaceBetween)
        .with_align_items(AlignItems::Center)
}
