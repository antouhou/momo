use super::{
    DEFAULT_ACTIVATION_THRESHOLD, DEFAULT_REARM_DURATION, PageScrollDirection, ScrollPagingAxis,
    ScrollPagingBehavior, ScrollPagingState,
};
use daiko::{
    Element, Id, SurfaceId, Vec2,
    component::{Component, ComponentContext},
    integration::input::{InputEvent, InputEventModifiers},
    style::Style,
    testing::{SimpleTestApp, TestRunner},
};
use std::time::{Duration, Instant};

const SCROLL_PAGING_TEST_STATE_ID: &str = "momo_kit_scroll_paging_test_state";

#[derive(Clone, Copy)]
struct ConfiguredScrollPagingProbe;

impl Component for ConfiguredScrollPagingProbe {
    fn to_element(&self, context: &mut ComponentContext) -> Element {
        let page_index = context.use_local_state(|| 0_isize);
        if let Some(direction) =
            ScrollPagingBehavior::new(context, Id::new(SCROLL_PAGING_TEST_STATE_ID))
                .with_activation_threshold(2.0)
                .with_rearm_duration(Duration::ZERO)
                .with_scroll_axis(ScrollPagingAxis::Horizontal)
                .apply()
        {
            *page_index.write() += direction.page_delta();
        }

        Element::new()
            .with_tag(format!("page-{}", *page_index.read()))
            .with_style(Style::new().with_fixed_size(100.0, 100.0))
    }
}

#[test]
fn scroll_delta_accumulates_until_the_default_threshold() {
    let now = Instant::now();
    let mut state = ScrollPagingState::default();

    assert_eq!(
        state.apply_delta(
            3.0,
            now,
            DEFAULT_ACTIVATION_THRESHOLD,
            DEFAULT_REARM_DURATION
        ),
        None
    );
    assert_eq!(
        state.apply_delta(
            5.0,
            now,
            DEFAULT_ACTIVATION_THRESHOLD,
            DEFAULT_REARM_DURATION
        ),
        Some(PageScrollDirection::Next)
    );
}

#[test]
fn custom_threshold_and_rearm_duration_are_applied() {
    let now = Instant::now();
    let custom_threshold = 2.0;
    let custom_rearm_duration = Duration::from_millis(50);
    let mut state = ScrollPagingState::default();

    assert_eq!(
        state.apply_delta(
            custom_threshold,
            now,
            custom_threshold,
            custom_rearm_duration
        ),
        Some(PageScrollDirection::Next)
    );
    assert_eq!(
        state.apply_delta(
            -custom_threshold,
            now + Duration::from_millis(49),
            custom_threshold,
            custom_rearm_duration,
        ),
        None
    );
    assert_eq!(
        state.apply_delta(
            -custom_threshold,
            now + custom_rearm_duration,
            custom_threshold,
            custom_rearm_duration,
        ),
        Some(PageScrollDirection::Previous)
    );
}

#[test]
fn paging_axis_can_be_configured() {
    let scroll_delta = Vec2::new(6.0, -4.0);

    assert_eq!(ScrollPagingAxis::Horizontal.select_delta(scroll_delta), 6.0);
    assert_eq!(ScrollPagingAxis::Vertical.select_delta(scroll_delta), -4.0);
    assert_eq!(
        ScrollPagingAxis::VerticalWithHorizontalFallback.select_delta(scroll_delta),
        -4.0
    );
    assert_eq!(
        ScrollPagingAxis::VerticalWithHorizontalFallback.select_delta(Vec2::new(6.0, 0.0)),
        6.0
    );
}

#[test]
fn builder_configuration_controls_live_scroll_interactions() {
    let mut runner = TestRunner::new(SimpleTestApp::new(
        Element::new().with_content(ConfiguredScrollPagingProbe),
    ));
    runner.set_viewport_size(100.0, 100.0);
    runner.run_frame();
    runner.hover_element("page-0");
    runner.run_frame();

    send_scroll(&mut runner, Vec2::new(2.0, -100.0));
    assert!(runner.find_element_by_tag("page-1").is_some());

    send_scroll(&mut runner, Vec2::new(-2.0, 100.0));
    assert!(runner.find_element_by_tag("page-0").is_some());
}

#[test]
fn page_directions_have_signed_page_deltas() {
    assert_eq!(PageScrollDirection::Previous.page_delta(), -1);
    assert_eq!(PageScrollDirection::Next.page_delta(), 1);
}

fn send_scroll(runner: &mut TestRunner<SimpleTestApp>, scroll_delta: Vec2) {
    runner.app_runner_mut().context.add_input_event(
        SurfaceId::ROOT,
        InputEvent::scroll(scroll_delta, InputEventModifiers::default(), Instant::now()),
    );
    runner.run_frame();
    runner.run_frame();
}
