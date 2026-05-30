mod style;

use self::style::{slider_fill_style, slider_root_style, slider_thumb_style, slider_track_style};
use daiko::animation::SmoothFollowConfig;
use daiko::component::{Component, ComponentContext};
use daiko::style::Color;
use daiko::{Element, Id, Pos2, Rect};
use std::hash::Hash;
use std::time::Duration;

const DEFAULT_MIN_VALUE: u8 = 0;
const DEFAULT_MAX_VALUE: u8 = 100;
const DEFAULT_TRACK_WIDTH_FALLBACK: f32 = 160.0;
const DEFAULT_TRACK_HEIGHT: f32 = 8.0;
const DEFAULT_THUMB_SIZE: f32 = 16.0;
const DEFAULT_TRACK_COLOR: Color = Color::from_rgb(88, 92, 97);
const DEFAULT_FILL_COLOR: Color = Color::from_rgb(190, 194, 199);
const DEFAULT_THUMB_COLOR: Color = Color::from_rgb(236, 240, 243);
const DEFAULT_THUMB_BORDER_COLOR: Color = Color::from_rgba_premultiplied(165, 165, 165, 96);
const DEFAULT_HIGHLIGHTED_THUMB_BORDER_COLOR: Color =
    Color::from_rgba_premultiplied(212, 212, 212, 168);
const MIN_SMOOTH_FOLLOW_DURATION_MS: u64 = 180;
const SMOOTH_FOLLOW_CURRENT_VELOCITY_WEIGHTING: f32 = 0.3;
const SMOOTH_FOLLOW_STOP_DECELERATION_WEIGHTING: f32 = 0.36;
const THUMB_OFFSET_SMOOTH_FOLLOW_SUFFIX: &str = "slider-thumb-offset";

#[derive(Clone, Copy, Default)]
struct SliderLocalState {
    is_drag_active: bool,
    is_thumb_offset_initialized: bool,
}

#[derive(Clone, Copy)]
pub(crate) struct Slider {
    id: Id,
    default_value: u8,
    min: u8,
    max: u8,
    track_width: Option<f32>,
    track_height: f32,
    thumb_size: f32,
    track_color: Color,
    fill_color: Color,
    thumb_color: Color,
    thumb_border_color: Color,
    highlighted_thumb_border_color: Color,
    is_highlighted: bool,
}

impl Slider {
    pub(crate) fn new(id: impl Hash) -> Self {
        Self {
            id: Id::new(id),
            default_value: DEFAULT_MIN_VALUE,
            min: DEFAULT_MIN_VALUE,
            max: DEFAULT_MAX_VALUE,
            track_width: None,
            track_height: DEFAULT_TRACK_HEIGHT,
            thumb_size: DEFAULT_THUMB_SIZE,
            track_color: DEFAULT_TRACK_COLOR,
            fill_color: DEFAULT_FILL_COLOR,
            thumb_color: DEFAULT_THUMB_COLOR,
            thumb_border_color: DEFAULT_THUMB_BORDER_COLOR,
            highlighted_thumb_border_color: DEFAULT_HIGHLIGHTED_THUMB_BORDER_COLOR,
            is_highlighted: false,
        }
    }

    pub(crate) fn default_value(mut self, default_value: u8) -> Self {
        self.default_value = default_value;
        self
    }

    pub(crate) fn range(mut self, min: u8, max: u8) -> Self {
        let (min, max) = normalized_bounds(min, max);
        self.min = min;
        self.max = max;
        self
    }

    pub(crate) fn track_height(mut self, height: f32) -> Self {
        self.track_height = height;
        self
    }

    pub(crate) fn thumb_size(mut self, size: f32) -> Self {
        self.thumb_size = size;
        self
    }

    pub(crate) fn track_color(mut self, color: Color) -> Self {
        self.track_color = color;
        self
    }

    pub(crate) fn fill_color(mut self, color: Color) -> Self {
        self.fill_color = color;
        self
    }

    pub(crate) fn thumb_color(mut self, color: Color) -> Self {
        self.thumb_color = color;
        self
    }

    pub(crate) fn thumb_border_colors(
        mut self,
        border_color: Color,
        highlighted_border_color: Color,
    ) -> Self {
        self.thumb_border_color = border_color;
        self.highlighted_thumb_border_color = highlighted_border_color;
        self
    }

    pub(crate) fn highlighted(mut self, is_highlighted: bool) -> Self {
        self.is_highlighted = is_highlighted;
        self
    }

    fn clamp_value(self, value: i16) -> u8 {
        clamp_slider_value(value, self.min, self.max)
    }

    fn clamped_default_value(self) -> u8 {
        self.clamp_value(i16::from(self.default_value))
    }
}

impl Component for Slider {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let default_value = self.clamped_default_value();
        let value = ctx.use_shared_state(self.id, move || default_value);
        let local_state = ctx.use_local_state(SliderLocalState::default);
        let raw_current_value = *value.read();
        let mut current_value = self.clamp_value(i16::from(raw_current_value));
        let SliderLocalState {
            mut is_drag_active,
            is_thumb_offset_initialized,
        } = *local_state.read();
        let mut pointer = ctx.pointer();
        let pointer_position = pointer.current_position();
        let just_pressed = pointer.just_pressed();
        let is_pressed = pointer.is_pressed();
        let is_dragging = pointer.is_dragging();
        let just_released_anywhere = pointer.just_released_anywhere();

