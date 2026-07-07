mod state;
mod style;

use crate::components::login_screen::clock::{
    state::{ClockLocalState, clock_text, spawn_clock_thread},
    style::{clock_style, clock_text_style},
};
use daiko::{
    Element,
    component::{Component, ComponentContext},
    widgets::text::Text,
};

#[derive(Clone, Copy)]
pub(super) struct Clock {
    live: bool,
}

impl Clock {
    pub(super) fn new(live: bool) -> Self {
        Self { live }
    }
}

impl Component for Clock {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let local_state = ctx.use_local_state(ClockLocalState::default);
        let clock_text = clock_text(ctx);

        if self.live && !local_state.read().worker_started {
            local_state.write_silent().worker_started = true;
            spawn_clock_thread(clock_text.clone());
        }

        Element::new()
            .with_tag("greeter-clock")
            .with_style(clock_style())
            .with_content(Text::new(clock_text.read().clone()).with_style(clock_text_style()))
    }
}
