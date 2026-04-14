mod app_grid;
mod app_tile;
mod clock_chip;
mod model;
mod time;

use crate::components::home::app_grid::AppGrid;
use crate::components::home::clock_chip::clock_chip;
use crate::components::home::model::{
    HOME_CLOCK_STATE_ID, HOME_CLOCK_THREAD_ID, SCREEN_PADDING, SECTION_GAP,
};
use crate::components::home::time::{read_system_time, spawn_clock_thread};
use daiko::component::{Component, ComponentContext};
use daiko::layout::{FlexDirection, JustifyContent};
use daiko::style::{Color, Style};
use daiko::widgets::container::{Container, Fit};
use daiko::widgets::heading::{Heading, HeadingLevel};
use daiko::widgets::text::VerticalTextAlignment;
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
        // ctx.app_context.set_fullscreen(true);

        let clock_text = ctx.use_global_state(Id::new(HOME_CLOCK_STATE_ID), read_system_time);
        let clock_thread_started = ctx.use_global_state(Id::new(HOME_CLOCK_THREAD_ID), || false);

        if self.live_clock && !*clock_thread_started.read() {
            *clock_thread_started.write_silent() = true;
            spawn_clock_thread(clock_text.clone());
        }

        let header = Container::horizontal()
            .with_style(Style::new().with_justify_content(JustifyContent::SpaceBetween))
            .with_fit(Fit::new().exact_content_height())
            .align_items_center()
            .build()
            .with_tag("apps-header")
            .with_content(
                Heading::new("Apps", HeadingLevel::H1)
                    .with_vertical_text_alignment(VerticalTextAlignment::Center),
            )
            .with_content(clock_chip(clock_text.read().clone()));

        Element::new()
            .with_tag("home-root")
            .with_style(home_style())
            .with_content(header)
            .with_content(AppGrid)
    }
}

fn home_style() -> Style {
    Style::new()
        .with_background_color(Color::from_rgb(9, 12, 19))
        .with_direction(FlexDirection::Column)
        .with_padding(SCREEN_PADDING)
        .with_spacing((SECTION_GAP, SECTION_GAP))
}

#[cfg(test)]
mod tests {
    use super::Home;
    use daiko::navigation::{FocusKey, FocusOrigin};
    use daiko::testing::TestRunner;
    use daiko::{App, AppContext, Vec2};

    struct HomeTestApp;

    impl App for HomeTestApp {
        type RootComponent = Home;

        fn create(&mut self, _ctx: &mut AppContext) -> Self::RootComponent {
            Home::for_testing()
        }

        fn stop(&mut self, _ctx: &mut AppContext) {}
    }

    #[test]
    fn first_tile_is_preferred_focus_target() {
        let mut runner = TestRunner::new(HomeTestApp);
        runner.set_viewport_size(1280.0, 720.0);
        runner.run_frame();

        runner.focus_element_by_key(FocusKey::new("live-tv"), FocusOrigin::Navigation);
        runner.assert_focused("live-tv");
    }

    #[test]
    fn directional_navigation_moves_across_the_grid() {
        let mut runner = TestRunner::new(HomeTestApp);
        runner.set_viewport_size(1280.0, 720.0);
        runner.run_frame();

        runner.focus_element_by_key(FocusKey::new("live-tv"), FocusOrigin::Navigation);
        runner.navigate_right();
        runner.run_frame();
        runner.assert_focused("movies");

        runner.navigate_down();
        runner.run_frame();
        runner.assert_focused("settings");
    }

    #[test]
    fn root_matches_viewport_size() {
        let mut runner = TestRunner::new(HomeTestApp);
        runner.set_viewport_size(1280.0, 720.0);
        runner.run_frame();

        runner.assert_element_bounds("home-root", Vec2::new(0.0, 0.0), Vec2::new(1280.0, 720.0));
    }

    #[test]
    fn clock_chip_stays_near_the_right_edge() {
        let mut runner = TestRunner::new(HomeTestApp);
        runner.set_viewport_size(1280.0, 720.0);
        runner.run_frame();

        let (position, size) = runner.get_element_bounds("clock-chip");
        assert!(
            position.x > 1000.0,
            "clock chip should be near the right edge"
        );
        assert!(
            size.x < 220.0,
            "clock chip should size to content, not fill the row"
        );
    }

    #[test]
    fn apps_row_is_centered() {
        let mut runner = TestRunner::new(HomeTestApp);
        runner.set_viewport_size(1280.0, 720.0);
        runner.run_frame();

        let (position, _size) = runner.get_element_bounds("live-tv");
        assert!(
            position.x > 100.0,
            "first app tile should not hug the left edge"
        );
    }

    #[test]
    fn apps_header_has_visible_height() {
        let mut runner = TestRunner::new(HomeTestApp);
        runner.set_viewport_size(1280.0, 720.0);
        runner.run_frame();

        let (_position, size) = runner.get_element_bounds("apps-header");
        assert!(size.y > 20.0, "apps header should have visible height");
    }

    #[test]
    fn hovering_tile_moves_focus_to_the_hovered_app() {
        let mut runner = TestRunner::new(HomeTestApp);
        runner.set_viewport_size(1280.0, 720.0);
        runner.run_frame();

        let tile_center = runner.get_element_center("movies");
        runner.move_pointer_to(tile_center);
        runner.run_frame();
        runner.run_frame();

        runner.assert_focused("movies");
    }
}
