use super::super::style::{
    PANEL_RADIUS, SETTINGS_MENU_CONTENT_WIDTH, SETTINGS_MENU_GAP,
    SETTINGS_MENU_HORIZONTAL_PADDING, SETTINGS_MENU_VERTICAL_PADDING, SETTINGS_MENU_WIDTH,
    SETTINGS_PANEL_BORDER_WIDTH,
    SETTINGS_SCROLLABLE_FOCUS_PADDING, SETTINGS_TOP_ACTIONS_GAP, settings_panel_border_color,
    settings_panel_color,
};
use daiko::layout::{AlignItems, FlexDirection, ItemSize, JustifyContent, SizeConstraint};
use daiko::style::{Border, BorderRadius, Indent, Stroke, Style};

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
}

pub(crate) fn settings_content_style() -> Style {
    Style::new()
        .with_size_constraint(
            SizeConstraint::exact_content_height().with_exact_width(SETTINGS_MENU_CONTENT_WIDTH),
        )
        .with_direction(FlexDirection::Column)
        .with_spacing((SETTINGS_MENU_GAP, SETTINGS_MENU_GAP))
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
        .with_size_constraint(
            SizeConstraint::exact_content_height().with_exact_width(SETTINGS_MENU_CONTENT_WIDTH),
        )
        .with_direction(FlexDirection::Column)
        .with_padding(Indent::new(
            SETTINGS_MENU_HORIZONTAL_PADDING,
            SETTINGS_SCROLLABLE_FOCUS_PADDING,
            SETTINGS_MENU_HORIZONTAL_PADDING,
            SETTINGS_MENU_VERTICAL_PADDING,
        ))
        .with_spacing((SETTINGS_MENU_GAP, SETTINGS_MENU_GAP))
}

pub(crate) fn settings_tile_row_style() -> Style {
    Style::new()
        .with_size_constraint(SizeConstraint::exact_content_height())
        .with_direction(FlexDirection::Row)
        .with_justify_content(JustifyContent::SpaceBetween)
        .with_align_items(AlignItems::Center)
}
