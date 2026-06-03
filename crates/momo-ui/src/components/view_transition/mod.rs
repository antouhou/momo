mod state;
mod style;

use self::state::{
    ViewTransitionMeasurements, ViewTransitionSlotMotion, ViewTransitionState,
    incoming_view_transition_slot_motion, outgoing_view_transition_slot_motion,
    outgoing_view_transition_slot_target_offset, stable_view_transition_slot_motion,
    view_transition_slot_motion_offset,
};
use self::style::{
    DEFAULT_VIEW_TRANSITION_SLIDE_DISTANCE, VIEW_TRANSITION_DURATION_MS,
    view_transition_slot_style, view_transition_style,
};
use daiko::animation::AnimationParameters;
use daiko::animation::easing::EasingFunction;
use daiko::channel::Channel;
use daiko::component::{Child, Component, ComponentContext, IntoChild};
use daiko::state_management::StateHandle;
use daiko::{Element, Id, Vec2};
use std::hash::Hash;
use std::time::Duration;

pub struct ViewTransition {
    id: Id,
    from_view: Option<Child>,
    to_view: Child,
    transition_key: Option<Id>,
    direction: ViewTransitionDirection,
    slide_distance: f32,
}

impl ViewTransition {
    pub fn new() -> Self {
        Self {
            id: Id::new("view_transition"),
            from_view: None,
            to_view: Element::new().into_child(),
            transition_key: None,
            direction: ViewTransitionDirection::Forward,
            slide_distance: DEFAULT_VIEW_TRANSITION_SLIDE_DISTANCE,
        }
    }

    pub fn from(mut self, from_view: impl IntoChild) -> Self {
        self.from_view = Some(from_view.into_child());
        self
    }

    pub fn to(mut self, to_view: impl IntoChild) -> Self {
        self.to_view = to_view.into_child();
        self
    }

    pub fn with_id(mut self, id: impl Hash) -> Self {
        self.id = Id::new(id);
        self
    }

    pub fn with_transition_key(mut self, transition_key: impl Hash) -> Self {
        self.transition_key = Some(Id::new(transition_key));
        self
    }

    pub fn with_direction(mut self, direction: ViewTransitionDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn with_slide_distance(mut self, slide_distance: f32) -> Self {
        self.slide_distance = slide_distance;
        self
    }

    pub(crate) fn use_controller(
        ctx: &mut ComponentContext,
        id: impl Hash,
    ) -> ViewTransitionController {
        let id = Id::new(id);

        ViewTransitionController {
            status: ctx
                .use_shared_state(view_transition_status_id(id), ViewTransitionStatus::default),
            events: ctx.use_channel_with_id(view_transition_events_id(id)),
        }
    }
}

impl Component for ViewTransition {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let transition_state = ctx.use_local_state(ViewTransitionState::default);
        let measurements = ctx.use_shared_state(view_transition_measurements_id(self.id), || {
            ViewTransitionMeasurements::default()
        });
        let measurement = *measurements.read();
        let layout_size = ctx.layout().map(|layout| layout.size());
        let key = self.transition_key.unwrap_or(Id::NULL);
        let mut completed_event = None;
        let animation = ctx.animation(
            AnimationParameters::default()
                .with_duration(Duration::from_millis(VIEW_TRANSITION_DURATION_MS))
                .with_easing(EasingFunction::EaseInOut),
        );
        let mut next_state = transition_state.read().clone();

        if next_state.current_key.is_none() {
            next_state.current_key = Some(key);
            *transition_state.write_silent() = next_state.clone();
        }

        let key_changed = self.transition_key.is_some() && next_state.current_key != Some(key);
        let mut started_transition_this_run = false;

