use crate::{
    app_state::{AppEntry, use_apps_state},
    components::home::{
        app_grid::{
            AppGrid, PAGE_SCROLL_REARM_DURATION, PAGE_SCROLL_THRESHOLD, metrics::AppGridMetrics,
            state::app_grid_state_handle,
        },
        app_tile::{AppInfo, AppTile},
        model::{
            GRID_GAP, HOME_APP_GRID_FOCUSED_KEY_ID, HOME_APP_GRID_SCROLL_ACCUMULATOR_ID,
            HOME_APP_GRID_SMOOTH_OFFSET_ID,
        },
    },
};
use daiko::{
    Element, Id, Vec2,
    animation::SmoothFollowConfig,
    component::{Component, ComponentContext},
    layout::{AlignItems, FlexDirection, JustifyContent},
    navigation::{FocusEntryPolicy, FocusKey, TraversalPolicy},
    style::{Overflow, Style},
    widgets::container::{Container, Fit},
};
use std::time::{Duration, Instant};

#[derive(Clone)]
pub(in crate::components::home::app_grid) struct AppGridViewport {
    pub(crate) grid: AppGrid,
    pub(crate) metrics: AppGridMetrics,
    pub(crate) animate: bool,
}

impl Component for AppGridViewport {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let focus_scope = ctx.focus_scope();
        focus_scope.set_entry_policy(FocusEntryPolicy::Spatial(
            TraversalPolicy::NavigationDirectionDistance,
        ));
        let last_focused_key =
            ctx.use_local_state_with_id(Id::new(HOME_APP_GRID_FOCUSED_KEY_ID), || None::<FocusKey>);
        let page_state = app_grid_state_handle(ctx);

        let apps_handle = use_apps_state(ctx);
        let apps_state = apps_handle.read();
        let apps = &apps_state.app_entries;

        let mut target_page = (page_state.read().active_page).min(self.metrics.last_page_index());
        let focused_key = focus_scope.focused_child_key();
        if focused_key != *last_focused_key.read() {
            *last_focused_key.write_silent() = focused_key;
            if let Some(focused_page) =
                focused_page_index(focused_key, self.metrics.tiles_per_page, apps)
            {
                target_page = focused_page.min(self.metrics.last_page_index());
            }
        }

        if let Some(page_delta) = scroll_page_delta(ctx) {
            target_page = self
                .metrics
                .offset_page(target_page, page_delta)
                .min(self.metrics.last_page_index());
        }

        if page_state.read().active_page != target_page {
            page_state.write().active_page = target_page;
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
                    .with_overflow(Overflow::Visible),
            )
            .with_content(build_page_strip(
                &self.grid,
                self.metrics,
                rendered_offset,
                apps,
                page_state.read().active_page,
            ))
    }
}

#[derive(Default)]
struct AppGridScrollState {
    accumulated_delta: f32,
    locked_until: Option<Instant>,
}

fn focused_page_index(
    focused_key: Option<FocusKey>,
    tiles_per_page: usize,
    apps: &[AppEntry],
) -> Option<usize> {
    let focused_key = focused_key?;
    apps.iter()
        .position(|app| FocusKey::new(app.id()) == focused_key)
        .map(|app_index| app_index / tiles_per_page)
}

fn scroll_page_delta(ctx: &mut ComponentContext) -> Option<isize> {
    let scroll_state = ctx.use_local_state_with_id(
        Id::new(HOME_APP_GRID_SCROLL_ACCUMULATOR_ID),
        AppGridScrollState::default,
    );
    let scroll_delta = ctx.consume_scroll()?;

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

fn page_delta_for_scroll(accumulated_delta: f32) -> Option<isize> {
    // Scroll down to get to the next page, scroll up to get to the previous one
    if accumulated_delta <= -PAGE_SCROLL_THRESHOLD {
        Some(-1)
    } else if accumulated_delta >= PAGE_SCROLL_THRESHOLD {
        Some(1)
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

fn build_page_strip(
    grid: &AppGrid,
    metrics: AppGridMetrics,
    rendered_offset: f32,
    apps: &[AppEntry],
    active_page: usize,
) -> Element {
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
            // .with_transform(Some(Transform::new().then_translate(rendered_offset, 0.0)))
            .with_overflow(Overflow::Visible),
    );

    let empty_page = Element::new().with_style(
        Style::new()
            .with_fixed_size(metrics.page_width, metrics.page_height)
            .with_overflow(Overflow::Visible),
    );

    for page_index in 0..metrics.page_count {
        let is_active_or_overscan = page_index.abs_diff(active_page) <= 1;
        if is_active_or_overscan {
            page_strip.add_content(build_page_contents(grid, metrics, page_index, apps));
        } else {
            page_strip.add_content(empty_page.clone());
        }
    }

    page_strip
}

pub(in crate::components::home::app_grid) fn build_page_contents(
    grid: &AppGrid,
    metrics: AppGridMetrics,
    page_index: usize,
    apps: &[AppEntry],
) -> Element {
    let first_app_index = page_index * metrics.tiles_per_page;
    let page_app_count = apps
        .len()
        .saturating_sub(first_app_index)
        .min(metrics.tiles_per_page);
    let page_apps = &apps[first_app_index..first_app_index + page_app_count];

    let mut page = Container::vertical()
        .with_fit(
            Fit::new()
                .exact_width(metrics.page_width)
                .exact_height(metrics.page_height),
        )
        .align_items_center()
        .build()
        .with_tag(format!("apps-grid-page-{page_index}"));

    page.style_mut().set_overflow(Overflow::Visible);

    for (row_index, row) in page_apps.chunks(metrics.columns).enumerate() {
        let mut row_container = Container::horizontal()
            .with_padding(GRID_GAP / 2.0)
            .with_fit(
                Fit::new()
                    .exact_width(metrics.row_width)
                    .exact_content_height(),
            )
            .align_items_start()
            .with_spacing((GRID_GAP, GRID_GAP))
            .build();

        row_container.style_mut().set_overflow(Overflow::Visible);

        for (column_index, app) in row.iter().enumerate() {
            let app_index = first_app_index + row_index * metrics.columns + column_index;
            row_container.add_content(AppTile {
                app: AppInfo::new(app),
                preferred_focus: grid.preferred_focus_app_id.as_deref().map(String::as_str)
                    == Some(app.id())
                    || (grid.prefer_first_tile
                        && grid.preferred_focus_app_id.is_none()
                        && app_index == 0),
                interactions_disabled: grid.interactions_disabled,
                is_hidden_for_launch: grid.hidden_app_id.as_deref().map(String::as_str)
                    == Some(app.id()),
            });
        }

        page.add_content(row_container);
    }

    page
}
