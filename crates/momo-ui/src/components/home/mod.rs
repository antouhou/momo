mod app_grid;
pub(crate) mod app_icon;
pub(crate) mod app_tile;
#[cfg(feature = "bench-support")]
pub mod benchmark_support;
pub(crate) mod bluetooth;
mod clock_chip;
mod header;
mod launch;
pub(crate) mod model;
// pub(crate) mod settings_button;
pub(crate) mod system_status;
#[cfg(test)]
mod tests;
mod time;

use crate::components::dock::Dock;
use crate::components::home::app_grid::{AppGrid, PageDots};
use crate::components::home::header::HomeHeader;
use crate::components::home::launch::controller::use_launch_controller;
use crate::components::home::launch::overlay::LaunchOverlay;
use crate::components::home::model::{HOME_CLOCK_STATE_ID, HOME_CLOCK_THREAD_ID, SECTION_GAP};
use crate::components::home::time::{read_system_time, spawn_clock_thread};
use crate::components::quick_settings::{settings_overlay, should_render_settings_menu};
use daiko::component::{Component, ComponentContext};
use daiko::layout::{FlexDirection, ItemSize};
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

impl Component for Home {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        if self.live_clock {
            let clock_thread_started =
                ctx.peek_global_state(Id::new(HOME_CLOCK_THREAD_ID), || false);

            if !*clock_thread_started.read() {
                let clock_text =
                    ctx.peek_global_state(Id::new(HOME_CLOCK_STATE_ID), read_system_time);
                *clock_thread_started.write_silent() = true;
                spawn_clock_thread(clock_text);
            }
        }

        let launch = use_launch_controller(ctx);
        let should_render_settings_menu = should_render_settings_menu(ctx);

        let mut root = Element::new()
            .with_tag("login_screen-root")
            .with_style(home_style())
            // TODO: make the element inside the header into a view transition
            .with_content(HomeHeader::new(PageDots))
            .with_content(AppGrid {
                interactions_disabled: launch.active_launch.is_some(),
                hidden_app_id: launch.launched_app_id.clone(),
                preferred_focus_app_id: launch.preferred_focus_app_id,
                prefer_first_tile: launch.preferred_dock_focus_key.is_none(),
            })
            .with_content(
                Element::new()
                    .with_style(Style::new().with_fixed_height(ItemSize::Points(96.0)))
                    .with_content(Dock {
                        interactions_disabled: launch.active_launch.is_some(),
                        hidden_app_id: launch.launched_app_id,
                        preferred_focus_key: launch.preferred_dock_focus_key,
                    }),
            );

        if should_render_settings_menu {
            root.add_content(settings_overlay(ctx));
        }

        if let Some(active_launch) = launch.active_launch {
            root.add_content(LaunchOverlay {
                launch: active_launch,
            });
        }

        root
    }
}

