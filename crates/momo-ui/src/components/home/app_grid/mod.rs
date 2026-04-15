mod metrics;

use crate::components::home::app_tile::AppTile;
use crate::components::home::model::{
    GRID_GAP, HOME_APP_GRID_FOCUSED_KEY_ID, HOME_APP_GRID_PAGE_STATE_ID,
    HOME_APP_GRID_SCROLL_ACCUMULATOR_ID, HOME_APP_GRID_SMOOTH_OFFSET_ID, MOCK_APPS, SCREEN_PADDING,
    TILE_HEIGHT, TILE_WIDTH, columns_for_width, rows_for_height,
};
use daiko::animation::SmoothFollowConfig;
use daiko::component::{Component, ComponentContext};
use daiko::layout::{AlignItems, FlexDirection, ItemSize, JustifyContent, Layout};
use daiko::navigation::{FocusBoundary, FocusKey, FocusOrigin};
use daiko::style::{Border, BorderRadius, Color, Overflow, Stroke, Style};
use daiko::widgets::container::{Container, Fit};
use daiko::{Element, Id, Vec2};
use std::time::{Duration, Instant};
use crate::components::home::app_grid::metrics::AppGridMetrics;

const PAGE_DOTS_HEIGHT: f32 = 10.0;
const PAGE_DOTS_GAP: f32 = 8.0;
const PAGE_DOTS_TOP_GAP: f32 = 18.0;
const PAGE_DOT_SIZE: f32 = 8.0;
const PAGE_DOT_FOCUS_PADDING: f32 = 2.0;
const PAGE_DOT_FOCUS_BORDER_WIDTH: f32 = 2.0;
const ACTIVE_PAGE_DOT_WIDTH: f32 = 22.0;
const PAGE_SCROLL_THRESHOLD: f32 = 8.0;
const PAGE_SCROLL_REARM_DURATION: Duration = Duration::from_millis(220);

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
        let focus_scope = ctx.focus_scope();
        focus_scope.set_boundary(FocusBoundary::Stop);
        let last_focused_key =
            ctx.use_local_state_with_id(Id::new(HOME_APP_GRID_FOCUSED_KEY_ID), || None::<FocusKey>);

        let mut target_page = (*page_state.read()).min(metrics.last_page_index());
        let focused_key = focus_scope.focused_child_key();
        if focused_key != *last_focused_key.read() {
            *last_focused_key.write_silent() = focused_key;
            if let Some(focused_page) = focused_page_index(focused_key, metrics.tiles_per_page) {
                target_page = focused_page.min(metrics.last_page_index());
            }
        }

        if *page_state.read() != target_page {
            *page_state.write() = target_page;
        }

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
                active_page: target_page,
                interactions_disabled: self.grid.interactions_disabled,
            })
    }
}

#[derive(Clone)]
struct AppGridViewport {
    grid: AppGrid,
    metrics: AppGridMetrics,
    animate: bool,
}

impl Component for AppGridViewport {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let viewport_layout = ctx.layout();
        let page_state = ctx.use_shared_state(Id::new(HOME_APP_GRID_PAGE_STATE_ID), || 0);
        if let Some(page_delta) = scroll_page_delta(ctx, viewport_layout) {
            let target_page = self
                .metrics
                .offset_page(*page_state.read(), page_delta)
                .min(self.metrics.last_page_index());
            if *page_state.read() != target_page {
                *page_state.write() = target_page;
            }
        }

        let target_page = (*page_state.read()).min(self.metrics.last_page_index());
        if *page_state.read() != target_page {
            *page_state.write() = target_page;
        }

        let target_offset = -(target_page as f32) * self.metrics.page_width;
        let mut smooth_offset = ctx.smooth_follow_with_id::<f32>(
            Id::new(HOME_APP_GRID_SMOOTH_OFFSET_ID),
            SmoothFollowConfig::new(Duration::from_millis(180), 0.3, 0.36),
        );
        let rendered_offset = if self.animate {
            smooth_offset.follow(target_offset)
        } else {
            smooth_offset.reset_to(target_offset);
            target_offset
        };

        Element::new()
            .with_tag("apps-grid-viewport")
            .with_style(
                Style::new()
                    .with_fixed_size(self.metrics.page_width, self.metrics.page_height)
                    .with_overflow(Overflow::Hidden),
            )
            .with_content(build_page_strip(&self.grid, self.metrics, rendered_offset))
    }
}

#[derive(Default)]
struct AppGridScrollState {
    accumulated_delta: f32,
    locked_until: Option<Instant>,
}

fn focused_page_index(focused_key: Option<FocusKey>, tiles_per_page: usize) -> Option<usize> {
    let focused_key = focused_key?;
    MOCK_APPS
        .iter()
        .position(|app| FocusKey::new(app.id) == focused_key)
        .map(|app_index| app_index / tiles_per_page)
}

fn page_dot_focus_key(page_index: usize) -> FocusKey {
    FocusKey::new(format!("apps-grid-page-dot-{page_index}"))
}

