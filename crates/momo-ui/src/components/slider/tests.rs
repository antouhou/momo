use daiko::{
    App, AppContext, Element, Id, Pos2, Vec2, layout::ItemSize, style::Style, testing::TestRunner,
};
use super::{Component, ComponentContext, DEFAULT_MAX_VALUE, Slider};

const TEST_SLIDER_STATE_ID: &str = "test-slider-state";
const TEST_SLIDER_DEFAULT_VALUE: u8 = 40;
const TEST_SLIDER_HOST_TAG: &str = "slider-test-host";
const TEST_SLIDER_REMOUNT_HOST_TAG: &str = "slider-remount-host";
const TEST_SLIDER_REMOUNT_STATE_ID: &str = "slider-remount-state";
const TEST_SLIDER_THUMB_SIZE: f32 = 16.0;

struct SliderTestApp;

impl App for SliderTestApp {
    type RootComponent = SliderTestRoot;

    fn create(&mut self, _ctx: &mut AppContext) -> Self::RootComponent {
        SliderTestRoot
    }

    fn stop(&mut self, _ctx: &mut AppContext) {}
}

#[derive(Clone, Copy)]
struct SliderTestRoot;

impl Component for SliderTestRoot {
    fn to_element(&self, _ctx: &mut ComponentContext) -> Element {
        Element::new()
            .with_tag("slider-test-root")
            .with_style(Style::new().with_fixed_size(300.0, 120.0))
            .with_content(
                Element::new()
                    .with_tag(TEST_SLIDER_HOST_TAG)
                    .with_style(
                        Style::new()
                            .with_fixed_width(ItemSize::Points(160.0))
                            .with_fixed_height(ItemSize::Points(24.0)),
                    )
                    .with_content(
                        Slider::new(TEST_SLIDER_STATE_ID)
                            .default_value(TEST_SLIDER_DEFAULT_VALUE)
                            .track_height(8.0)
                            .thumb_size(TEST_SLIDER_THUMB_SIZE),
                    ),
            )
    }
}

struct SliderRemountTestApp;

impl App for SliderRemountTestApp {
    type RootComponent = SliderRemountTestRoot;

    fn create(&mut self, _ctx: &mut AppContext) -> Self::RootComponent {
        SliderRemountTestRoot
    }

    fn stop(&mut self, _ctx: &mut AppContext) {}
}

#[derive(Clone, Copy)]
struct SliderRemountTestRoot;

impl Component for SliderRemountTestRoot {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let show_slider = ctx.use_shared_state(Id::new(TEST_SLIDER_REMOUNT_STATE_ID), || false);

        let content = if *show_slider.read() {
            Element::new()
                .with_tag(TEST_SLIDER_REMOUNT_HOST_TAG)
                .with_style(
                    Style::new()
                        .with_fixed_width(ItemSize::Points(160.0))
                        .with_fixed_height(ItemSize::Points(24.0)),
                )
                .with_content(
                    Slider::new(TEST_SLIDER_STATE_ID)
                        .default_value(DEFAULT_MAX_VALUE)
                        .track_height(8.0)
                        .thumb_size(TEST_SLIDER_THUMB_SIZE),
                )
        } else {
            Element::new()
                .with_tag(TEST_SLIDER_REMOUNT_HOST_TAG)
                .with_style(
                    Style::new()
                        .with_fixed_width(ItemSize::Points(260.0))
                        .with_fixed_height(ItemSize::Points(24.0)),
                )
                .with_content(Element::new().with_style(Style::new().with_fixed_size(260.0, 24.0)))
        };

        Element::new()
            .with_tag("slider-remount-root")
            .with_style(Style::new().with_fixed_size(300.0, 120.0))
            .with_content(content)
    }
}

#[test]
fn clicking_outside_slider_after_thumb_initializes_does_not_change_value() {
    let mut runner = TestRunner::new(SliderTestApp);
    runner.set_viewport_size(300.0, 120.0);
    runner.run_frame();
    runner.run_frame();

    runner.click_primary_button(Pos2::new(260.0, 90.0));
    runner.run_frame();

    assert_eq!(slider_value(&mut runner), TEST_SLIDER_DEFAULT_VALUE);
}

#[test]
fn clicking_inside_slider_still_changes_value() {
    let mut runner = TestRunner::new(SliderTestApp);
    runner.set_viewport_size(300.0, 120.0);
    runner.run_frame();

    runner.click_primary_button(Pos2::new(152.0, 8.0));
    runner.run_frame();

    assert_eq!(slider_value(&mut runner), DEFAULT_MAX_VALUE);
}

#[test]
fn remounted_slider_snaps_thumb_to_current_layout() {
    let mut runner = TestRunner::new(SliderRemountTestApp);
    runner.set_viewport_size(300.0, 120.0);
    runner.run_frame();
    runner.run_frame();

    *runner
        .app_runner_mut()
        .context
        .peek_shared_state(Id::new(TEST_SLIDER_REMOUNT_STATE_ID), || false)
        .write() = true;
    runner.run_frame();

    let host_x = element_bounds(&runner, TEST_SLIDER_REMOUNT_HOST_TAG).0.x;
    let expected_thumb_x = host_x + 160.0 - TEST_SLIDER_THUMB_SIZE;
    let actual_thumb_x = slider_thumb_x(&runner);

    assert!(
        (actual_thumb_x - expected_thumb_x).abs() < 0.5,
        "remounted thumb should snap to current layout, expected={expected_thumb_x}, actual={actual_thumb_x}"
    );
}

fn slider_value(runner: &mut TestRunner<SliderTestApp>) -> u8 {
    *runner
        .app_runner_mut()
        .context
        .peek_shared_state(Id::new(TEST_SLIDER_STATE_ID), || TEST_SLIDER_DEFAULT_VALUE)
        .read()
}

fn slider_thumb_x(runner: &TestRunner<SliderRemountTestApp>) -> f32 {
    let host_id = element_id(runner, TEST_SLIDER_REMOUNT_HOST_TAG);
    let slider_id = runner.tree().children_ids(&host_id)[0];
    let thumb_id = runner.tree().children_ids(&slider_id)[1];

    runner.tree().layout(&thumb_id).unwrap().position_absolute.x
}

fn element_bounds<TApp: App>(runner: &TestRunner<TApp>, tag: &str) -> (Vec2, Vec2) {
    let element_id = element_id(runner, tag);
    let layout = runner.tree().layout(&element_id).unwrap();

    (layout.position_absolute, layout.size)
}

fn element_id<TApp: App>(runner: &TestRunner<TApp>, tag: &str) -> usize {
    runner
        .tree()
        .elements()
        .find_map(|(element_id, element)| (element.tag() == Some(tag)).then_some(element_id))
        .unwrap_or_else(|| panic!("Element with tag '{tag}' not found"))
}