        if key_changed {
            let progress_before_key_change =
                if next_state.previous_key.is_some() && next_state.target_size.is_some() {
                    animation.progress()
                } else if next_state.previous_key.is_some() {
                    0.0
                } else {
                    1.0
                };

            let mut measurement_update = measurement;
            measurement_update.incoming_key = None;
            measurement_update.incoming_size = None;
            *measurements.write_silent() = measurement_update;

            let from_size = if next_state.previous_key.is_some() {
                current_viewport_size(&next_state, progress_before_key_change)
            } else {
                layout_size.or(measurement.stable_size)
            }
            .or(next_state.viewport_size)
            .unwrap_or(Vec2::new(self.slide_distance, 0.0));

            let incoming_motion =
                incoming_view_transition_slot_motion(self.direction, self.slide_distance);
            let outgoing_motion =
                outgoing_view_transition_slot_motion(self.direction, self.slide_distance);
            let mut current_motion = incoming_motion;
            let mut previous_motion = outgoing_motion;
            let mut previous_key = next_state.current_key;

            if next_state.previous_key.is_some() {
                let current_offset = view_transition_slot_motion_offset(
                    next_state.current_motion,
                    progress_before_key_change,
                );
                let previous_offset = view_transition_slot_motion_offset(
                    next_state.previous_motion,
                    progress_before_key_change,
                );
                let outgoing_target_offset = outgoing_view_transition_slot_target_offset(
                    self.direction,
                    self.slide_distance,
                );

                if next_state.previous_key == Some(key) {
                    previous_key = next_state.current_key;
                    current_motion = ViewTransitionSlotMotion::new(previous_offset, 0.0);
                    previous_motion =
                        ViewTransitionSlotMotion::new(current_offset, outgoing_target_offset);
                } else {
                    previous_motion =
                        ViewTransitionSlotMotion::new(current_offset, outgoing_target_offset);
                }
            }

            next_state.viewport_size = Some(from_size);
            next_state.from_size = Some(from_size);
            next_state.target_size = None;
            next_state.previous_key = previous_key;
            next_state.current_key = Some(key);
            next_state.current_motion = current_motion;
            next_state.previous_motion = previous_motion;
            animation.reset();
            started_transition_this_run = true;
            *transition_state.write_silent() = next_state.clone();
        }

        if next_state.previous_key.is_some()
            && !started_transition_this_run
            && measurement.incoming_key == Some(key)
            && let Some(target_size) = measurement.incoming_size
            && (next_state.target_size.is_none()
                || (next_state.target_size != Some(target_size)
                    && animation.progress_linear() == 0.0))
        {
            next_state.target_size = Some(target_size);
            animation.restart_reset();
            *transition_state.write_silent() = next_state.clone();
        }

        let progress = if next_state.previous_key.is_some() && next_state.target_size.is_some() {
            animation.progress()
        } else if next_state.previous_key.is_some() {
            0.0
        } else {
            1.0
        };

        if next_state.previous_key.is_some()
            && next_state.target_size.is_some()
            && !animation.is_running()
            && animation.progress_linear() >= 1.0
        {
            completed_event = next_state
                .previous_key
                .map(|outgoing_key| ViewTransitionEvent::Completed { outgoing_key });
            next_state.viewport_size = next_state.target_size;
            next_state.from_size = None;
            next_state.target_size = None;
            next_state.previous_key = None;
            next_state.current_motion = stable_view_transition_slot_motion();
            next_state.previous_motion = stable_view_transition_slot_motion();
            *transition_state.write_silent() = next_state.clone();
        } else if next_state.previous_key.is_none() && !animation.is_running() {
            // Keep the stable viewport size fresh without requesting another render.
            let measured_size = layout_size.or(measurement.stable_size);
            if let Some(measured_size) = measured_size
                && next_state.viewport_size != Some(measured_size)
            {
                next_state.viewport_size = Some(measured_size);
            }
            *transition_state.write_silent() = next_state.clone();
        }

        let is_transitioning = next_state.previous_key.is_some();
        let viewport_size = if is_transitioning {
            match (next_state.from_size, next_state.target_size) {
                (Some(from_size), Some(target_size)) => {
                    Some(lerp_vec2(from_size, target_size, progress))
                }
                _ => next_state.viewport_size,
            }
        } else {
            None
        };
        let current_phase = if next_state.previous_key.is_some() {
            ViewTransitionPhase::Incoming
        } else {
            ViewTransitionPhase::Stable
        };
        let current_motion = if next_state.previous_key.is_some() {
            next_state.current_motion
        } else {
            stable_view_transition_slot_motion()
        };
        let previous_motion = next_state.previous_motion;

        publish_view_transition_status(ctx, self.id, is_transitioning);
        if let Some(event) = completed_event {
            let _ = ctx
                .use_channel_with_id(view_transition_events_id(self.id))
                .send(event);
        }

        let mut el =
            Element::new().with_style(view_transition_style(self.slide_distance, viewport_size));

