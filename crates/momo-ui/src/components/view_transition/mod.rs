mod style;

use self::style::{
    DEFAULT_VIEW_TRANSITION_SLIDE_DISTANCE, VIEW_TRANSITION_DURATION_MS, ViewTransitionSlotMotion,
    incoming_view_transition_slot_motion, outgoing_view_transition_slot_motion,
    outgoing_view_transition_slot_target_offset, stable_view_transition_slot_motion,
    view_transition_slot_motion_offset, view_transition_slot_style, view_transition_style,
};
use daiko::animation::AnimationParameters;
use daiko::animation::easing::EasingFunction;
use daiko::component::{Child, Component, ComponentContext, IntoChild};
use daiko::{Element, Id, Vec2};
use std::hash::Hash;
use std::time::Duration;

pub struct ViewTransition {
    id: Id,
    current_view: Child,
    transition_key: Option<Id>,
    direction: ViewTransitionDirection,
    slide_distance: f32,
}

impl ViewTransition {
    pub fn new(current_view: impl IntoChild) -> Self {
        Self {
            id: Id::new("view_transition"),
            current_view: current_view.into_child(),
            transition_key: None,
            direction: ViewTransitionDirection::Forward,
            slide_distance: DEFAULT_VIEW_TRANSITION_SLIDE_DISTANCE,
        }
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
}

impl Component for ViewTransition {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let transition_state = ctx.use_local_state(ViewTransitionState::default);
        let measurements = ctx.use_shared_state(view_transition_measurements_id(self.id), || {
            ViewTransitionMeasurements::default()
        });
        let mut snapshot = transition_state.read().clone();
        let measurement_snapshot = *measurements.read();
        let layout_size = ctx.layout().map(|layout| layout.size());
        let key = self.transition_key.unwrap_or(Id::NULL);
        let mut completed_outgoing_key = None;
        let animation = ctx.animation(
            AnimationParameters::default()
                .with_duration(Duration::from_millis(VIEW_TRANSITION_DURATION_MS))
                .with_easing(EasingFunction::EaseInOut),
        );

        if snapshot.current_key.is_none() {
            snapshot.current_key = Some(key);
            snapshot.current_view = Some(self.current_view.clone());
            *transition_state.write_silent() = snapshot.clone();
        }

        let key_changed = self.transition_key.is_some() && snapshot.current_key != Some(key);
        let mut started_transition_this_run = false;

        if key_changed {
            let progress_before_key_change =
                if snapshot.previous_view.is_some() && snapshot.target_size.is_some() {
                    animation.progress()
                } else if snapshot.previous_view.is_some() {
                    0.0
                } else {
                    1.0
                };

            let mut measurement_update = measurement_snapshot;
            measurement_update.incoming_key = None;
            measurement_update.incoming_size = None;
            *measurements.write_silent() = measurement_update;

            let from_size = if snapshot.previous_view.is_some() {
                current_viewport_size(&snapshot, progress_before_key_change)
            } else {
                layout_size.or(measurement_snapshot.stable_size)
            }
            .or(snapshot.viewport_size)
            .unwrap_or(Vec2::new(self.slide_distance, 0.0));

            let incoming_motion =
                incoming_view_transition_slot_motion(self.direction, self.slide_distance);
            let outgoing_motion =
                outgoing_view_transition_slot_motion(self.direction, self.slide_distance);
            let mut current_motion = incoming_motion;
            let mut previous_motion = outgoing_motion;
            let mut current_view = self.current_view.clone();
            let mut previous_key = snapshot.current_key;
            let mut previous_view = snapshot
                .current_view
                .clone()
                .or_else(|| Some(self.current_view.clone()));

            if snapshot.previous_view.is_some() {
                let current_offset = view_transition_slot_motion_offset(
                    snapshot.current_motion,
                    progress_before_key_change,
                );
                let previous_offset = view_transition_slot_motion_offset(
                    snapshot.previous_motion,
                    progress_before_key_change,
                );
                let outgoing_target_offset = outgoing_view_transition_slot_target_offset(
                    self.direction,
                    self.slide_distance,
                );

                if snapshot.previous_key == Some(key) {
                    if let Some(returning_view) = snapshot.previous_view.clone() {
                        current_view = returning_view;
                    }
                    previous_key = snapshot.current_key;
                    previous_view = snapshot.current_view.clone();
                    current_motion = ViewTransitionSlotMotion::new(previous_offset, 0.0);
                    previous_motion =
                        ViewTransitionSlotMotion::new(current_offset, outgoing_target_offset);
                } else {
                    previous_view = snapshot.current_view.clone();
                    previous_motion =
                        ViewTransitionSlotMotion::new(current_offset, outgoing_target_offset);
                }
            }

            snapshot.viewport_size = Some(from_size);
            snapshot.from_size = Some(from_size);
            snapshot.target_size = None;
            snapshot.previous_view = previous_view;
            snapshot.previous_key = previous_key;
            snapshot.current_key = Some(key);
            snapshot.current_view = Some(current_view);
            snapshot.current_motion = current_motion;
            snapshot.previous_motion = previous_motion;
            animation.reset();
            started_transition_this_run = true;
            *transition_state.write_silent() = snapshot.clone();
        }

        if snapshot.previous_view.is_some()
            && !started_transition_this_run
            && measurement_snapshot.incoming_key == Some(key)
            && let Some(target_size) = measurement_snapshot.incoming_size
            && (snapshot.target_size.is_none()
                || (snapshot.target_size != Some(target_size)
                    && animation.progress_linear() == 0.0))
        {
            snapshot.target_size = Some(target_size);
            animation.restart_reset();
            *transition_state.write_silent() = snapshot.clone();
        }

