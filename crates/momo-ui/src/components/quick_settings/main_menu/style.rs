use super::super::style::{
    SETTINGS_MENU_CONTENT_WIDTH, SETTINGS_MENU_GAP, SETTINGS_MENU_HORIZONTAL_PADDING,
    SETTINGS_MENU_VERTICAL_PADDING, SETTINGS_SCROLLABLE_FOCUS_PADDING, SETTINGS_TOP_ACTIONS_GAP,
};
use daiko::layout::{AlignItems, FlexDirection, ItemSize, JustifyContent, SizeConstraint};
use daiko::style::{Indent, Style};

pub(super) fn settings_top_row_style() -> Style {
    Style::new()
        .with_size_constraint(SizeConstraint::exact_content_height())
        .with_fixed_width(ItemSize::Percent(1.0))
        .with_direction(FlexDirection::Row)
        .with_align_items(AlignItems::Center)
        .with_justify_content(JustifyContent::SpaceBetween)
}

pub(super) fn settings_top_actions_style() -> Style {
    Style::new()
        .with_direction(FlexDirection::Row)
        .with_align_items(AlignItems::Center)
        .with_justify_content(JustifyContent::FlexEnd)
        .with_spacing((SETTINGS_TOP_ACTIONS_GAP, SETTINGS_TOP_ACTIONS_GAP))
}

pub(super) fn settings_tile_grid_style() -> Style {
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

pub(super) fn settings_tile_row_style() -> Style {
    Style::new()
        .with_size_constraint(SizeConstraint::exact_content_height())
        .with_direction(FlexDirection::Row)
        .with_justify_content(JustifyContent::SpaceBetween)
        .with_align_items(AlignItems::Center)
}