        el.add_content(ViewTransitionSlot {
            measurements_id: view_transition_measurements_id(self.id),
            report_key: Some(key),
            report_kind: match current_phase {
                ViewTransitionPhase::Stable => ViewTransitionSlotReportKind::Stable,
                ViewTransitionPhase::Incoming => ViewTransitionSlotReportKind::Incoming,
                ViewTransitionPhase::Outgoing => ViewTransitionSlotReportKind::None,
            },
            phase: current_phase,
            progress,
            motion: current_motion,
            slide_distance: self.slide_distance,
            content: self.to_view.clone(),
        });

        if is_transitioning {
            el.add_content(ViewTransitionSlot {
                measurements_id: view_transition_measurements_id(self.id),
                report_key: None,
                report_kind: ViewTransitionSlotReportKind::None,
                phase: ViewTransitionPhase::Outgoing,
                progress,
                motion: previous_motion,
                slide_distance: self.slide_distance,
                content: self
                    .from_view
                    .clone()
                    .unwrap_or_else(|| self.to_view.clone()),
            });
        }

        el
    }
}

#[derive(Clone)]
struct ViewTransitionSlot {
    measurements_id: Id,
    report_key: Option<Id>,
    report_kind: ViewTransitionSlotReportKind,
    phase: ViewTransitionPhase,
    progress: f32,
    motion: ViewTransitionSlotMotion,
    slide_distance: f32,
    content: Child,
}

impl Component for ViewTransitionSlot {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        if let Some(layout) = ctx.layout() {
            let measurements =
                ctx.use_shared_state(self.measurements_id, ViewTransitionMeasurements::default);
            let mut measurement = *measurements.read();
            let size = layout.size();
            let mut changed = false;

            match self.report_kind {
                ViewTransitionSlotReportKind::Stable if measurement.stable_size != Some(size) => {
                    measurement.stable_size = Some(size);
                    changed = true;
                }
                ViewTransitionSlotReportKind::Incoming
                    if measurement.incoming_size != Some(size) =>
                {
                    measurement.incoming_key = self.report_key;
                    measurement.incoming_size = Some(size);
                    changed = true;
                }
                _ => {}
            }

            if changed {
                *measurements.write() = measurement;
            }
        }

        Element::new()
            .with_style(view_transition_slot_style(
                self.phase,
                self.progress,
                self.motion,
                self.slide_distance,
            ))
            .with_content(self.content.clone())
    }
}

#[derive(Clone, Copy)]
enum ViewTransitionSlotReportKind {
    None,
    Stable,
    Incoming,
}

#[derive(Clone, Copy, Default)]
struct ViewTransitionStatus {
    is_transitioning: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ViewTransitionEvent {
    Completed { outgoing_key: Id },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ViewTransitionDirection {
    Forward,
    Backward,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ViewTransitionPhase {
    Stable,
    Incoming,
    Outgoing,
}

#[derive(Clone)]
pub(crate) struct ViewTransitionController {
    status: StateHandle<ViewTransitionStatus>,
    events: Channel<ViewTransitionEvent>,
}

impl ViewTransitionController {
    pub(crate) fn is_transitioning(&self) -> bool {
        self.status.read().is_transitioning
    }

    pub(crate) fn events(&self) -> impl Iterator<Item = ViewTransitionEvent> {
        self.events.iter()
    }
}

fn publish_view_transition_status(ctx: &mut ComponentContext, id: Id, is_transitioning: bool) {
    let status = ctx.use_shared_state(view_transition_status_id(id), ViewTransitionStatus::default);
    let previous_status = *status.read();

    if previous_status.is_transitioning != is_transitioning {
        *status.write() = ViewTransitionStatus { is_transitioning };
    }
}

fn view_transition_status_id(id: Id) -> Id {
    Id::new(("view_transition_status", id))
}

fn view_transition_events_id(id: Id) -> Id {
    Id::new(("view_transition_events", id))
}

fn view_transition_measurements_id(id: Id) -> Id {
    Id::new(("view_transition_measurements", id))
}

fn lerp_vec2(from: Vec2, to: Vec2, progress: f32) -> Vec2 {
    from + (to - from) * progress
}

fn current_viewport_size(state: &ViewTransitionState, progress: f32) -> Option<Vec2> {
    match (state.from_size, state.target_size) {
        (Some(from_size), Some(target_size)) => Some(lerp_vec2(from_size, target_size, progress)),
        _ => state.viewport_size,
    }
}