        let progress = if snapshot.previous_view.is_some() && snapshot.target_size.is_some() {
            animation.progress()
        } else if snapshot.previous_view.is_some() {
            0.0
        } else {
            1.0
        };

        if snapshot.previous_view.is_some()
            && snapshot.target_size.is_some()
            && !animation.is_running()
            && animation.progress_linear() >= 1.0
        {
            snapshot.previous_view = None;
            completed_outgoing_key = snapshot.previous_key;
            snapshot.current_view = Some(self.current_view.clone());
            snapshot.viewport_size = snapshot.target_size;
            snapshot.from_size = None;
            snapshot.target_size = None;
            snapshot.previous_key = None;
            snapshot.current_motion = stable_view_transition_slot_motion();
            snapshot.previous_motion = stable_view_transition_slot_motion();
            *transition_state.write_silent() = snapshot.clone();
        } else if snapshot.previous_view.is_none() && !animation.is_running() {
            // Keep the next outgoing child fresh without requesting another render.
            snapshot.current_view = Some(self.current_view.clone());
            let measured_size = layout_size.or(measurement_snapshot.stable_size);
            if let Some(measured_size) = measured_size
                && snapshot.viewport_size != Some(measured_size)
            {
                snapshot.viewport_size = Some(measured_size);
            }
            *transition_state.write_silent() = snapshot.clone();
        }

        let is_transitioning = snapshot.previous_view.is_some();
        publish_view_transition_status(ctx, self.id, is_transitioning, completed_outgoing_key);

        let viewport_size = if is_transitioning {
            match (snapshot.from_size, snapshot.target_size) {
                (Some(from_size), Some(target_size)) => {
                    Some(lerp_vec2(from_size, target_size, progress))
                }
                _ => snapshot.viewport_size,
            }
        } else {
            None
        };

        let mut el =
            Element::new().with_style(view_transition_style(self.slide_distance, viewport_size));

        let current_phase = if snapshot.previous_view.is_some() {
            ViewTransitionPhase::Incoming
        } else {
            ViewTransitionPhase::Stable
        };
        let current_motion = if snapshot.previous_view.is_some() {
            snapshot.current_motion
        } else {
            stable_view_transition_slot_motion()
        };

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
            content: self.current_view.clone(),
        });

        if let Some(previous_view) = snapshot.previous_view {
            el.add_content(ViewTransitionSlot {
                measurements_id: view_transition_measurements_id(self.id),
                report_key: None,
                report_kind: ViewTransitionSlotReportKind::None,
                phase: ViewTransitionPhase::Outgoing,
                progress,
                motion: snapshot.previous_motion,
                slide_distance: self.slide_distance,
                content: previous_view,
            });
        }

        el
    }
}

#[derive(Clone, Default)]
struct ViewTransitionState {
    current_key: Option<Id>,
    previous_key: Option<Id>,
    current_view: Option<Child>,
    previous_view: Option<Child>,
    viewport_size: Option<Vec2>,
    from_size: Option<Vec2>,
    target_size: Option<Vec2>,
    current_motion: ViewTransitionSlotMotion,
    previous_motion: ViewTransitionSlotMotion,
}

#[derive(Clone, Copy, Default)]
struct ViewTransitionMeasurements {
    stable_size: Option<Vec2>,
    incoming_key: Option<Id>,
    incoming_size: Option<Vec2>,
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
            let mut snapshot = *measurements.read();
            let size = layout.size();
            let mut changed = false;

            match self.report_kind {
                ViewTransitionSlotReportKind::Stable if snapshot.stable_size != Some(size) => {
                    snapshot.stable_size = Some(size);
                    changed = true;
                }
                ViewTransitionSlotReportKind::Incoming if snapshot.incoming_size != Some(size) => {
                    snapshot.incoming_key = self.report_key;
                    snapshot.incoming_size = Some(size);
                    changed = true;
                }
                _ => {}
            }

            if changed {
                *measurements.write() = snapshot;
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
pub(crate) struct ViewTransitionStatus {
    pub(crate) is_transitioning: bool,
    pub(crate) completed_outgoing_key: Option<Id>,
    pub(crate) completion_serial: u64,
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

pub(crate) fn view_transition_status(
    ctx: &mut ComponentContext,
    id: impl Hash,
) -> ViewTransitionStatus {
    *ctx.use_shared_state(
        view_transition_status_id(Id::new(id)),
        ViewTransitionStatus::default,
    )
    .read()
}

fn publish_view_transition_status(
    ctx: &mut ComponentContext,
    id: Id,
    is_transitioning: bool,
    completed_outgoing_key: Option<Id>,
) {
    let status = ctx.use_shared_state(view_transition_status_id(id), ViewTransitionStatus::default);
    let snapshot = *status.read();
    let completed = completed_outgoing_key.is_some();

    if snapshot.is_transitioning != is_transitioning
        || completed
        || snapshot.completed_outgoing_key.is_some()
    {
        *status.write() = ViewTransitionStatus {
            is_transitioning,
            completed_outgoing_key,
            completion_serial: if completed {
                snapshot.completion_serial + 1
            } else {
                snapshot.completion_serial
            },
        };
    }
}

fn view_transition_status_id(id: Id) -> Id {
    Id::new(("view_transition_status", id))
}

fn view_transition_measurements_id(id: Id) -> Id {
    Id::new(("view_transition_measurements", id))
}

fn lerp_vec2(from: Vec2, to: Vec2, progress: f32) -> Vec2 {
    from + (to - from) * progress
}

fn current_viewport_size(snapshot: &ViewTransitionState, progress: f32) -> Option<Vec2> {
    match (snapshot.from_size, snapshot.target_size) {
        (Some(from_size), Some(target_size)) => Some(lerp_vec2(from_size, target_size, progress)),
        _ => snapshot.viewport_size,
    }
}
