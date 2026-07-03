mod style;

use crate::components::login_screen::profile_tile::style::{
    avatar_style, avatar_text_style, label_text_style, tile_style,
};
use daiko::Element;
use daiko::channel::Channel;
use daiko::component::{Component, ComponentContext};
use daiko::navigation::FocusOrigin;
use daiko::widgets::text::Text;

#[derive(Clone, Copy)]
pub(super) enum AvatarTone {
    Blue,
    Violet,
    Green,
    Neutral,
}

#[derive(Clone, Copy)]
pub(super) enum GlyphScale {
    Standard,
    Large,
}

#[derive(Clone, Copy)]
pub(super) struct ProfileTilePresentation {
    pub(super) tag: &'static str,
    pub(super) label: &'static str,
    pub(super) glyph: &'static str,
    pub(super) avatar_tone: AvatarTone,
    pub(super) glyph_scale: GlyphScale,
    pub(super) is_preferred_focus: bool,
}

pub(super) struct ProfileTile {
    presentation: ProfileTilePresentation,
    activation_channel: Channel<()>,
}

impl ProfileTile {
    pub(super) fn new(ctx: &mut ComponentContext, presentation: ProfileTilePresentation) -> Self {
        Self {
            presentation,
            activation_channel: ctx.create_channel(),
        }
    }

    pub(super) fn activated(&self) -> bool {
        self.activation_channel.iter().next().is_some()
    }
}

impl Component for ProfileTile {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let mut pointer = ctx.pointer();
        let focusable = ctx.focusable();

        focusable.set_preferred_focus(self.presentation.is_preferred_focus);

        if pointer.just_pressed() {
            focusable.request_focus(FocusOrigin::Pointer);
        }

        if pointer.just_pressed() || focusable.just_activated() {
            let _ = self.activation_channel.send(());
        }

        let is_highlighted = pointer.is_hovering() || focusable.is_focus_visible();

        Element::new()
            .with_tag(self.presentation.tag)
            .with_style(tile_style(ctx, is_highlighted))
            .with_content(
                Element::new()
                    .with_style(avatar_style(
                        ctx,
                        self.presentation.avatar_tone,
                        is_highlighted,
                    ))
                    .with_content(
                        Text::new(self.presentation.glyph)
                            .with_style(avatar_text_style(self.presentation.glyph_scale)),
                    ),
            )
            .with_content(
                Text::new(self.presentation.label).with_style(label_text_style(is_highlighted)),
            )
    }
}
