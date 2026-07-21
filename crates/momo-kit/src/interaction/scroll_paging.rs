use daiko::{Id, Vec2, component::ComponentContext};
use std::time::{Duration, Instant};

const DEFAULT_ACTIVATION_THRESHOLD: f32 = 8.0;
const DEFAULT_REARM_DURATION: Duration = Duration::from_millis(220);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PageScrollDirection {
    Previous,
    Next,
}

impl PageScrollDirection {
    pub const fn page_delta(self) -> isize {
        match self {
            Self::Previous => -1,
            Self::Next => 1,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ScrollPagingAxis {
    Horizontal,
    Vertical,
    #[default]
    VerticalWithHorizontalFallback,
}

impl ScrollPagingAxis {
    fn select_delta(self, scroll_delta: Vec2) -> f32 {
        match self {
            Self::Horizontal => scroll_delta.x,
            Self::Vertical => scroll_delta.y,
            Self::VerticalWithHorizontalFallback if scroll_delta.y.abs() > f32::EPSILON => {
                scroll_delta.y
            }
            Self::VerticalWithHorizontalFallback => scroll_delta.x,
        }
    }
}

#[derive(Default)]
struct ScrollPagingState {
    accumulated_delta: f32,
    locked_until: Option<Instant>,
}

impl ScrollPagingState {
    fn apply_delta(
        &mut self,
        scroll_delta: f32,
        now: Instant,
        activation_threshold: f32,
        rearm_duration: Duration,
    ) -> Option<PageScrollDirection> {
        if scroll_delta.abs() <= f32::EPSILON {
            return None;
        }

        if self
            .locked_until
            .is_some_and(|locked_until| now < locked_until)
        {
            self.accumulated_delta = 0.0;
            return None;
        }

        self.locked_until = None;
        self.accumulated_delta += scroll_delta;
        let direction = if self.accumulated_delta <= -activation_threshold {
            Some(PageScrollDirection::Previous)
        } else if self.accumulated_delta >= activation_threshold {
            Some(PageScrollDirection::Next)
        } else {
            None
        };

        if direction.is_some() {
            self.accumulated_delta = 0.0;
            self.locked_until = now.checked_add(rearm_duration);
        }

        direction
    }
}

/// Converts exclusive scroll input into discrete previous/next page interactions.
///
/// By default, paging activates after accumulating `8.0` points of scroll input, rearms after
/// 220 milliseconds, and uses vertical input with horizontal input as a fallback.
pub struct ScrollPagingBehavior<'context, 'app> {
    context: &'context mut ComponentContext<'app>,
    scroll_state_id: Id,
    activation_threshold: f32,
    rearm_duration: Duration,
    scroll_axis: ScrollPagingAxis,
}

impl<'context, 'app> ScrollPagingBehavior<'context, 'app> {
    pub fn new(context: &'context mut ComponentContext<'app>, scroll_state_id: Id) -> Self {
        Self {
            context,
            scroll_state_id,
            activation_threshold: DEFAULT_ACTIVATION_THRESHOLD,
            rearm_duration: DEFAULT_REARM_DURATION,
            scroll_axis: ScrollPagingAxis::default(),
        }
    }

    /// Sets the accumulated scroll distance required to activate one page interaction.
    pub fn with_activation_threshold(mut self, activation_threshold: f32) -> Self {
        self.activation_threshold = activation_threshold.abs().max(f32::EPSILON);
        self
    }

    /// Sets how long additional scroll input is ignored after a page interaction.
    pub fn with_rearm_duration(mut self, rearm_duration: Duration) -> Self {
        self.rearm_duration = rearm_duration;
        self
    }

    /// Selects which axis supplies scroll input to the paging accumulator.
    pub fn with_scroll_axis(mut self, scroll_axis: ScrollPagingAxis) -> Self {
        self.scroll_axis = scroll_axis;
        self
    }

    pub fn apply(self) -> Option<PageScrollDirection> {
        let scroll_state = self
            .context
            .use_local_state_with_id(self.scroll_state_id, ScrollPagingState::default);
        let scroll_delta = self.context.consume_scroll()?;
        let axis_delta = self.scroll_axis.select_delta(scroll_delta);

        scroll_state.write_silent().apply_delta(
            axis_delta,
            Instant::now(),
            self.activation_threshold,
            self.rearm_duration,
        )
    }
}

#[cfg(test)]
#[path = "scroll_paging_tests.rs"]
mod tests;