        if current_value != raw_current_value {
            *value.write_silent() = current_value;
        }

        let layout = ctx.layout();
        let track_area = layout.map(slider_track_area);
        let has_resolved_track_width = layout.is_some() || self.track_width.is_some();
        let rendered_track_width = slider_track_width(*self, track_area);
        let pressed_inside_track = just_pressed
            && pointer_position
                .zip(track_area)
                .is_some_and(|(position, area)| is_pointer_inside_track(position, area));

        if pressed_inside_track {
            is_drag_active = true;
            local_state.write_silent().is_drag_active = is_drag_active;
        }

        if is_drag_active
            && (is_pressed || is_dragging || just_released_anywhere)
            && let Some(position) = pointer_position
        {
            current_value =
                slider_value_from_track_position(position, track_area, rendered_track_width, *self);
            *value.write_silent() = current_value;
        }

        if is_drag_active && (just_released_anywhere || !is_pressed && !is_dragging) {
            is_drag_active = false;
            local_state.write_silent().is_drag_active = is_drag_active;
        }

        let target_thumb_offset = slider_thumb_offset(current_value, rendered_track_width, *self);
        let mut smooth_thumb_offset = ctx.smooth_follow_with_id::<f32>(
            self.id.with(THUMB_OFFSET_SMOOTH_FOLLOW_SUFFIX),
            slider_smooth_follow_config(),
        );
        let rendered_thumb_offset = if !is_thumb_offset_initialized && !has_resolved_track_width {
            smooth_thumb_offset.reset_to(target_thumb_offset);
            target_thumb_offset
        } else if !is_thumb_offset_initialized || is_drag_active {
            smooth_thumb_offset.reset_to(target_thumb_offset);
            if !is_thumb_offset_initialized {
                local_state.write_silent().is_thumb_offset_initialized = true;
            }
            target_thumb_offset
        } else {
            smooth_thumb_offset.follow(target_thumb_offset)
        };

        Element::new()
            .with_style(slider_root_style(*self))
            .with_content(
                Element::new()
                    .with_style(slider_track_style(rendered_track_width, *self))
                    .with_content(Element::new().with_style(slider_fill_style(
                        rendered_track_width,
                        *self,
                        slider_fill_width(rendered_thumb_offset, *self),
                    ))),
            )
            .with_content(Element::new().with_style(slider_thumb_style(
                *self,
                rendered_thumb_offset,
                ctx,
            )))
    }
}

pub(crate) fn clamp_slider_value(value: i16, min: u8, max: u8) -> u8 {
    let (min, max) = normalized_bounds(min, max);
    value.clamp(i16::from(min), i16::from(max)) as u8
}

fn slider_thumb_offset(current_value: u8, track_width: f32, slider: Slider) -> f32 {
    let max_offset = (track_width - slider.thumb_size).max(0.0);
    let range_span = f32::from(slider.max.saturating_sub(slider.min).max(1));
    let normalized_value = f32::from(current_value.saturating_sub(slider.min)) / range_span;
    max_offset * normalized_value
}

fn slider_fill_width(thumb_offset: f32, slider: Slider) -> f32 {
    thumb_offset + slider.thumb_size
}

fn slider_track_area(layout: daiko::layout::Layout) -> Rect {
    Rect::from_origin_and_size(layout.position_absolute.to_point(), layout.size.into())
}

fn is_pointer_inside_track(pointer_position: Pos2, track_area: Rect) -> bool {
    pointer_position.x >= track_area.min.x
        && pointer_position.x <= track_area.max.x
        && pointer_position.y >= track_area.min.y
        && pointer_position.y <= track_area.max.y
}

fn slider_value_from_track_position(
    pointer_position: Pos2,
    track_area: Option<Rect>,
    track_width: f32,
    slider: Slider,
) -> u8 {
    let Some(track_area) = track_area else {
        return slider.clamped_default_value();
    };

    let max_offset = (track_width - slider.thumb_size).max(0.0);
    let thumb_centered_offset =
        (pointer_position.x - track_area.min.x - slider.thumb_size * 0.5).clamp(0.0, max_offset);
    let normalized_value = if max_offset <= f32::EPSILON {
        0.0
    } else {
        thumb_centered_offset / max_offset
    };
    let range_span = f32::from(slider.max.saturating_sub(slider.min));

    slider.clamp_value((f32::from(slider.min) + normalized_value * range_span).round() as i16)
}

fn normalized_bounds(min: u8, max: u8) -> (u8, u8) {
    if min <= max { (min, max) } else { (max, min) }
}

fn slider_track_width(slider: Slider, track_area: Option<Rect>) -> f32 {
    track_area
        .map(|area| area.width())
        .or(slider.track_width)
        .unwrap_or(DEFAULT_TRACK_WIDTH_FALLBACK)
        .max(slider.thumb_size)
}

fn slider_smooth_follow_config() -> SmoothFollowConfig {
    SmoothFollowConfig::new(
        Duration::from_millis(MIN_SMOOTH_FOLLOW_DURATION_MS),
        SMOOTH_FOLLOW_CURRENT_VELOCITY_WEIGHTING,
        SMOOTH_FOLLOW_STOP_DECELERATION_WEIGHTING,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use daiko::layout::ItemSize;
    use daiko::style::Style;
    use daiko::testing::TestRunner;
    use daiko::{App, AppContext};

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
                    .with_content(
                        Element::new().with_style(Style::new().with_fixed_size(260.0, 24.0)),
                    )
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
}
