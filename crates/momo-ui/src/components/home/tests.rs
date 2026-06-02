use super::Home;
use super::app_grid::AppGrid;
use super::bluetooth::initialize_bluetooth_state;
use super::model::{MOCK_APPS, SCREEN_PADDING, TILE_HEIGHT, columns_for_width};
use super::system_status::initialize_system_status_state;
use daiko::component::{Component, ComponentContext};
use daiko::integration::input::{InputEvent, InputEventModifiers, Key};
use daiko::layout::{AlignItems, FlexDirection, ItemSize};
use daiko::navigation::{FocusKey, FocusOrigin};
use daiko::style::Style;
use daiko::testing::TestRunner;
use daiko::{App, AppContext, Element, Pos2, Vec2};
use std::thread;
use std::time::{Duration, Instant};
use system_control::SystemControl;

struct HomeTestApp;

impl App for HomeTestApp {
    type RootComponent = Home;

    fn create(&mut self, ctx: &mut AppContext) -> Self::RootComponent {
        let system_control =
            SystemControl::new().expect("failed to initialize system control for tests");
        initialize_bluetooth_state(ctx, system_control.bluetooth());
        initialize_system_status_state(ctx, system_control.volume(), system_control.battery());
        Home::for_testing()
    }

    fn stop(&mut self, _ctx: &mut AppContext) {}
}

struct FixedWidthGridTestApp;

impl App for FixedWidthGridTestApp {
    type RootComponent = FixedWidthGridRoot;

