mod style;

use self::style::{
    DEFAULT_VIEW_TRANSITION_SLIDE_DISTANCE, VIEW_TRANSITION_DURATION_MS,
    view_transition_slot_style, view_transition_style,
};
use daiko::animation::AnimationParameters;
use daiko::animation::easing::EasingFunction;
use daiko::component::{Child, Component, ComponentContext, IntoChild};
use daiko::{Element, Id, Vec2};
use std::hash::Hash;
use std::time::Duration;

pub struct ViewTransition {
    current_view: Child,
    transition_key: Option<Id>,
    direction: ViewTransitionDirection,
    slide_distance: f32,
}

impl ViewTransition {
    pub fn new(current_view: impl IntoChild) -> Self {
        Self {
            current_view: current_view.into_child(),
            transition_key: None,
            direction: ViewTransitionDirection::Forward,
            slide_distance: DEFAULT_VIEW_TRANSITION_SLIDE_DISTANCE,
        }
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
        let mut snapshot = transition_state.read().clone();
        let layout_size = ctx.layout().map(|layout| layout.size());
        let key = self.transition_key.unwrap_or(Id::NULL);
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

        if key_changed {
            snapshot.viewport_size = layout_size
                .or(snapshot.viewport_size)
                .or(Some(Vec2::new(self.slide_distance, 0.0)));
            snapshot.previous_view = snapshot
                .current_view
                .clone()
                .or_else(|| Some(self.current_view.clone()));
            snapshot.current_key = Some(key);
            snapshot.current_view = Some(self.current_view.clone());
            animation.restart_reset();
            *transition_state.write_silent() = snapshot.clone();
        }

        let progress = if snapshot.previous_view.is_some() || animation.is_running() {
            animation.progress()
        } else {
            1.0
        };

        if snapshot.previous_view.is_some()
            && !animation.is_running()
            && animation.progress_linear() >= 1.0
        {
            snapshot.previous_view = None;
            snapshot.current_view = Some(self.current_view.clone());
            *transition_state.write_silent() = snapshot.clone();
        } else if snapshot.previous_view.is_none() && !animation.is_running() {
            // Keep the next outgoing child fresh without requesting another render.
            snapshot.current_view = Some(self.current_view.clone());
            if let Some(layout_size) = layout_size
                && snapshot.viewport_size != Some(layout_size)
            {
                snapshot.viewport_size = Some(layout_size);
            }
            *transition_state.write_silent() = snapshot.clone();
        }

        let is_transitioning = snapshot.previous_view.is_some();
        let mut el = Element::new().with_style(view_transition_style(
            self.slide_distance,
            is_transitioning.then_some(snapshot.viewport_size).flatten(),
        ));

        let current_phase = if progress < 1.0 {
            ViewTransitionPhase::Incoming
        } else {
            ViewTransitionPhase::Stable
        };

        el.add_content(
            Element::new()
                .with_style(view_transition_slot_style(
                    current_phase,
                    progress,
                    self.direction,
                    self.slide_distance,
                ))
                .with_content(self.current_view.clone()),
        );

        if let Some(previous_view) = snapshot.previous_view {
            el.add_content(
                Element::new()
                    .with_style(view_transition_slot_style(
                        ViewTransitionPhase::Outgoing,
                        progress,
                        self.direction,
                        self.slide_distance,
                    ))
                    .with_content(previous_view),
            );
        }

        el
    }
}

#[derive(Clone, Default)]
struct ViewTransitionState {
    current_key: Option<Id>,
    current_view: Option<Child>,
    previous_view: Option<Child>,
    viewport_size: Option<Vec2>,
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
