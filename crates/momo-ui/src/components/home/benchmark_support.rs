use crate::app_state::{APPS_STATE_ID, AppEntry, AppsState};
use crate::components::home::app_grid::AppGrid;
use daiko::component::{Component, ComponentContext};
use daiko::layout::{AlignItems, FlexDirection, ItemSize};
use daiko::style::{Color, Style};
use daiko::{App, AppContext, Element, Id, Vec2};
use std::path::PathBuf;
use std::sync::Arc;

const DEFAULT_APP_COUNT: usize = 72;
const DEFAULT_GRID_WIDTH: f32 = 960.0;
const DEFAULT_GRID_HEIGHT: f32 = 500.0;

pub struct AppGridBenchmarkApp {
    app_count: usize,
}

impl Default for AppGridBenchmarkApp {
    fn default() -> Self {
        Self {
            app_count: DEFAULT_APP_COUNT,
        }
    }
}

impl AppGridBenchmarkApp {
    pub fn new(app_count: usize) -> Self {
        Self { app_count }
    }
}

impl App for AppGridBenchmarkApp {
    type RootComponent = AppGridBenchmarkRoot;

    fn create(&mut self, ctx: &mut AppContext) -> Self::RootComponent {
        initialize_benchmark_app_state(ctx, self.app_count);
        AppGridBenchmarkRoot::default()
    }

    fn stop(&mut self, _ctx: &mut AppContext) {}
}

#[derive(Clone, Copy)]
pub struct AppGridBenchmarkRoot {
    grid_size: Vec2,
}

impl Default for AppGridBenchmarkRoot {
    fn default() -> Self {
        Self {
            grid_size: Vec2::new(DEFAULT_GRID_WIDTH, DEFAULT_GRID_HEIGHT),
        }
    }
}

impl Component for AppGridBenchmarkRoot {
    fn to_element(&self, _ctx: &mut ComponentContext) -> Element {
        Element::new()
            .with_tag("app-grid-benchmark-root")
            .with_style(
                Style::new()
                    .with_direction(FlexDirection::Column)
                    .with_align_items(AlignItems::Center)
                    .with_fixed_width(ItemSize::Percent(1.0))
                    .with_fixed_height(ItemSize::Percent(1.0)),
            )
            .with_content(
                Element::new()
                    .with_tag("app-grid-benchmark-shell")
                    .with_style(
                        Style::new()
                            .with_direction(FlexDirection::Column)
                            .with_fixed_size(self.grid_size.x, self.grid_size.y),
                    )
                    .with_content(AppGrid {
                        interactions_disabled: false,
                        hidden_app_id: None,
                        preferred_focus_app_id: None,
                        prefer_first_tile: true,
                    }),
            )
    }
}

fn initialize_benchmark_app_state(ctx: &mut AppContext, app_count: usize) {
    let apps_state = ctx.peek_global_state(Id::new(APPS_STATE_ID), AppsState::default);
    let mut apps_state = apps_state.write();
    apps_state.is_loading = false;
    apps_state.app_entries = benchmark_apps(app_count);
}

fn benchmark_apps(app_count: usize) -> Vec<AppEntry> {
    (0..app_count)
        .map(|index| {
            let id = Arc::new(format!("benchmark-app-{index}"));
            AppEntry {
                id: Arc::clone(&id),
                name: id,
                icon: Arc::new(None::<PathBuf>),
                accent: Color::from_rgb(0, 125, 215),
            }
        })
        .collect()
}
