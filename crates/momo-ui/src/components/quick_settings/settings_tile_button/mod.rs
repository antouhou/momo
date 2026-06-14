mod style;

use self::style::{
    settings_tile_button_style, settings_tile_content_style, settings_tile_icon_style,
    settings_tile_text_column_style, tile_title_style,
};
use super::common::{QuickSettingsControlState, QuickSettingsGlyph, glyph_element};
use super::style::{SETTINGS_ICON_FRAME_SIZE, SETTINGS_ICON_SIZE, settings_tile_icon_color};
use daiko::Element;
use daiko::channel::Channel;
use daiko::component::{Component, ComponentContext};
use daiko::navigation::{FocusKey, FocusOrigin};
use daiko::widgets::text::Text;

#[derive(Clone, Copy)]
pub(super) struct SettingsTileSpec {
    pub(super) tag: &'static str,
    pub(super) focus_key_id: &'static str,
    pub(super) label: &'static str,
    pub(super) glyph: QuickSettingsGlyph,
    pub(super) is_active: bool,
    pub(super) is_preferred_focus: bool,
}

pub(super) struct SettingsTileButton {
    pub(super) spec: SettingsTileSpec,
    is_active: bool,
    should_request_focus: bool,
    activation_channel: Option<Channel<()>>,
}

impl SettingsTileButton {
    pub(super) fn new(ctx: &mut ComponentContext, spec: SettingsTileSpec, is_active: bool) -> Self {
        Self {
            spec,
            is_active,
            should_request_focus: false,
            activation_channel: Some(ctx.create_channel()),
        }
    }

    pub fn request_focus(&mut self) {
        self.should_request_focus = true;
    }

    pub fn activated(&self) -> bool {
        match &self.activation_channel {
            Some(channel) => channel.iter().count() > 0,
            None => false,
        }
    }
}

impl Component for SettingsTileButton {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let mut pointer = ctx.pointer();
        let focusable = ctx.focusable();

        focusable.set_focus_key(FocusKey::new(self.spec.focus_key_id));
        focusable.set_preferred_focus(self.spec.is_preferred_focus);

        if pointer.just_pressed() {
            focusable.request_focus(FocusOrigin::Pointer);
        }

        if self.should_request_focus {
            focusable.request_focus(FocusOrigin::Programmatic);
        }

        let state = QuickSettingsControlState {
            is_hovered: pointer.is_hovering(),
            is_focused: focusable.is_focused(),
        };

        if (pointer.just_pressed() || focusable.just_activated())
            && let Some(channel) = &self.activation_channel
        {
            let _ = channel.send(());
        }

        Element::new()
            .with_tag(self.spec.tag)
            .with_style(settings_tile_button_style(state, ctx))
            .with_content(settings_tile_content(ctx, self.spec, self.is_active))
    }
}

fn settings_tile_content(
    ctx: &mut ComponentContext,
    spec: SettingsTileSpec,
    is_active: bool,
) -> Element {
    Element::new()
        .with_style(settings_tile_content_style())
        .with_content(
            Element::new()
                .with_style(settings_tile_icon_style(ctx, is_active))
                .with_content(glyph_element(
                    spec.glyph,
                    SETTINGS_ICON_SIZE,
                    SETTINGS_ICON_FRAME_SIZE,
                    settings_tile_icon_color(is_active),
                )),
        )
        .with_content(
            Element::new()
                .with_style(settings_tile_text_column_style())
                .with_content(Text::new(spec.label).with_style(tile_title_style())),
        )
}
