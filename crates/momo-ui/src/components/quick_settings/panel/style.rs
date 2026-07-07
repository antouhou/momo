use super::super::style::{
    PANEL_RADIUS, SETTINGS_MENU_GAP, SETTINGS_MENU_WIDTH, SETTINGS_PANEL_BORDER_WIDTH,
    settings_panel_border_color, settings_panel_color,
};
use daiko::{
    layout::{AlignItems, FlexDirection, ItemSize, SizeConstraint},
    style::{Border, BorderRadius, Overflow, Stroke, Style},
};

pub(crate) fn settings_menu_style(max_height: f32) -> Style {
    Style::new()
        .with_size_constraint(SizeConstraint::exact_content_height().with_max_height(max_height))
        .with_fixed_width(ItemSize::Points(SETTINGS_MENU_WIDTH))
        .with_direction(FlexDirection::Column)
        .with_align_items(AlignItems::Center)
        .with_spacing((SETTINGS_MENU_GAP, SETTINGS_MENU_GAP))
        .with_background_color(settings_panel_color())
        .with_border(Border::uniform(Stroke::new(
            SETTINGS_PANEL_BORDER_WIDTH,
            settings_panel_border_color(),
        )))
        .with_border_radius(BorderRadius::all(PANEL_RADIUS))
        .with_overflow(Overflow::Hidden)
}
