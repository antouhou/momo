use daiko::{Id, Vec2, component::ComponentContext};
use std::time::{Duration, Instant};

const PAGE_SCROLL_THRESHOLD: f32 = 8.0;
const PAGE_SCROLL_REARM_DURATION: Duration = Duration::from_millis(220);

#[derive(Default)]
struct PageScrollState {
    accumulated_delta: f32,
    locked_until: Option<Instant>,
}

pub(super) fn scroll_page_delta(ctx: &mut ComponentContext, scroll_state_id: Id) -> Option<isize> {
    let scroll_state = ctx.use_local_state_with_id(scroll_state_id, PageScrollState::default);
    let scroll_delta = ctx.consume_scroll()?;

    let scroll_axis_delta = primary_scroll_axis_delta(scroll_delta);
    if scroll_axis_delta.abs() <= f32::EPSILON {
        return None;
    }

    let now = Instant::now();
    let mut scroll_state = scroll_state.write_silent();
    if scroll_state
        .locked_until
        .is_some_and(|locked_until| now < locked_until)
    {
        scroll_state.accumulated_delta = 0.0;
        return None;
    }

    scroll_state.locked_until = None;
    scroll_state.accumulated_delta += scroll_axis_delta;
    let page_delta = page_delta_for_scroll(scroll_state.accumulated_delta);
    if page_delta.is_some() {
        scroll_state.accumulated_delta = 0.0;
        scroll_state.locked_until = Some(now + PAGE_SCROLL_REARM_DURATION);
    }
    page_delta
}

fn page_delta_for_scroll(accumulated_delta: f32) -> Option<isize> {
    if accumulated_delta <= -PAGE_SCROLL_THRESHOLD {
        Some(-1)
    } else if accumulated_delta >= PAGE_SCROLL_THRESHOLD {
        Some(1)
    } else {
        None
    }
}

fn primary_scroll_axis_delta(scroll_delta: Vec2) -> f32 {
    if scroll_delta.y.abs() > f32::EPSILON {
        scroll_delta.y
    } else {
        scroll_delta.x
    }
}
