mod style;

use crate::components::login_screen::action_button::style::{
    action_button_style, action_text_style,
};
use daiko::Element;
use daiko::channel::Channel;
use daiko::component::{Component, ComponentContext};
use daiko::navigation::FocusOrigin;
use daiko::widgets::text::Text;

pub(super) struct ActionButton {
    tag: &'static str,
    label: &'static str,
    is_primary: bool,
    is_preferred_focus: bool,
    activation_channel: Channel<()>,
}

impl ActionButton {
    pub(super) fn new(
        ctx: &mut ComponentContext,
        tag: &'static str,
        label: &'static str,
        is_primary: bool,
        is_preferred_focus: bool,
    ) -> Self {
        Self {
            tag,
            label,
            is_primary,
            is_preferred_focus,
            activation_channel: ctx.create_channel(),
        }
    }

    pub(super) fn activated(&self) -> bool {
        self.activation_channel.iter().next().is_some()
    }
}

impl Component for ActionButton {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let mut pointer = ctx.pointer();
        let focusable = ctx.focusable();
        focusable.set_preferred_focus(self.is_preferred_focus);

        if pointer.just_pressed() {
            focusable.request_focus(FocusOrigin::Pointer);
        }

        if pointer.just_pressed() || focusable.just_activated() {
            let _ = self.activation_channel.send(());
        }

        let is_highlighted = pointer.is_hovering() || focusable.is_focus_visible();

        Element::new()
            .with_tag(self.tag)
            .with_style(action_button_style(ctx, self.is_primary, is_highlighted))
            .with_content(Text::new(self.label).with_style(action_text_style()))
    }
}