fn scroll_page_delta(ctx: &mut ComponentContext, viewport_layout: Option<Layout>) -> Option<isize> {
    let scroll_state = ctx.use_local_state_with_id(
        Id::new(HOME_APP_GRID_SCROLL_ACCUMULATOR_ID),
        AppGridScrollState::default,
    );
    let scroll_delta = ctx.scroll()?;
    if !pointer_is_inside_layout(ctx, viewport_layout) {
        return None;
    }

    let scroll_axis_delta = scroll_axis_delta(scroll_delta);
    if scroll_axis_delta.abs() <= f32::EPSILON {
        return None;
    }

    let now = Instant::now();
    let mut scroll_state = scroll_state.write_silent();
    if scroll_state
        .locked_until
        .is_some_and(|locked_until| now < locked_until)
    {
        scroll_state.accumulated_delta = 0.0;
        return None;
    }

    scroll_state.locked_until = None;
    scroll_state.accumulated_delta += scroll_axis_delta;
    let page_delta = page_delta_for_scroll(scroll_state.accumulated_delta);
    if page_delta.is_some() {
        scroll_state.accumulated_delta = 0.0;
        scroll_state.locked_until = Some(now + PAGE_SCROLL_REARM_DURATION);
    }
    page_delta
}

fn pointer_is_inside_layout(ctx: &mut ComponentContext, viewport_layout: Option<Layout>) -> bool {
    let Some(layout) = viewport_layout else {
        return false;
    };
    let Some(pointer_position) = ctx.app_context.input_state().pointer.interact_position() else {
        return false;
    };
    let visible_area = layout.visible_area;

    pointer_position.x >= visible_area.min.x
        && pointer_position.x <= visible_area.max.x
        && pointer_position.y >= visible_area.min.y
        && pointer_position.y <= visible_area.max.y
}

fn page_delta_for_scroll(accumulated_delta: f32) -> Option<isize> {
    if accumulated_delta <= -PAGE_SCROLL_THRESHOLD {
        Some(1)
    } else if accumulated_delta >= PAGE_SCROLL_THRESHOLD {
        Some(-1)
    } else {
        None
    }
}

fn scroll_axis_delta(scroll_delta: Vec2) -> f32 {
    if scroll_delta.y.abs() > f32::EPSILON {
        scroll_delta.y
    } else {
        scroll_delta.x
    }
}

fn build_page_strip(grid: &AppGrid, metrics: AppGridMetrics, rendered_offset: f32) -> Element {
    let mut page_strip = Element::new().with_tag("apps-grid-page-strip").with_style(
        Style::new()
            .with_fixed_size(
                metrics.page_width * metrics.page_count as f32,
                metrics.page_height,
            )
            .with_direction(FlexDirection::Row)
            .with_align_items(AlignItems::FlexStart)
            .with_justify_content(JustifyContent::FlexStart)
            .with_absolute_position(Vec2::new(rendered_offset, 0.0))
            .with_overflow(Overflow::Visible),
    );

    for page_index in 0..metrics.page_count {
        page_strip.add_content(build_page(grid, metrics, page_index));
    }

    page_strip
}

fn build_page(grid: &AppGrid, metrics: AppGridMetrics, page_index: usize) -> Element {
    let first_app_index = page_index * metrics.tiles_per_page;
    let page_app_count = MOCK_APPS
        .len()
        .saturating_sub(first_app_index)
        .min(metrics.tiles_per_page);
    let page_apps = &MOCK_APPS[first_app_index..first_app_index + page_app_count];

    let mut page = Container::vertical()
        .with_fit(
            Fit::new()
                .exact_width(metrics.page_width)
                .exact_height(metrics.page_height),
        )
        .align_items_center()
        .with_spacing((GRID_GAP, GRID_GAP))
        .build()
        .with_tag(format!("apps-grid-page-{page_index}"));

    for (row_index, row) in page_apps.chunks(metrics.columns).enumerate() {
        let mut row_container = Container::horizontal()
            .with_fit(
                Fit::new()
                    .exact_width(metrics.row_width)
                    .exact_height(TILE_HEIGHT),
            )
            .align_items_start()
            .with_spacing((GRID_GAP, GRID_GAP))
            .build();

        for (column_index, app) in row.iter().enumerate() {
            let app_index = first_app_index + row_index * metrics.columns + column_index;
            row_container.add_content(AppTile {
                app: *app,
                preferred_focus: grid.preferred_focus_app_id == Some(app.id)
                    || (grid.preferred_focus_app_id.is_none() && app_index == 0),
                interactions_disabled: grid.interactions_disabled,
                is_hidden_for_launch: grid.hidden_app_id == Some(app.id),
                focus_left_app_id: page_edge_focus_target(metrics, app_index, -1),
                focus_right_app_id: page_edge_focus_target(metrics, app_index, 1),
            });
        }

        page.add_content(row_container);
    }

    page
}

