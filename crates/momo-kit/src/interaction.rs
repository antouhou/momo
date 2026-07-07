use daiko::{
    component::ComponentContext,
    layout::Layout,
    navigation::{FocusKey, FocusOrigin},
};

pub struct ButtonBehavior<'context, 'app> {
    ctx: &'context mut ComponentContext<'app>,
    focus_key: Option<FocusKey>,
    preferred_focus: bool,
    enabled: bool,
    requested_focus: Option<FocusOrigin>,
}

impl<'context, 'app> ButtonBehavior<'context, 'app> {
    pub fn new(ctx: &'context mut ComponentContext<'app>) -> Self {
        Self {
            ctx,
            focus_key: None,
            preferred_focus: false,
            enabled: true,
            requested_focus: None,
        }
    }

    pub fn with_focus_key(mut self, focus_key: FocusKey) -> Self {
        self.focus_key = Some(focus_key);
        self
    }

    pub fn with_preferred_focus(mut self, preferred_focus: bool) -> Self {
        self.preferred_focus = preferred_focus;
        self
    }

    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn with_requested_focus(mut self, requested_focus: Option<FocusOrigin>) -> Self {
        self.requested_focus = requested_focus;
        self
    }

    pub fn apply(self) -> ButtonInteractionState {
        let mut pointer = self.ctx.pointer();
        let focusable = self.ctx.focusable();
        let layout = self.ctx.layout();

        focusable.set_navigation_enabled(self.enabled);
        focusable.set_preferred_focus(self.preferred_focus);
        if let Some(focus_key) = self.focus_key {
            focusable.set_focus_key(focus_key);
        }
        if self.enabled
            && let Some(focus_origin) = self.requested_focus
        {
            focusable.request_focus(focus_origin);
        }

        let pointer_activated = self.enabled && pointer.just_pressed();
        let focus_activated = self.enabled && focusable.just_activated();

        if pointer_activated {
            focusable.request_focus(FocusOrigin::Pointer);
        }

        ButtonInteractionState {
            is_hovering: self.enabled && pointer.is_hovering(),
            is_focus_visible: self.enabled && focusable.is_focus_visible(),
            is_focused: self.enabled && focusable.is_focused(),
            is_pressed: self.enabled && pointer.is_pressed(),
            just_activated: pointer_activated || focus_activated,
            layout,
        }
    }
}

pub struct ButtonInteractionState {
    pub is_hovering: bool,
    pub is_focus_visible: bool,
    pub is_focused: bool,
    pub is_pressed: bool,
    pub just_activated: bool,
    pub layout: Option<Layout>,
}
