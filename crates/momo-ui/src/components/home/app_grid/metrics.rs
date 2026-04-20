use crate::components::home::app_grid::{PAGE_DOTS_HEIGHT, PAGE_DOTS_TOP_GAP};
use crate::components::home::model::{
    GRID_GAP, MOCK_APPS, SCREEN_PADDING, TILE_HEIGHT, TILE_WIDTH, columns_for_width,
    rows_for_height,
};
use daiko::Vec2;

fn grid_axis_size(item_count: usize, item_size: f32) -> f32 {
    item_count as f32 * item_size + item_count as f32 * GRID_GAP
}

#[derive(Clone, Copy)]
pub(in crate::components::home::app_grid) struct AppGridMetrics {
    pub(crate) page_width: f32,
    pub(crate) page_height: f32,
    pub(crate) row_width: f32,
    pub(crate) columns: usize,
    pub(crate) rows: usize,
    pub(crate) tiles_per_page: usize,
    pub(crate) page_count: usize,
}

impl AppGridMetrics {
    pub(crate) fn from_wrapper_size(wrapper_size: Vec2) -> Self {
        let page_width = wrapper_size.x.max(TILE_WIDTH);
        let available_width = (page_width - SCREEN_PADDING * 2.0).max(TILE_WIDTH);
        let available_grid_height =
            (wrapper_size.y - PAGE_DOTS_HEIGHT - PAGE_DOTS_TOP_GAP).max(TILE_HEIGHT);
        let columns = columns_for_width(available_width);
        let rows = rows_for_height(available_grid_height);
        let tiles_per_page = (columns * rows).max(1);
        let page_count = MOCK_APPS.len().div_ceil(tiles_per_page).max(1);
        let row_width = grid_axis_size(columns, TILE_WIDTH);
        let page_height = grid_axis_size(rows, TILE_HEIGHT);

        Self {
            page_width,
            page_height,
            row_width,
            columns,
            rows,
            tiles_per_page,
            page_count,
        }
    }

    pub(crate) fn last_page_index(self) -> usize {
        self.page_count.saturating_sub(1)
    }

    pub(crate) fn offset_page(self, page_index: usize, delta: isize) -> usize {
        if delta.is_negative() {
            page_index.saturating_sub(delta.unsigned_abs())
        } else {
            page_index
                .saturating_add(delta as usize)
                .min(self.last_page_index())
        }
    }
}
