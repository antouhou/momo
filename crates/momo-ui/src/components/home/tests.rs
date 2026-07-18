use super::{
    Home,
    bluetooth::initialize_bluetooth_state,
    compositor::{CompositorEventInbox, HOME_COMPOSITOR_EVENT_INBOX_STATE_ID},
    model::TILE_HEIGHT,
    system_status::initialize_system_status_state,
};
use crate::app_state::{APPS_STATE_ID, AppEntry, AppsState};
use daiko::{
    App, AppContext, Id, Pos2, SurfaceId, Vec2,
    integration::{
        AppMessage, SurfaceCommand, SurfaceKeyboardInteractivity, SurfaceLayer,
        input::{InputEvent, InputEventModifiers},
    },
    navigation::{FocusKey, FocusOrigin},
    style::{Color, Transform},
    testing::TestRunner,
    window_events::WindowEvent,
};
use momo_compositor::{CompositorAction, CompositorEvent};
use std::{
    path::PathBuf,
    sync::{Arc, mpsc},
    time::{Duration, Instant},
};
use system_control::SystemControl;

const ASYNC_TEST_TIMEOUT: Duration = Duration::from_secs(2);
const ASYNC_TEST_POLL_INTERVAL: Duration = Duration::from_millis(1);

fn initialize_test_app_state(ctx: &mut AppContext) {
    let apps_state = ctx.peek_global_state(Id::new(APPS_STATE_ID), AppsState::default);
    let mut apps_state = apps_state.write();
    apps_state.is_loading = false;
    apps_state.app_entries = test_apps();
}

fn test_apps() -> Vec<AppEntry> {
    [
        ("live-tv", "Live TV"),
        ("movies", "Movies"),
        ("music", "Music"),
        ("photos", "Photos"),
        ("browser", "Browser"),
        ("settings", "Settings"),
        ("games", "Games"),
        ("store", "Store"),
        ("search", "Search"),
        ("camera", "Camera"),
        ("calendar", "Calendar"),
        ("weather", "Weather"),
        ("sports", "Sports"),
        ("news", "News"),
        ("kids", "Kids"),
        ("fitness", "Fitness"),
        ("radio", "Radio"),
        ("podcasts", "Podcasts"),
        ("files", "Files"),
        ("gallery", "Gallery"),
        ("mail", "Mail"),
        ("maps", "Maps"),
        ("notes", "Notes"),
        ("contacts", "Contacts"),
        ("assistant", "Assistant"),
    ]
    .into_iter()
    .map(|(id, name)| AppEntry {
        id: Arc::new(id.to_string()),
        name: Arc::new(name.to_string()),
        icon: Arc::new(None::<PathBuf>),
        accent: Color::from_rgb(0, 125, 215),
    })
    .collect()
}

struct HomeTestApp;

impl App for HomeTestApp {
    type RootComponent = Home;

