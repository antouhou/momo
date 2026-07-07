use daiko::{
    layout::{AlignItems, FlexDirection, ItemSize, JustifyContent},
    style::{Overflow, Style},
};

pub(super) const PAGE_DOTS_HEIGHT: f32 = 10.0;
pub(super) const PAGE_DOTS_GAP: f32 = 8.0;
pub(super) const PAGE_DOTS_TOP_GAP: f32 = 18.0;
pub(super) const PAGE_DOT_SIZE: f32 = 8.0;
pub(super) const PAGE_DOT_FOCUS_PADDING: f32 = 2.0;
pub(super) const PAGE_DOT_FOCUS_BORDER_WIDTH: f32 = 2.0;
pub(super) const ACTIVE_PAGE_DOT_WIDTH: f32 = 22.0;

pub(super) fn app_grid_wrapper_style() -> Style {
    Style::new()
        .with_direction(FlexDirection::Column)
        .with_justify_content(JustifyContent::Center)
        .with_align_items(AlignItems::Center)
        .with_grow(1.0)
        .with_fixed_width(ItemSize::Percent(1.0))
        .with_min_height(ItemSize::Points(0.0))
        .with_overflow(Overflow::Visible)
}

pub(super) fn app_grid_pager_style(page_width: f32) -> Style {
    Style::new()
        .with_direction(FlexDirection::Column)
        .with_justify_content(JustifyContent::Center)
        .with_align_items(AlignItems::Center)
        .with_spacing((PAGE_DOTS_TOP_GAP, PAGE_DOTS_TOP_GAP))
        .with_fixed_width(ItemSize::Points(page_width))
}
