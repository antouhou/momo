mod app_grid;
mod app_tile;
mod clock_chip;
mod header;
mod launch;
mod model;
#[cfg(test)]
mod tests;
mod time;

use crate::components::home::app_grid::AppGrid;
use crate::components::home::header::HomeHeader;
use crate::components::home::launch::controller::use_launch_controller;
use crate::components::home::launch::overlay::render_launch_overlay;
use crate::components::home::model::{
    HOME_CLOCK_STATE_ID, HOME_CLOCK_THREAD_ID, SCREEN_PADDING, SECTION_GAP,
};
use crate::components::home::time::{read_system_time, spawn_clock_thread};
use daiko::component::{Component, ComponentContext};
use daiko::layout::FlexDirection;
use daiko::style::{Color, LinearGradient, LinearSideOrCorner, Style};
use daiko::{Element, Id};

#[derive(Clone, Copy)]
pub struct Home {
    live_clock: bool,
}

impl Home {
    pub fn new() -> Self {
        Self { live_clock: true }
    }

    #[cfg(test)]
    fn for_testing() -> Self {
        Self { live_clock: false }
    }
}

impl Default for Home {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for Home {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        ctx.app_context.set_fullscreen(true);

        if self.live_clock {
            let clock_thread_started =
                ctx.peek_global_state(Id::new(HOME_CLOCK_THREAD_ID), || false);

            if !*clock_thread_started.read() {
                let clock_text =
                    ctx.peek_global_state(Id::new(HOME_CLOCK_STATE_ID), read_system_time);
                *clock_thread_started.write_silent() = true;
                spawn_clock_thread(clock_text.clone());
            }
        }

        let launch = use_launch_controller(ctx);

        let mut root = Element::new()
            .with_tag("home-root")
            .with_style(home_style())
            .with_content(HomeHeader)
            .with_content(AppGrid {
                interactions_disabled: launch.active_launch.is_some(),
                hidden_app_id: launch.launched_app_id,
                preferred_focus_app_id: launch.preferred_focus_app_id,
            });

        if let Some(active_launch) = launch.active_launch {
            root.add_content(render_launch_overlay(
                ctx,
                active_launch,
                launch.launch_progress,
            ));
        }

        root
    }
}

fn home_style() -> Style {
    Style::new()
        .with_background(
            LinearGradient::to(LinearSideOrCorner::TopRight)
                .stop(Color::from_rgb(6, 13, 16))
                .stop(Color::from_rgb(10, 32, 38))
                .stop(Color::from_rgb(54, 47, 28)),
        )
        .with_direction(FlexDirection::Column)
        .with_padding(SCREEN_PADDING)
        .with_spacing((SECTION_GAP, SECTION_GAP))
}