fn page_edge_focus_target(
    metrics: AppGridMetrics,
    app_index: usize,
    page_delta: isize,
) -> Option<&'static str> {
    let page_index = app_index / metrics.tiles_per_page;
    let index_in_page = app_index % metrics.tiles_per_page;
    let row_index = index_in_page / metrics.columns;
    let column_index = index_in_page % metrics.columns;
    let target_page_index = metrics.offset_page(page_index, page_delta);

    if target_page_index == page_index || row_index >= metrics.rows {
        return None;
    }

    let target_column_index = if page_delta.is_negative() {
        metrics.columns.saturating_sub(1)
    } else {
        0
    };
    let is_page_edge = if page_delta.is_negative() {
        column_index == 0
    } else {
        column_index + 1 == metrics.columns
    };

    if !is_page_edge {
        return None;
    }

    let target_app_index = target_page_index * metrics.tiles_per_page
        + row_index * metrics.columns
        + target_column_index;
    MOCK_APPS.get(target_app_index).map(|app| app.id)
}

#[derive(Clone, Copy)]
struct PageDots {
    page_count: usize,
    active_page: usize,
    interactions_disabled: bool,
}

impl Component for PageDots {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let focus_scope = ctx.focus_scope();
        focus_scope.set_boundary(FocusBoundary::Escape);
        focus_scope.set_default_focus(page_dot_focus_key(self.active_page));

        let mut dots = Container::horizontal()
            .with_fit(Fit::new().exact_content_size())
            .align_items_center()
            .with_spacing((PAGE_DOTS_GAP, PAGE_DOTS_GAP))
            .build()
            .with_tag("apps-grid-page-dots");

        for page_index in 0..self.page_count {
            dots.add_content(PageDot {
                page_index,
                is_active: page_index == self.active_page,
                interactions_disabled: self.interactions_disabled,
            });
        }

        dots
    }
}

#[derive(Clone, Copy)]
struct PageDot {
    page_index: usize,
    is_active: bool,
    interactions_disabled: bool,
}

impl Component for PageDot {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let mut pointer = ctx.pointer();
        let focusable = ctx.focusable();
        focusable.set_focus_key(page_dot_focus_key(self.page_index));
        focusable.set_navigation_enabled(!self.interactions_disabled);

        if !self.interactions_disabled && pointer.just_entered() {
            focusable.request_focus(FocusOrigin::Pointer);
        }

        let just_selected =
            !self.interactions_disabled && (pointer.just_pressed() || focusable.just_activated());
        if just_selected {
            *ctx.use_shared_state(Id::new(HOME_APP_GRID_PAGE_STATE_ID), || 0)
                .write() = self.page_index;
        }

        page_dot(
            self.page_index,
            self.is_active,
            focusable.is_focus_visible() || pointer.is_hovering(),
        )
    }
}

fn page_dot(page_index: usize, is_active: bool, show_border: bool) -> Element {
    let dot_width = page_dot_visual_width(is_active);
    Element::new()
        .with_tag(format!("apps-grid-page-dot-{page_index}"))
        .with_style(page_dot_target_style(show_border, dot_width))
        .with_content(page_dot_visual(page_index, is_active))
}

fn page_dot_target_style(show_border: bool, dot_width: f32) -> Style {
    let border_color = if show_border {
        Color::from_rgb(236, 246, 255)
    } else {
        Color::TRANSPARENT
    };
    let target_outset = (PAGE_DOT_FOCUS_PADDING + PAGE_DOT_FOCUS_BORDER_WIDTH) * 2.0;

    Style::new()
        .with_direction(FlexDirection::Row)
        .with_justify_content(JustifyContent::Center)
        .with_align_items(AlignItems::Center)
        .with_fixed_size(dot_width + target_outset, PAGE_DOT_SIZE + target_outset)
        .with_padding(PAGE_DOT_FOCUS_PADDING)
        .with_border(Border::uniform(Stroke::new(
            PAGE_DOT_FOCUS_BORDER_WIDTH,
            border_color,
        )))
        .with_border_radius(BorderRadius::all((PAGE_DOT_SIZE + target_outset) / 2.0))
}

fn page_dot_visual(page_index: usize, is_active: bool) -> Element {
    let width = page_dot_visual_width(is_active);
    let color = if is_active {
        Color::from_rgb(236, 246, 255)
    } else {
        Color::from_rgb(88, 105, 124)
    };

    Element::new()
        .with_tag(format!("apps-grid-page-dot-visual-{page_index}"))
        .with_style(
            Style::new()
                .with_fixed_size(width, PAGE_DOT_SIZE)
                .with_background_color(color)
                .with_border_radius(BorderRadius::all(PAGE_DOT_SIZE / 2.0)),
        )
}

fn page_dot_visual_width(is_active: bool) -> f32 {
    if is_active {
        ACTIVE_PAGE_DOT_WIDTH
    } else {
        PAGE_DOT_SIZE
    }
}
