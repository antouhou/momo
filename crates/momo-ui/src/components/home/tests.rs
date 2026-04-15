use super::Home;
use super::app_grid::AppGrid;
use super::model::{MOCK_APPS, SCREEN_PADDING, TILE_HEIGHT, columns_for_width};
use daiko::component::{Component, ComponentContext};
use daiko::integration::input::{InputEvent, InputEventModifiers};
use daiko::layout::{AlignItems, FlexDirection, ItemSize};
use daiko::navigation::{FocusKey, FocusOrigin};
use daiko::style::Style;
use daiko::testing::TestRunner;
use daiko::{App, AppContext, Element, Pos2, Vec2};
use std::thread;
use std::time::{Duration, Instant};

struct HomeTestApp;

impl App for HomeTestApp {
    type RootComponent = Home;

    fn create(&mut self, _ctx: &mut AppContext) -> Self::RootComponent {
        Home::for_testing()
    }

    fn stop(&mut self, _ctx: &mut AppContext) {}
}

struct FixedWidthGridTestApp;

impl App for FixedWidthGridTestApp {
    type RootComponent = FixedWidthGridRoot;

    fn create(&mut self, _ctx: &mut AppContext) -> Self::RootComponent {
        FixedWidthGridRoot
    }

    fn stop(&mut self, _ctx: &mut AppContext) {}
}

#[derive(Clone, Copy)]
struct FixedWidthGridRoot;

