mod metrics;
mod page_dots;
mod app_grid_viewport;
use crate::components::home::app_grid::metrics::AppGridMetrics;
use crate::components::home::model::{HOME_APP_GRID_PAGE_STATE_ID, SCREEN_PADDING, TILE_HEIGHT, TILE_WIDTH};
use daiko::component::{Component, ComponentContext};
use daiko::layout::{AlignItems, FlexDirection, ItemSize, JustifyContent};
use daiko::navigation::{FocusKey};
use daiko::style::{Overflow, Style};
use daiko::{Element, Id, Vec2};
use std::time::{Duration};
use crate::components::home::app_grid::app_grid_viewport::AppGridViewport;
use crate::components::home::app_grid::page_dots::PageDots;

const PAGE_DOTS_HEIGHT: f32 = 10.0;
const PAGE_DOTS_GAP: f32 = 8.0;
const PAGE_DOTS_TOP_GAP: f32 = 18.0;
const PAGE_DOT_SIZE: f32 = 8.0;
const PAGE_DOT_FOCUS_PADDING: f32 = 2.0;
const PAGE_DOT_FOCUS_BORDER_WIDTH: f32 = 2.0;
const ACTIVE_PAGE_DOT_WIDTH: f32 = 22.0;
const PAGE_SCROLL_THRESHOLD: f32 = 8.0;
const PAGE_SCROLL_REARM_DURATION: Duration = Duration::from_millis(220);

fn page_dot_focus_key(page_index: usize) -> FocusKey {
    FocusKey::new(format!("apps-grid-page-dot-{page_index}"))
}

#[derive(Clone)]
pub(super) struct AppGrid {
    pub interactions_disabled: bool,
    pub hidden_app_id: Option<&'static str>,
    pub preferred_focus_app_id: Option<&'static str>,
}

fn app_grid_wrapper_style() -> Style {
    Style::new()
        .with_direction(FlexDirection::Column)
        .with_justify_content(JustifyContent::Center)
        .with_align_items(AlignItems::Center)
        .with_grow(1.0)
        .with_overflow(Overflow::Visible)
}

impl Component for AppGrid {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let wrapper_size = ctx
            .layout()
            .map(|layout| layout.size)
            .filter(|size| size.x >= TILE_WIDTH && size.y >= TILE_HEIGHT);

        Element::new()
            .with_tag("apps-grid")
            .with_style(app_grid_wrapper_style())
            .with_content(AppGridPager {
                grid: self.clone(),
                wrapper_size,
            })
    }
}

#[derive(Clone)]
struct AppGridPager {
    grid: AppGrid,
    wrapper_size: Option<Vec2>,
}

impl Component for AppGridPager {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let metrics = AppGridMetrics::from_wrapper_size(self.wrapper_size.unwrap_or_else(|| {
            Vec2::new(
                TILE_WIDTH + SCREEN_PADDING * 2.0,
                TILE_HEIGHT + PAGE_DOTS_HEIGHT + PAGE_DOTS_TOP_GAP,
            )
        }));
        let page_state = ctx.use_shared_state(Id::new(HOME_APP_GRID_PAGE_STATE_ID), || 0);
        let active_page = (*page_state.read()).min(metrics.last_page_index());

        Element::new()
            .with_tag("apps-grid-pager")
            .with_style(
                Style::new()
                    .with_direction(FlexDirection::Column)
                    .with_justify_content(JustifyContent::Center)
                    .with_align_items(AlignItems::Center)
                    .with_spacing((PAGE_DOTS_TOP_GAP, PAGE_DOTS_TOP_GAP))
                    .with_fixed_width(ItemSize::Points(metrics.page_width)),
            )
            .with_content(AppGridViewport {
                grid: self.grid.clone(),
                metrics,
                animate: self.wrapper_size.is_some(),
            })
            .with_content(PageDots {
                page_count: metrics.page_count,
                active_page,
                interactions_disabled: self.grid.interactions_disabled,
            })
    }
}