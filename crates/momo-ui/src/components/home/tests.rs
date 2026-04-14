use super::Home;
use daiko::navigation::{FocusKey, FocusOrigin};
use daiko::testing::TestRunner;
use daiko::{App, AppContext, Vec2};
use std::thread;
use std::time::Duration;

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

#[test]
fn activating_tile_creates_launch_overlay_for_that_app() {
    let mut runner = TestRunner::new(HomeTestApp);
    runner.set_viewport_size(1280.0, 720.0);
    runner.run_frame();

    runner.click_element("movies");
    runner.run_frame();

    assert!(runner.find_element_by_tag("launch-overlay").is_some());
    assert!(
        runner
            .find_element_by_tag("launch-overlay-surface")
            .is_some()
    );
    assert!(
        runner
            .find_element_by_tag("launch-overlay-app-movies")
            .is_some()
    );
}

#[test]
fn cancel_reverses_launch_overlay_and_restores_tile_focus() {
    let mut runner = TestRunner::new(HomeTestApp);
    runner.set_viewport_size(1280.0, 720.0);
    runner.run_frame();

    runner.click_element("movies");
    runner.run_frame();
    thread::sleep(Duration::from_millis(420));
    runner.run_frame();

    runner.press_cancel();
    thread::sleep(Duration::from_millis(420));
    runner.run_frame();
    runner.run_frame();

    assert!(runner.find_element_by_tag("launch-overlay").is_none());
    runner.assert_focused("movies");
}