fn home_style() -> Style {
    // TODO: Some possible gradient to set as the default background selection, move to the settings menu
    // later
    // // Minty
    // let gradient = LinearGradient::to(LinearSideOrCorner::TopRight)
    //     .stop(Color::from_rgb(3, 10, 8))       // black green
    //     .stop(Color::from_rgb(5, 48, 36))      // emerald shadow
    //     .stop(Color::from_rgb(92, 168, 143));  // muted mint highlight
    // // Dark to light
    // let gradient = LinearGradient::to(LinearSideOrCorner::BottomRight)
    //     .stop(Color::from_rgb(2, 4, 10))
    //     .stop(Color::from_rgb(7, 10, 24))
    //     .stop(Color::from_rgb(22, 17, 45))
    //     .stop(Color::from_rgb(64, 47, 82))
    //     .stop(Color::from_rgb(168, 132, 82));
    // // Gray-ish
    // let gradient = LinearGradient::to(LinearSideOrCorner::TopRight)
    //     .stop(Color::from_rgb(3, 5, 7))
    //     .stop(Color::from_rgb(12, 17, 22))
    //     .stop(Color::from_rgb(31, 39, 48))
    //     .stop(Color::from_rgb(88, 98, 106))
    //     .stop(Color::from_rgb(190, 176, 145));
    // // Wine
    // let gradient = LinearGradient::to(LinearSideOrCorner::Right)
    //     .stop(Color::from_rgb(8, 4, 6))
    //     .stop(Color::from_rgb(28, 8, 17))
    //     .stop(Color::from_rgb(72, 22, 38))
    //     .stop(Color::from_rgb(128, 67, 66))
    //     .stop(Color::from_rgb(214, 161, 112));
    // // Ubuntu-ish
    // let gradient = LinearGradient::to(LinearSideOrCorner::BottomRight)
    //     .stop(Color::from_rgb(1, 3, 12))
    //     .stop(Color::from_rgb(5, 14, 36))
    //     .stop(Color::from_rgb(20, 24, 67))
    //     .stop(Color::from_rgb(86, 42, 75))
    //     .stop(Color::from_rgb(202, 118, 71));
    // // Emerald/deep sea
    // let gradient = LinearGradient::to(LinearSideOrCorner::TopLeft)
    //     .stop(Color::from_rgb(1, 7, 6))
    //     .stop(Color::from_rgb(4, 22, 18))
    //     .stop(Color::from_rgb(6, 58, 47))
    //     .stop(Color::from_rgb(21, 116, 91))
    //     .stop(Color::from_rgb(176, 209, 176));
    // // Steel/champaigne
    // let gradient = LinearGradient::to(LinearSideOrCorner::TopRight)
    //     .stop_between_percent(0.00, 0.26, Color::from_rgb(3, 5, 7))
    //     .stop_at_percent(0.46, Color::from_rgb(12, 17, 22))
    //     .stop_at_percent(0.68, Color::from_rgb(31, 39, 48))
    //     .stop_between_percent(0.82, 0.92, Color::from_rgb(88, 98, 106))
    //     .stop_at_percent(1.00, Color::from_rgb(190, 176, 145));
    // // Graphite
    // let gradient = LinearGradient::to(LinearSideOrCorner::TopRight)
    //     .stop_at_percent(0.00, Color::from_rgb(3, 5, 7))
    //     .stop_at_percent(0.25, Color::from_rgb(8, 10, 13))
    //     .stop_at_percent(0.48, Color::from_rgb(17, 20, 25))
    //     .stop_at_percent(0.68, Color::from_rgb(35, 40, 47))
    //     .stop_at_percent(0.84, Color::from_rgb(68, 70, 70))
    //     .stop_at_percent(1.00, Color::from_rgb(126, 115, 91));
    // // Midnight
    // let gradient = LinearGradient::to(LinearSideOrCorner::BottomRight)
    //     .stop_at_percent(0.00, Color::from_rgb(5, 5, 13))
    //     .stop_at_percent(0.28, Color::from_rgb(10, 8, 24))
    //     .stop_at_percent(0.52, Color::from_rgb(18, 13, 36))
    //     .stop_at_percent(0.74, Color::from_rgb(35, 25, 58))
    //     .stop_at_percent(0.90, Color::from_rgb(55, 40, 78))
    //     .stop_at_percent(1.00, Color::from_rgb(82, 65, 104));
    // // Emerald
    // let gradient = LinearGradient::to(LinearSideOrCorner::TopLeft)
    //     .stop_at_percent(0.00, Color::from_rgb(1, 6, 6))
    //     .stop_at_percent(0.30, Color::from_rgb(3, 16, 14))
    //     .stop_at_percent(0.54, Color::from_rgb(5, 33, 28))
    //     .stop_at_percent(0.74, Color::from_rgb(11, 61, 50))
    //     .stop_at_percent(0.90, Color::from_rgb(31, 93, 76))
    //     .stop_at_percent(1.00, Color::from_rgb(83, 125, 106));
    // // Space/copper
    let gradient = LinearGradient::to(LinearSideOrCorner::BottomRight)
        .stop_at_percent(0.00, Color::from_rgb(2, 3, 12))
        .stop_at_percent(0.28, Color::from_rgb(6, 10, 27))
        .stop_at_percent(0.52, Color::from_rgb(18, 19, 49))
        .stop_at_percent(0.72, Color::from_rgb(43, 31, 65))
        .stop_at_percent(0.90, Color::from_rgb(82, 51, 66))
        .stop_at_percent(1.00, Color::from_rgb(126, 75, 58));
    // // Other emerald/dark forest
    // let gradient = LinearGradient::to(LinearSideOrCorner::TopLeft)
    //     .stop_at_percent(0.00, Color::from_rgb(2, 8, 7))
    //     .stop_at_percent(0.30, Color::from_rgb(4, 18, 15))
    //     .stop_at_percent(0.54, Color::from_rgb(7, 38, 31))
    //     .stop_at_percent(0.74, Color::from_rgb(15, 68, 54))
    //     .stop_at_percent(0.90, Color::from_rgb(43, 100, 82))
    //     .stop_at_percent(1.00, Color::from_rgb(91, 135, 116));

    // let gradient = LinearGradient::to(LinearSideOrCorner::TopRight)
    //     .stop_at_percent(0.00, Color::from_rgb(5, 6, 8))
    //     .stop_at_percent(0.30, Color::from_rgb(13, 15, 18))
    //     .stop_at_percent(0.55, Color::from_rgb(27, 30, 34))
    //     .stop_at_percent(0.76, Color::from_rgb(50, 53, 56))
    //     .stop_at_percent(0.92, Color::from_rgb(78, 76, 68))
    //     .stop_at_percent(1.00, Color::from_rgb(108, 96, 76));

    Style::new()
        .with_background(gradient)
        .with_direction(FlexDirection::Column)
        .with_spacing((SECTION_GAP, SECTION_GAP))
}