impl Component for FixedWidthGridRoot {
    fn to_element(&self, _ctx: &mut ComponentContext) -> Element {
        Element::new()
            .with_tag("fixed-grid-root")
            .with_style(
                Style::new()
                    .with_direction(FlexDirection::Column)
                    .with_align_items(AlignItems::Center)
                    .with_fixed_width(ItemSize::Percent(1.0))
                    .with_fixed_height(ItemSize::Percent(1.0)),
            )
            .with_content(
                Element::new()
                    .with_tag("grid-shell")
                    .with_style(
                        Style::new()
                            .with_direction(FlexDirection::Column)
                            .with_fixed_size(960.0, 500.0),
                    )
                    .with_content(AppGrid {
                        interactions_disabled: false,
                        hidden_app_id: None,
                        preferred_focus_app_id: None,
                    }),
            )
    }
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
fn apps_row_is_centered_in_the_content_area() {
    let mut runner = TestRunner::new(HomeTestApp);
    runner.set_viewport_size(1280.0, 720.0);
    runner.run_frame();

    let (first_position, _first_size) = runner.get_element_bounds("live-tv");
    let (last_position, last_size) = runner.get_element_bounds("photos");
    let left_gutter = first_position.x - SCREEN_PADDING;
    let right_gutter = 1280.0 - SCREEN_PADDING - (last_position.x + last_size.x);

    assert!(
        left_gutter > 0.0,
        "first app tile should have centered gutter inside the content area"
    );
    assert!(
        (left_gutter - right_gutter).abs() < 0.5,
        "app row gutters should be symmetric, left={left_gutter}, right={right_gutter}"
    );
}

#[test]
fn app_grid_shows_page_dots() {
    let mut runner = TestRunner::new(HomeTestApp);
    runner.set_viewport_size(1280.0, 720.0);
    runner.run_frame();

    assert!(runner.find_element_by_tag("apps-grid-page-dots").is_some());
    assert!(runner.find_element_by_tag("apps-grid-page-dot-0").is_some());
    assert!(runner.find_element_by_tag("apps-grid-page-dot-1").is_some());
}

#[test]
fn app_grid_does_not_clip_edge_columns_near_breakpoints() {
    for viewport_width in [
        594.0, 600.0, 854.0, 860.0, 866.0, 1120.0, 1126.0, 1132.0, 1280.0, 1392.0, 1398.0,
    ] {
        let mut runner = TestRunner::new(HomeTestApp);
        runner.set_viewport_size(viewport_width, 720.0);
        runner.run_frame();
        runner.run_frame();

        let (viewport_position, viewport_size) = runner.get_element_bounds("apps-grid-viewport");
        let viewport_left = viewport_position.x;
        let viewport_right = viewport_position.x + viewport_size.x;
        let content_left = SCREEN_PADDING;
        let content_right = viewport_width - SCREEN_PADDING;
        let expected_columns = columns_for_width(viewport_width - SCREEN_PADDING * 2.0);
        let expected_first_page_tile_count = expected_columns * 2;

        assert!(
            viewport_left.abs() < 0.5,
            "pager viewport should start at the window edge at viewport width {viewport_width}: viewport_left={viewport_left}"
        );
        assert!(
            (viewport_right - viewport_width).abs() < 0.5,
            "pager viewport should end at the window edge at viewport width {viewport_width}: viewport_right={viewport_right}, viewport_size={viewport_size:?}"
        );

        for app in MOCK_APPS.iter().take(expected_first_page_tile_count) {
            let (tile_position, tile_size) = runner.get_element_bounds(app.id);
            assert!(
                tile_position.x >= viewport_left - 0.5,
                "{} should not be clipped on the left at viewport width {viewport_width}",
                app.id
            );
            assert!(
                tile_position.x + tile_size.x <= viewport_right + 0.5,
                "{} should not be clipped on the right at viewport width {viewport_width}",
                app.id
            );
        }

        let first_row_last_app_index = expected_columns.saturating_sub(1);
        let (first_tile_position, _first_tile_size) = runner.get_element_bounds(MOCK_APPS[0].id);
        let (last_tile_position, last_tile_size) =
            runner.get_element_bounds(MOCK_APPS[first_row_last_app_index].id);
        let left_gutter = first_tile_position.x - content_left;
        let right_gutter = content_right - (last_tile_position.x + last_tile_size.x);

        assert!(
            left_gutter >= -0.5,
            "first page tile should not start before the content edge at viewport width {viewport_width}"
        );
        assert!(
            (left_gutter - right_gutter).abs() < 0.5,
            "first row should be centered at viewport width {viewport_width}: left={left_gutter}, right={right_gutter}"
        );
    }
}

#[test]
fn app_grid_uses_wrapper_layout_for_page_width() {
    let mut runner = TestRunner::new(FixedWidthGridTestApp);
    runner.set_viewport_size(1280.0, 720.0);
    runner.run_frame();
    runner.run_frame();

    let (shell_position, shell_size) = runner.get_element_bounds("grid-shell");
    let (viewport_position, viewport_size) = runner.get_element_bounds("apps-grid-viewport");
    let expected_columns = columns_for_width(shell_size.x - SCREEN_PADDING * 2.0);
    let first_row_last_app_index = expected_columns.saturating_sub(1);
    let (first_tile_position, _first_tile_size) = runner.get_element_bounds(MOCK_APPS[0].id);
    let (last_tile_position, last_tile_size) =
        runner.get_element_bounds(MOCK_APPS[first_row_last_app_index].id);
    let content_left = shell_position.x + SCREEN_PADDING;
    let content_right = shell_position.x + shell_size.x - SCREEN_PADDING;
    let left_gutter = first_tile_position.x - content_left;
    let right_gutter = content_right - (last_tile_position.x + last_tile_size.x);

    assert!(
        (viewport_position.x - shell_position.x).abs() < 0.5,
        "pager viewport should start at its wrapper edge"
    );
    assert!(
        (viewport_size.x - shell_size.x).abs() < 0.5,
        "pager viewport should use wrapper width, viewport={viewport_size:?}, shell={shell_size:?}"
    );
    assert_eq!(
        expected_columns, 3,
        "960px shell should compute columns from the logical 880px content width"
    );
    assert!(
        (left_gutter - right_gutter).abs() < 0.5,
        "row should be centered inside the shell's logical content area"
    );
}

#[test]
fn app_grid_height_shrinks_after_window_height_shrinks() {
    let mut runner = TestRunner::new(HomeTestApp);
    runner.set_viewport_size(1280.0, 980.0);
    runner.run_frame();
    runner.run_frame();
    runner.run_frame();

    let (_tall_position, tall_size) = runner.get_element_bounds("apps-grid-viewport");

    runner.set_viewport_size(1280.0, 560.0);
    runner.run_frame();
    runner.run_frame();
    runner.run_frame();
    runner.run_frame();

    let (_short_position, short_size) = runner.get_element_bounds("apps-grid-viewport");

    assert!(
        tall_size.y >= short_size.y + TILE_HEIGHT,
        "grid viewport should drop at least one row after shrinking the window height, tall={tall_size:?}, short={short_size:?}"
    );
}

#[test]
fn directional_navigation_pages_at_the_grid_edge() {
    let mut runner = TestRunner::new(HomeTestApp);
    runner.set_viewport_size(1280.0, 720.0);
    runner.run_frame();

    runner.focus_element_by_key(FocusKey::new("photos"), FocusOrigin::Navigation);
    runner.navigate_right();
    runner.run_frame();

    runner.assert_focused("podcasts");

    let (_inactive_dot_position, inactive_dot_size) =
        runner.get_element_bounds("apps-grid-page-dot-visual-0");
    let (_active_dot_position, active_dot_size) =
        runner.get_element_bounds("apps-grid-page-dot-visual-1");
    assert!(
        active_dot_size.x > inactive_dot_size.x,
        "second page dot should be active after paging right"
    );
}

#[test]
fn focused_page_dot_keeps_active_page_visual_distinct() {
    let mut runner = TestRunner::new(HomeTestApp);
    runner.set_viewport_size(1280.0, 720.0);
    runner.run_frame();
    runner.run_frame();

    runner.focus_element_by_key(
        FocusKey::new("apps-grid-page-dot-1"),
        FocusOrigin::Navigation,
    );
    runner.run_frame();

    let (_active_position, active_size) = runner.get_element_bounds("apps-grid-page-dot-visual-0");
    let (_focused_position, focused_size) =
        runner.get_element_bounds("apps-grid-page-dot-visual-1");

    assert!(
        active_size.x > focused_size.x,
        "focused inactive page dot should keep the inactive visual size"
    );
}

#[test]
fn inactive_page_dot_target_keeps_compact_width() {
    let mut runner = TestRunner::new(HomeTestApp);
    runner.set_viewport_size(1280.0, 720.0);
    runner.run_frame();
    runner.run_frame();

    let (_active_position, active_size) = runner.get_element_bounds("apps-grid-page-dot-0");
    let (_inactive_position, inactive_size) = runner.get_element_bounds("apps-grid-page-dot-1");

    assert!(
        active_size.x > inactive_size.x,
        "inactive page dot target should not reserve the active page dot width"
    );
    assert!(
        (active_size.x - 30.0).abs() < 0.5,
        "active page dot target should be visual width plus 2px padding and 2px border per side"
    );
    assert!(
        (inactive_size.x - 16.0).abs() < 0.5,
        "inactive page dot target should be visual width plus 2px padding and 2px border per side"
    );
}

#[test]
fn hovered_page_dot_keeps_inactive_visual_distinct() {
    let mut runner = TestRunner::new(HomeTestApp);
    runner.set_viewport_size(1280.0, 720.0);
    runner.run_frame();
    runner.run_frame();

    runner.hover_element("apps-grid-page-dot-1");
    runner.run_frame();

    let (_active_position, active_size) = runner.get_element_bounds("apps-grid-page-dot-visual-0");
    let (_hovered_position, hovered_size) =
        runner.get_element_bounds("apps-grid-page-dot-visual-1");
    let (_hovered_target_position, hovered_target_size) =
        runner.get_element_bounds("apps-grid-page-dot-1");

    assert!(
        active_size.x > hovered_size.x,
        "hovered inactive page dot should keep the inactive visual size"
    );
    assert!(
        (hovered_target_size.x - 16.0).abs() < 0.5,
        "hovered inactive page dot target should keep padding and border geometry"
    );
}

#[test]
fn page_dot_focus_can_escape_back_to_the_grid() {
    let mut runner = TestRunner::new(HomeTestApp);
    runner.set_viewport_size(1280.0, 720.0);
    runner.run_frame();
    runner.run_frame();

    runner.focus_element_by_key(
        FocusKey::new("apps-grid-page-dot-0"),
        FocusOrigin::Navigation,
    );
    runner.navigate_up();
    runner.run_frame();

    let focused_tag = runner
        .focused_element()
        .and_then(|element| element.tag())
        .unwrap_or("<untagged>");
    assert!(
        !focused_tag.starts_with("apps-grid-page-dot-"),
        "focus should be able to escape the page dot scope, but stayed on {focused_tag}"
    );
}

#[test]
fn clicking_page_dot_jumps_to_that_page() {
    let mut runner = TestRunner::new(HomeTestApp);
    runner.set_viewport_size(1280.0, 720.0);
    runner.run_frame();
    runner.run_frame();

    let (initial_position, _initial_size) = runner.get_element_bounds("apps-grid-page-1");
    runner.click_element("apps-grid-page-dot-1");
    runner.run_frame();
    thread::sleep(Duration::from_millis(260));
    runner.run_frame();

    let (position, _size) = runner.get_element_bounds("apps-grid-page-1");
    assert!(
        position.x < initial_position.x - 100.0,
        "clicking a page dot should move that page toward the viewport"
    );
}

#[test]
fn activating_focused_page_dot_jumps_to_that_page() {
    let mut runner = TestRunner::new(HomeTestApp);
    runner.set_viewport_size(1280.0, 720.0);
    runner.run_frame();
    runner.run_frame();

    let (initial_position, _initial_size) = runner.get_element_bounds("apps-grid-page-1");
    runner.focus_element_by_key(
        FocusKey::new("apps-grid-page-dot-1"),
        FocusOrigin::Navigation,
    );
    runner.press_confirm();
    thread::sleep(Duration::from_millis(260));
    runner.run_frame();

    let (position, _size) = runner.get_element_bounds("apps-grid-page-1");
    assert!(
        position.x < initial_position.x - 100.0,
        "activating a focused page dot should move that page toward the viewport"
    );
}

#[test]
fn vertical_wheel_scroll_pages_the_grid() {
    let mut runner = TestRunner::new(HomeTestApp);
    runner.set_viewport_size(1280.0, 720.0);
    runner.run_frame();
    runner.run_frame();

    let (first_tile_position, first_tile_size) = runner.get_element_bounds("live-tv");
    let first_tile_center = Pos2::new(
        first_tile_position.x + first_tile_size.x / 2.0,
        first_tile_position.y + first_tile_size.y / 2.0,
    );
    let (initial_position, _initial_size) = runner.get_element_bounds("apps-grid-page-1");

    runner.move_pointer_to(first_tile_center);
    for _ in 0..4 {
        runner
            .app_runner_mut()
            .context
            .add_input_event(InputEvent::scroll(
                Vec2::new(0.0, -2.0),
                InputEventModifiers::default(),
                Instant::now(),
            ));
        runner.run_frame();
    }
    for _ in 0..8 {
        runner
            .app_runner_mut()
            .context
            .add_input_event(InputEvent::scroll(
                Vec2::new(0.0, -2.0),
                InputEventModifiers::default(),
                Instant::now(),
            ));
        runner.run_frame();
    }
    thread::sleep(Duration::from_millis(260));
    runner.run_frame();

    let (position, _size) = runner.get_element_bounds("apps-grid-page-1");
    let (second_next_position, _second_next_size) = runner.get_element_bounds("apps-grid-page-2");
    assert!(
        position.x < initial_position.x - 100.0,
        "vertical wheel scroll should move the next page toward the viewport, initial_x={}, current_x={}",
        initial_position.x,
        position.x
    );
    assert!(
        second_next_position.x > 600.0,
        "continued deltas after one page switch should not immediately advance another page"
    );

    let (first_page_scrolled_position, _first_page_scrolled_size) =
        runner.get_element_bounds("apps-grid-page-0");
    for _ in 0..4 {
        runner
            .app_runner_mut()
            .context
            .add_input_event(InputEvent::scroll(
                Vec2::new(0.0, 2.0),
                InputEventModifiers::default(),
                Instant::now(),
            ));
        runner.run_frame();
    }
    thread::sleep(Duration::from_millis(260));
    runner.run_frame();

    let (first_page_returned_position, _first_page_returned_size) =
        runner.get_element_bounds("apps-grid-page-0");
    assert!(
        first_page_returned_position.x > first_page_scrolled_position.x + 100.0,
        "positive vertical wheel scroll should move the previous page back toward the viewport"
    );
}

#[test]
fn apps_header_has_visible_height() {
    let mut runner = TestRunner::new(HomeTestApp);
    runner.set_viewport_size(1280.0, 720.0);
    runner.run_frame();

    let (position, size) = runner.get_element_bounds("apps-header");
    assert!(size.y > 20.0, "apps header should have visible height");
    assert!(
        position.x.abs() < 0.5,
        "apps header should fill from the window edge"
    );
}

#[test]
fn apps_header_content_is_padded() {
    let mut runner = TestRunner::new(HomeTestApp);
    runner.set_viewport_size(1280.0, 720.0);
    runner.run_frame();

    let (title_position, _title_size) = runner.get_element_bounds("apps-header-title");
    let (clock_position, clock_size) = runner.get_element_bounds("clock-chip");
    let clock_right_edge = clock_position.x + clock_size.x;

    assert!(
        (title_position.x - SCREEN_PADDING).abs() < 0.5,
        "header title should start after the left padding"
    );
    assert!(
        title_position.y >= SCREEN_PADDING - 0.5,
        "header title should sit below the top padding"
    );
    assert!(
        clock_right_edge <= 1280.0 - SCREEN_PADDING + 0.5,
        "clock chip should stay inside the right padding"
    );
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