    fn create(&mut self, ctx: &mut AppContext) -> Self::RootComponent {
        let system_control =
            SystemControl::new().expect("failed to initialize system control for tests");
        initialize_bluetooth_state(ctx, system_control.bluetooth());
        initialize_system_status_state(ctx, system_control.volume(), system_control.battery());
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
fn settings_menu_opens_from_the_header_button() {
    let mut runner = TestRunner::new(HomeTestApp);
    runner.set_viewport_size(1280.0, 720.0);
    runner.run_frame();

    runner.click_element("header-settings-button");
    runner.run_frame();

    assert!(runner.find_element_by_tag("header-settings-menu").is_some());
}

#[test]
fn settings_menu_anchors_near_the_top_right_corner() {
    let mut runner = TestRunner::new(HomeTestApp);
    runner.set_viewport_size(1280.0, 720.0);
    runner.run_frame();

    runner.click_element("header-settings-button");
    runner.run_frame();
    thread::sleep(Duration::from_millis(320));
    runner.run_frame();
    let (end_position, _) = runner.get_element_bounds("header-settings-menu");
    let expected_x = 1280.0 - SCREEN_PADDING - 392.0;

    assert!(
        (end_position.x - expected_x).abs() < 0.5,
        "settings drawer should anchor near the top-right corner, end_position={end_position:?}"
    );
    assert!(
        (end_position.y - 96.0).abs() < 0.5,
        "settings drawer should sit below the header row, end_position={end_position:?}"
    );
}

#[test]
fn settings_menu_renders_the_mock_power_button() {
    let mut runner = TestRunner::new(HomeTestApp);
    runner.set_viewport_size(1280.0, 720.0);
    runner.run_frame();

    runner.click_element("header-settings-button");
    runner.run_frame();
    assert!(runner.find_element_by_tag("header-settings-menu").is_some());

    assert!(
        runner
            .find_element_by_tag("header-settings-exit-button")
            .is_some()
    );
}

#[test]
fn settings_sections_remember_the_last_focused_control_when_reentering() {
    let mut runner = TestRunner::new(HomeTestApp);
    runner.set_viewport_size(1280.0, 720.0);
    runner.run_frame();

    runner.click_element("header-settings-button");
    runner.run_frame();
    runner.run_frame();
    thread::sleep(Duration::from_millis(320));
    runner.run_frame();

    runner.focus_element_by_tag("header-settings-status-chip", FocusOrigin::Navigation);
    runner.run_frame();
    runner.navigate_right();
    runner.run_frame();
    runner.navigate_right();
    runner.run_frame();
    runner.navigate_right();
    runner.run_frame();
    runner.navigate_right();
    runner.run_frame();
    runner.assert_focused("header-settings-exit-button");

    runner.navigate_down();
    runner.assert_focused("header-settings-volume-control");
    runner.navigate_up();
    runner.assert_focused("header-settings-exit-button");

    runner.navigate_down();
    runner.assert_focused("header-settings-volume-control");
    runner.navigate_down();
    runner.assert_focused("header-settings-tile-network");
    runner.navigate_right();
    runner.run_frame();
    runner.assert_focused("header-settings-tile-bluetooth");

    runner.navigate_up();
    runner.assert_focused("header-settings-volume-control");
    runner.navigate_down();
    runner.assert_focused("header-settings-tile-bluetooth");
}

#[test]
fn settings_button_click_closes_the_open_menu_without_reopening_it() {
    let mut runner = TestRunner::new(HomeTestApp);
    runner.set_viewport_size(1280.0, 720.0);
    runner.run_frame();

    runner.click_element("header-settings-button");
    runner.run_frame();
    runner.run_frame();
    assert!(runner.find_element_by_tag("header-settings-menu").is_some());

    runner.click_element("header-settings-button");
    runner.run_frame();
    runner.run_frame();
    thread::sleep(Duration::from_millis(320));
    runner.run_frame();

    assert!(runner.find_element_by_tag("header-settings-menu").is_none());
}

#[test]
fn settings_menu_closes_from_cancel_navigation() {
    let mut runner = TestRunner::new(HomeTestApp);
    runner.set_viewport_size(1280.0, 720.0);
    runner.run_frame();

    runner.click_element("header-settings-button");
    runner.run_frame();
    runner.run_frame();
    assert!(runner.find_element_by_tag("header-settings-menu").is_some());

    runner.press_cancel();
    runner.run_frame();
    runner.run_frame();
    thread::sleep(Duration::from_millis(320));
    runner.run_frame();

    assert!(runner.find_element_by_tag("header-settings-menu").is_none());
    runner.assert_focused("header-settings-button");
}

#[test]
fn settings_menu_back_from_bluetooth_returns_to_main_without_closing() {
    let mut runner = TestRunner::new(HomeTestApp);
    runner.set_viewport_size(1280.0, 720.0);
    runner.run_frame();

    runner.click_element("header-settings-button");
    runner.run_frame();
    runner.run_frame();
    thread::sleep(Duration::from_millis(320));
    runner.run_frame();

    runner.click_element("header-settings-tile-bluetooth");
    runner.run_frame();
    assert!(
        runner
            .find_element_by_tag("header-settings-bluetooth-submenu")
            .is_some()
    );

    thread::sleep(Duration::from_millis(220));
    runner.run_frame();
    assert!(runner.find_element_by_tag("header-settings-menu").is_some());
    assert!(
        runner
            .find_element_by_tag("header-settings-bluetooth-submenu")
            .is_some()
    );

    runner.press_key_and_run_frame(Key::BrowserBack);
    runner.run_frame();

    assert!(runner.find_element_by_tag("header-settings-menu").is_some());
    assert!(
        runner
            .find_element_by_tag("header-settings-tile-bluetooth")
            .is_some()
    );

    thread::sleep(Duration::from_millis(1220));
    runner.run_frame();

    assert!(
        runner
            .find_element_by_tag("header-settings-bluetooth-submenu")
            .is_none()
    );
}

#[test]
fn settings_menu_height_animates_when_opening_bluetooth_submenu() {
    let mut runner = TestRunner::new(HomeTestApp);
    runner.set_viewport_size(1280.0, 720.0);
    runner.run_frame();

    runner.click_element("header-settings-button");
    runner.run_frame();
    runner.run_frame();
    thread::sleep(Duration::from_millis(320));
    runner.run_frame();

    let (_, main_size) = runner.get_element_bounds("header-settings-menu");

    runner.click_element("header-settings-tile-bluetooth");
    runner.run_frame();
    runner.run_frame();
    thread::sleep(Duration::from_millis(220));
    runner.run_frame();

    let (_, mid_transition_size) = runner.get_element_bounds("header-settings-menu");

    thread::sleep(Duration::from_millis(460));
    runner.run_frame();
    let (_, bluetooth_size) = runner.get_element_bounds("header-settings-menu");

    assert_ne!(
        main_size.y, bluetooth_size.y,
        "main and Bluetooth menu heights should differ for this regression to be meaningful"
    );

    let lower_bound = main_size.y.min(bluetooth_size.y);
    let upper_bound = main_size.y.max(bluetooth_size.y);
    assert!(
        mid_transition_size.y > lower_bound && mid_transition_size.y < upper_bound,
        "menu height should be between stable sizes during transition, main={}, mid={}, bluetooth={}",
        main_size.y,
        mid_transition_size.y,
        bluetooth_size.y,
    );
}

#[test]
fn settings_menu_closes_when_focus_leaves_the_overlay() {
    let mut runner = TestRunner::new(HomeTestApp);
    runner.set_viewport_size(1280.0, 720.0);
    runner.run_frame();

    runner.click_element("header-settings-button");
    runner.run_frame();
    runner.run_frame();
    assert!(runner.find_element_by_tag("header-settings-menu").is_some());

    runner.focus_element_by_key(FocusKey::new("live-tv"), FocusOrigin::Navigation);
    runner.run_frame();
    thread::sleep(Duration::from_millis(320));
    runner.run_frame();

    assert!(runner.find_element_by_tag("header-settings-menu").is_none());
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
fn app_grid_wrapper_shrinks_with_window_height() {
    let mut runner = TestRunner::new(HomeTestApp);
    runner.set_viewport_size(1280.0, 980.0);
    runner.run_frame();
    runner.run_frame();
    runner.run_frame();

    let (_tall_position, tall_size) = runner.get_element_bounds("apps-grid");

    runner.set_viewport_size(1280.0, 560.0);
    runner.run_frame();
    runner.run_frame();
    runner.run_frame();
    runner.run_frame();

    let (_short_position, short_size) = runner.get_element_bounds("apps-grid");

    assert!(
        tall_size.y >= short_size.y + 300.0,
        "grid wrapper should release its old minimum height after shrinking the window, tall={tall_size:?}, short={short_size:?}"
    );
}

#[test]
fn app_grid_width_shrinks_after_window_width_shrinks() {
    let mut runner = TestRunner::new(HomeTestApp);
    runner.set_viewport_size(1280.0, 720.0);
    runner.run_frame();
    runner.run_frame();
    runner.run_frame();

    let (_wide_grid_position, wide_grid_size) = runner.get_element_bounds("apps-grid");
    let (_wide_viewport_position, wide_viewport_size) =
        runner.get_element_bounds("apps-grid-viewport");

    runner.set_viewport_size(720.0, 720.0);
    runner.run_frame();
    runner.run_frame();
    runner.run_frame();
    runner.run_frame();

    let (_narrow_grid_position, narrow_grid_size) = runner.get_element_bounds("apps-grid");
    let (narrow_viewport_position, narrow_viewport_size) =
        runner.get_element_bounds("apps-grid-viewport");

    assert!(
        wide_grid_size.x >= narrow_grid_size.x + 400.0,
        "grid wrapper should release its old minimum width after shrinking the window, wide={wide_grid_size:?}, narrow={narrow_grid_size:?}"
    );
    assert!(
        wide_viewport_size.x >= narrow_viewport_size.x + 400.0,
        "grid viewport should shrink with the window width, wide={wide_viewport_size:?}, narrow={narrow_viewport_size:?}"
    );
    assert!(
        narrow_viewport_position.x.abs() < 0.5,
        "grid viewport should still start at the window edge after shrinking, position={narrow_viewport_position:?}"
    );
    assert!(
        (narrow_viewport_position.x + narrow_viewport_size.x - 720.0).abs() < 0.5,
        "grid viewport should match the narrowed window width, position={narrow_viewport_position:?}, size={narrow_viewport_size:?}"
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

    let (_active_position, active_size) =
        runner.get_element_bounds("apps-grid-page-dot-active-visual");
    let (_focused_position, focused_size) =
        runner.get_element_bounds("apps-grid-page-dot-visual-1");

    assert!(
        active_size.x > focused_size.x,
        "focused inactive page dot should keep the inactive visual size"
    );
}

#[test]
fn focused_active_page_dot_shows_pill_shaped_focus_ring() {
    let mut runner = TestRunner::new(HomeTestApp);
    runner.set_viewport_size(1280.0, 720.0);
    runner.run_frame();
    runner.run_frame();

    runner.focus_element_by_key(
        FocusKey::new("apps-grid-page-dot-0"),
        FocusOrigin::Navigation,
    );
    runner.run_frame();

    let (ring_position, ring_size) = runner.get_element_bounds("apps-grid-page-dot-focus-ring");
    let (pill_position, pill_size) = runner.get_element_bounds("apps-grid-page-dot-active-visual");
    let (_target_position, target_size) = runner.get_element_bounds("apps-grid-page-dot-0");

    assert!(
        ring_size.x > target_size.x + 0.5,
        "focus ring should expand to the active pill width on the active page"
    );
    assert!(
        (ring_position.x + ring_size.x * 0.5 - (pill_position.x + pill_size.x * 0.5)).abs() < 0.5,
        "focus ring should stay centered on the active pill"
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
        (active_size.x - 16.0).abs() < 0.5,
        "page dot targets should stay compact even when the active indicator is a pill"
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

    let (_active_position, active_size) =
        runner.get_element_bounds("apps-grid-page-dot-active-visual");
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
fn active_page_indicator_stays_inside_the_page_dot_track_bounds() {
    let mut runner = TestRunner::new(HomeTestApp);
    runner.set_viewport_size(1280.0, 720.0);
    runner.run_frame();
    runner.run_frame();

    let (track_position, track_size) = runner.get_element_bounds("apps-grid-page-dots");
    let (first_position, first_size) =
        runner.get_element_bounds("apps-grid-page-dot-active-visual");
    assert!(
        first_position.x >= track_position.x - 0.5,
        "active page indicator should not clip past the left edge of the track"
    );
    assert!(
        first_position.x + first_size.x <= track_position.x + track_size.x + 0.5,
        "active page indicator should stay inside the track width on the first page"
    );

    runner.click_element("apps-grid-page-dot-2");
    runner.run_frame();
    thread::sleep(Duration::from_millis(260));
    runner.run_frame();

    let (last_position, last_size) = runner.get_element_bounds("apps-grid-page-dot-active-visual");
    assert!(
        last_position.x >= track_position.x - 0.5,
        "active page indicator should stay inside the track width on the last page"
    );
    assert!(
        last_position.x + last_size.x <= track_position.x + track_size.x + 0.5,
        "active page indicator should not clip past the right edge of the track"
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
fn first_page_dot_up_targets_the_middle_column_when_the_grid_has_three_columns() {
    let mut runner = TestRunner::new(FixedWidthGridTestApp);
    runner.set_viewport_size(1280.0, 720.0);
    runner.run_frame();
    runner.run_frame();

    assert_eq!(columns_for_width(960.0 - SCREEN_PADDING * 2.0), 3);

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

    assert_eq!(
        focused_tag, "browser",
        "focus should move to the middle column of the last visible row on the first page, got {focused_tag}"
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
fn coordinate_clicking_page_dot_jumps_to_that_page() {
    let mut runner = TestRunner::new(HomeTestApp);
    runner.set_viewport_size(1280.0, 720.0);
    runner.run_frame();
    runner.run_frame();

    let dot_center = runner.get_element_center("apps-grid-page-dot-1");
    let (initial_position, _initial_size) = runner.get_element_bounds("apps-grid-page-1");
    runner.click_primary_button(dot_center);
    runner.run_frame();
    thread::sleep(Duration::from_millis(260));
    runner.run_frame();

    let (position, _size) = runner.get_element_bounds("apps-grid-page-1");
    assert!(
        position.x < initial_position.x - 100.0,
        "clicking the actual page-dot position should move that page toward the viewport"
    );
}

#[test]
fn coordinate_clicking_focused_page_dot_jumps_to_that_page() {
    let mut runner = TestRunner::new(HomeTestApp);
    runner.set_viewport_size(1280.0, 720.0);
    runner.run_frame();
    runner.run_frame();

    runner.focus_element_by_key(
        FocusKey::new("apps-grid-page-dot-1"),
        FocusOrigin::Navigation,
    );
    runner.run_frame();

    let dot_center = runner.get_element_center("apps-grid-page-dot-1");
    let (initial_position, _initial_size) = runner.get_element_bounds("apps-grid-page-1");
    runner.click_primary_button(dot_center);
    runner.run_frame();
    thread::sleep(Duration::from_millis(260));
    runner.run_frame();

    let (position, _size) = runner.get_element_bounds("apps-grid-page-1");
    assert!(
        position.x < initial_position.x - 100.0,
        "clicking a focused page dot through the ring should move that page toward the viewport"
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
                Vec2::new(0.0, 2.0),
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
                Vec2::new(0.0, 2.0),
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
        "scrolling down should move the next page toward the viewport, initial_x={}, current_x={}",
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
                Vec2::new(0.0, -2.0),
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
        "scrolling up should move the previous page back toward the viewport"
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
