mod app_grid_viewport;
mod metrics;
mod page_dots;
mod state;
mod style;

use crate::{
    app_state::use_apps_state,
    components::home::model::{SCREEN_PADDING, TILE_HEIGHT, TILE_WIDTH},
};
use app_grid_viewport::AppGridViewport;
use daiko::{
    Element, Vec2,
    component::{Component, ComponentContext},
    navigation::FocusKey,
};
use metrics::AppGridMetrics;
pub(crate) use page_dots::PageDots;
use state::app_grid_state_handle;
use std::{sync::Arc, time::Duration};
use style::{
    ACTIVE_PAGE_DOT_WIDTH, PAGE_DOT_FOCUS_BORDER_WIDTH, PAGE_DOT_FOCUS_PADDING, PAGE_DOT_SIZE,
    PAGE_DOTS_GAP, PAGE_DOTS_HEIGHT, PAGE_DOTS_TOP_GAP, app_grid_pager_style,
    app_grid_wrapper_style,
};
const PAGE_SCROLL_THRESHOLD: f32 = 8.0;
const PAGE_SCROLL_REARM_DURATION: Duration = Duration::from_millis(220);

pub(in crate::components::home::app_grid) fn page_dot_focus_key(page_index: usize) -> FocusKey {
    FocusKey::new(format!("apps-grid-page-dot-{page_index}"))
}

#[derive(Clone)]
pub(super) struct AppGrid {
    pub interactions_disabled: bool,
    pub hidden_app_id: Option<Arc<String>>,
    pub preferred_focus_app_id: Option<Arc<String>>,
    pub prefer_first_tile: bool,
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
        let apps_handle = use_apps_state(ctx);
        let apps = apps_handle.read();
        let metrics = AppGridMetrics::from_wrapper_size(
            self.wrapper_size.unwrap_or_else(|| {
                Vec2::new(
                    TILE_WIDTH + SCREEN_PADDING * 2.0,
                    TILE_HEIGHT + PAGE_DOTS_HEIGHT + PAGE_DOTS_TOP_GAP,
                )
            }),
            apps.app_entries.len(),
        );

        let app_grid_state = app_grid_state_handle(ctx);
        let (state_changed, active_page) = {
            let guard = app_grid_state.read();
            let active_page = guard.active_page.min(metrics.last_page_index());
            let state_changed =
                guard.active_page != active_page || guard.page_count != metrics.page_count;
            (state_changed, active_page)
        };
        if state_changed {
            let mut guard = app_grid_state.write();
            guard.active_page = active_page;
            guard.page_count = metrics.page_count;
        }

        Element::new()
            .with_tag("apps-grid-pager")
            .with_style(app_grid_pager_style(metrics.page_width))
            .with_content(AppGridViewport {
                grid: self.grid.clone(),
                metrics,
                animate: self.wrapper_size.is_some(),
            })
    }
}
