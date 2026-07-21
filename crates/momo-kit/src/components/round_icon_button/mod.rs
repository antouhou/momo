mod style;

use self::style::{round_icon_button_foreground_color, round_icon_button_style};
use super::svg_icon;
use crate::interaction::{ButtonBehavior, ButtonEvents};
use daiko::{
    Element,
    component::{Component, ComponentContext},
    navigation::FocusOrigin,
};

pub const ROUND_ICON_BUTTON_SIZE: f32 = 44.0;
pub const ROUND_ICON_BUTTON_BORDER_WIDTH: f32 = 1.0;
pub const ROUND_ICON_BUTTON_ICON_SIZE: usize = 18;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RoundIconButtonVariant {
    #[default]
    Standard,
    Accent,
    Danger,
}

#[derive(Clone)]
pub struct RoundIconButton {
    // TODO: ideally, I'd like for the round button to accept impl IntoChild, but unfortunately
    //  there's no API that would allow that: SVG color is animated. We can use clip path for
    //  that probably, and that'd probably be faster, and we wouldn't even need to know the byetes.
    //  But then again, that requires careful design around paths. Right now they're
    //  just vecs, and ideally they should be refs, kinda like we have with StrignOrRef. You get
    //  the point, it is needs some careful design and some time.
    svg: &'static [u8],
    events: ButtonEvents,
    tag: Option<&'static str>,
    variant: RoundIconButtonVariant,
    enabled: bool,
    preferred_focus: bool,
    requested_focus: Option<FocusOrigin>,
    icon_size: usize,
}

impl RoundIconButton {
    pub fn new(context: &mut ComponentContext, svg: &'static [u8]) -> Self {
        Self {
            svg,
            events: ButtonEvents::new(context),
            tag: None,
            variant: RoundIconButtonVariant::Standard,
            enabled: true,
            preferred_focus: false,
            requested_focus: None,
            icon_size: ROUND_ICON_BUTTON_ICON_SIZE,
        }
    }

    pub fn activated(&self) -> bool {
        self.events.activated()
    }

    pub fn has_been_focused(&mut self) -> bool {
        self.events.has_been_focused()
    }

    pub fn with_tag(mut self, tag: &'static str) -> Self {
        self.tag = Some(tag);
        self
    }

    pub fn with_variant(mut self, variant: RoundIconButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn with_preferred_focus(mut self, preferred_focus: bool) -> Self {
        self.preferred_focus = preferred_focus;
        self
    }

    pub fn with_requested_focus(mut self, requested_focus: Option<FocusOrigin>) -> Self {
        self.requested_focus = requested_focus;
        self
    }

    pub fn with_icon_size(mut self, icon_size: usize) -> Self {
        self.icon_size = icon_size;
        self
    }
}

impl Component for RoundIconButton {
    fn to_element(&self, context: &mut ComponentContext) -> Element {
        let interaction = ButtonBehavior::new(context)
            .with_enabled(self.enabled)
            .with_preferred_focus(self.preferred_focus)
            .with_requested_focus(self.requested_focus)
            .without_layout_tracking()
            .apply();

        self.events.publish(&interaction);

        let mut element = Element::new()
            .with_style(round_icon_button_style(context, &interaction, self.variant))
            .with_content(svg_icon(
                self.svg,
                self.icon_size,
                round_icon_button_foreground_color(&interaction, self.variant),
            ));
        if let Some(tag) = self.tag {
            element.set_tag(tag);
        }
        element
    }
}