    fn create(&mut self, ctx: &mut AppContext) -> Self::RootComponent {
        let system_control =
            SystemControl::new().expect("failed to initialize system control for tests");
        initialize_bluetooth_state(ctx, system_control.bluetooth());
        initialize_system_status_state(ctx, system_control.volume(), system_control.battery());
        initialize_test_app_state(ctx);
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
fn app_grid_height_shrinks_after_window_height_shrinks() {
    let mut runner = TestRunner::new(HomeTestApp);
    runner.set_viewport_size(1280.0, 980.0);
    runner.run_frame();

    let (_tall_position, tall_size) = runner.get_element_bounds("apps-grid-viewport");

    runner.set_viewport_size(1280.0, 560.0);
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

    let (_tall_position, tall_size) = runner.get_element_bounds("apps-grid");

    runner.set_viewport_size(1280.0, 560.0);
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

    let (_wide_grid_position, wide_grid_size) = runner.get_element_bounds("apps-grid");
    let (_wide_viewport_position, wide_viewport_size) =
        runner.get_element_bounds("apps-grid-viewport");

    runner.set_viewport_size(720.0, 720.0);
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
fn inactive_page_dot_target_keeps_compact_width() {
    let mut runner = TestRunner::new(HomeTestApp);
    runner.set_viewport_size(1280.0, 720.0);
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
fn vertical_wheel_scroll_pages_the_grid() {
    let mut runner = TestRunner::new(HomeTestApp);
    runner.set_viewport_size(1280.0, 720.0);
    runner.run_frame();

    let (first_tile_position, first_tile_size) = runner.get_element_bounds("live-tv");
    let first_tile_center = Pos2::new(
        first_tile_position.x + first_tile_size.x / 2.0,
        first_tile_position.y + first_tile_size.y / 2.0,
    );
    let (initial_position, _initial_size) = runner.get_element_bounds("apps-grid-page-1");

    runner.move_pointer_to(first_tile_center);
    for _ in 0..4 {
        runner.app_runner_mut().context.add_input_event(
            SurfaceId::ROOT,
            InputEvent::scroll(
                Vec2::new(0.0, 2.0),
                InputEventModifiers::default(),
                Instant::now(),
            ),
        );
        runner.run_frame();
    }
    for _ in 0..8 {
        runner.app_runner_mut().context.add_input_event(
            SurfaceId::ROOT,
            InputEvent::scroll(
                Vec2::new(0.0, 2.0),
                InputEventModifiers::default(),
                Instant::now(),
            ),
        );
        runner.run_frame();
    }
    run_until(&mut runner, "first page scroll animation", |runner| {
        runner.get_element_bounds("apps-grid-page-1").0.x < initial_position.x - 100.0
    });

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
    run_until(&mut runner, "return page scroll animation", |runner| {
        runner.app_runner_mut().context.add_input_event(
            SurfaceId::ROOT,
            InputEvent::scroll(
                Vec2::new(0.0, -2.0),
                InputEventModifiers::default(),
                Instant::now(),
            ),
        );
        runner.get_element_bounds("apps-grid-page-0").0.x > first_page_scrolled_position.x + 100.0
    });

    let (first_page_returned_position, _first_page_returned_size) =
        runner.get_element_bounds("apps-grid-page-0");
    assert!(
        first_page_returned_position.x > first_page_scrolled_position.x + 100.0,
        "scrolling up should move the previous page back toward the viewport"
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
fn launch_surface_starts_at_the_focused_tile_visual_bounds() {
    let mut runner = TestRunner::new(HomeTestApp);
    runner.set_viewport_size(1280.0, 720.0);
    runner.run_frame();

    runner.focus_element_by_key(FocusKey::new("movies"), FocusOrigin::Navigation);
    run_until(&mut runner, "focused tile transform", |runner| {
        let entry = runner
            .find_element_entry_by_tag("movies")
            .expect("movies tile should exist while waiting for focus transform");
        let (_, rendered_size) = rendered_element_bounds(
            entry.layout.position_absolute,
            entry.layout.size,
            entry.effective_transform.as_ref(),
        );
        rendered_size.x >= entry.layout.size.x * 1.049
    });
    let focused_tile_entry = runner
        .find_element_entry_by_tag("movies")
        .expect("movies tile should exist before launch");
    let focused_tile_bounds = rendered_element_bounds(
        focused_tile_entry.layout.position_absolute,
        focused_tile_entry.layout.size,
        focused_tile_entry.effective_transform.as_ref(),
    );

    runner.press_confirm();

    let launch_surface_entry = runner
        .find_element_entry_by_tag("launch-overlay-surface")
        .expect("launch surface should exist after activation");
    let launch_surface_bounds = rendered_element_bounds(
        launch_surface_entry.layout.position_absolute,
        launch_surface_entry.layout.size,
        launch_surface_entry.effective_transform.as_ref(),
    );

    assert_rect_near(
        launch_surface_bounds,
        focused_tile_bounds,
        "launch surface should match focused tile",
    );
}

#[test]
fn cancel_reverses_launch_overlay_and_restores_tile_focus() {
    let mut runner = TestRunner::new(HomeTestApp);
    runner.set_viewport_size(1280.0, 720.0);
    runner.run_frame();

    runner.click_element("movies");
    runner.run_frame();
    wait_for_launch_expansion(&mut runner);

    runner.press_cancel();
    run_until(&mut runner, "launch overlay contraction", |runner| {
        runner.find_element_by_tag("launch-overlay").is_none()
    });

    assert!(runner.find_element_by_tag("launch-overlay").is_none());
    runner.assert_focused("movies");
}

#[test]
fn window_focus_loss_reverses_launch_overlay() {
    let mut runner = TestRunner::new(HomeTestApp);
    runner.set_viewport_size(1280.0, 720.0);
    runner.run_frame();

    runner.click_element("movies");
    runner.run_frame();
    wait_for_launch_expansion(&mut runner);

    runner
        .app_runner_mut()
        .context
        .add_window_event(SurfaceId::ROOT, WindowEvent::focus_lost(Instant::now()));
    runner.run_frame();
    run_until(&mut runner, "focus-loss launch contraction", |runner| {
        runner.find_element_by_tag("launch-overlay").is_none()
    });

    assert!(runner.find_element_by_tag("launch-overlay").is_none());
}

#[test]
fn launch_moves_shell_to_background_only_after_window_focus_is_lost() {
    let mut runner = TestRunner::new(HomeTestApp);
    runner.set_viewport_size(1280.0, 720.0);
    runner.run_frame();
    let (app_message_sender, app_message_receiver) = mpsc::channel();
    runner
        .app_runner_mut()
        .context
        .set_app_message_bus(app_message_sender);

    runner.click_element("movies");
    runner.run_frame();

    assert!(
        !received_surface_layer(&app_message_receiver, SurfaceLayer::Background),
        "launching should keep the shell on its current layer until the app takes focus"
    );

    runner
        .app_runner_mut()
        .context
        .add_window_event(SurfaceId::ROOT, WindowEvent::focus_lost(Instant::now()));
    runner.run_frame();

    assert!(
        received_surface_layer(&app_message_receiver, SurfaceLayer::Background),
        "the shell should move to the background when the launched app takes focus"
    );
}

#[test]
fn window_focus_gain_moves_shell_to_top_layer() {
    let mut runner = TestRunner::new(HomeTestApp);
    runner.set_viewport_size(1280.0, 720.0);
    runner.run_frame();
    let (app_message_sender, app_message_receiver) = mpsc::channel();
    runner
        .app_runner_mut()
        .context
        .set_app_message_bus(app_message_sender);

    runner
        .app_runner_mut()
        .context
        .add_window_event(SurfaceId::ROOT, WindowEvent::focus_gained(Instant::now()));
    runner.run_frame();

    assert!(
        received_surface_layer(&app_message_receiver, SurfaceLayer::Top),
        "the shell should move to the top layer when it regains focus"
    );
}

fn request_compositor_launcher_toggle(runner: &mut TestRunner<HomeTestApp>) {
    let event_inbox = runner.app_runner_mut().context.peek_global_state(
        Id::new(HOME_COMPOSITOR_EVENT_INBOX_STATE_ID),
        CompositorEventInbox::default,
    );
    event_inbox
        .write()
        .pending_events
        .push(CompositorEvent::ActionActivated(
            CompositorAction::ToggleLauncher,
        ));
}

fn received_surface_commands(
    app_message_receiver: &mpsc::Receiver<AppMessage>,
) -> Vec<SurfaceCommand> {
    app_message_receiver
        .try_iter()
        .filter_map(|message| match message {
            AppMessage::SurfaceCommand(SurfaceId::ROOT, command) => Some(command),
            _ => None,
        })
        .collect()
}

fn received_surface_layer(
    app_message_receiver: &mpsc::Receiver<AppMessage>,
    expected_layer: SurfaceLayer,
) -> bool {
    app_message_receiver.try_iter().any(|message| {
        matches!(
            message,
            AppMessage::SurfaceCommand(
                SurfaceId::ROOT,
                SurfaceCommand::SetLayer(layer)
            ) if layer == expected_layer
        )
    })
}

fn wait_for_launch_expansion(runner: &mut TestRunner<HomeTestApp>) {
    run_until(runner, "launch overlay expansion", |runner| {
        runner
            .find_element_entry_by_tag("launch-overlay-surface")
            .is_some_and(|entry| entry.layout.size.x >= 1279.0 && entry.layout.size.y >= 719.0)
    });
}

fn run_until(
    runner: &mut TestRunner<HomeTestApp>,
    description: &str,
    mut condition: impl FnMut(&mut TestRunner<HomeTestApp>) -> bool,
) {
    let deadline = Instant::now() + ASYNC_TEST_TIMEOUT;
    while Instant::now() < deadline {
        runner.run_frame();
        if condition(runner) {
            return;
        }
        std::thread::sleep(ASYNC_TEST_POLL_INTERVAL);
    }

    panic!("timed out waiting for {description}");
}

fn rendered_element_bounds(
    position_absolute: Vec2,
    size: Vec2,
    effective_transform: Option<&Transform>,
) -> (Vec2, Vec2) {
    let Some(transform) = effective_transform else {
        return (position_absolute, size);
    };
    let corners: [(f32, f32); 4] = [
        transform.transform_local_point2d_to_world(0.0, 0.0),
        transform.transform_local_point2d_to_world(size.x, 0.0),
        transform.transform_local_point2d_to_world(0.0, size.y),
        transform.transform_local_point2d_to_world(size.x, size.y),
    ];
    let (min_x, max_x) = corners
        .iter()
        .map(|(x, _)| *x)
        .fold((f32::INFINITY, f32::NEG_INFINITY), |(min_x, max_x), x| {
            (min_x.min(x), max_x.max(x))
        });
    let (min_y, max_y) = corners
        .iter()
        .map(|(_, y)| *y)
        .fold((f32::INFINITY, f32::NEG_INFINITY), |(min_y, max_y), y| {
            (min_y.min(y), max_y.max(y))
        });

    (
        Vec2::new(min_x, min_y),
        Vec2::new(max_x - min_x, max_y - min_y),
    )
}

fn assert_vec2_near(actual: Vec2, expected: Vec2, message: &str) {
    let delta = actual - expected;
    assert!(
        delta.x.abs() <= 0.5 && delta.y.abs() <= 0.5,
        "{message}: expected {expected:?}, got {actual:?}"
    );
}

fn assert_rect_near(actual: (Vec2, Vec2), expected: (Vec2, Vec2), message: &str) {
    assert_vec2_near(
        actual.0,
        expected.0,
        &format!("{message} position; expected rect {expected:?}, got {actual:?}"),
    );
    assert_vec2_near(
        actual.1,
        expected.1,
        &format!("{message} size; expected rect {expected:?}, got {actual:?}"),
    );
}
