mod style;

use self::style::{
    overview_accent_color, overview_glyph_color, overview_glyph_frame_style, overview_glyph_size,
};
use super::icon::{DOCK_BUTTON_SIZE, dock_button_style};
use crate::components::home::model::tile_focus_transform;
use daiko::{
    Element, Vec2,
    channel::Channel,
    component::{Component, ComponentContext},
    widgets::image::{Image, ImageParams, ImageSource, ImageType},
};
use momo_kit::interaction::ButtonBehavior;

const OVERVIEW_ICON: &[u8] = include_bytes!("../../../../assets/window-maximize.svg");

pub(super) struct OverviewDockButton {
    pub(super) activation_channel: Channel<()>,
    pub(super) interactions_disabled: bool,
    pub(super) is_active: bool,
}

impl Component for OverviewDockButton {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let button = ButtonBehavior::new(ctx)
            .with_enabled(!self.interactions_disabled)
            .apply();
        let button_transform = tile_focus_transform(
            Vec2::new(DOCK_BUTTON_SIZE, DOCK_BUTTON_SIZE),
            button.is_focus_visible,
            ctx,
        );

        if button.just_activated {
            let _ = self.activation_channel.send(());
        }

        Element::new()
            .with_tag("overview-toggle")
            .with_style(dock_button_style(
                ctx,
                overview_accent_color(self.is_active),
                &button_transform,
                button.is_hovering,
                button.is_focus_visible || self.is_active,
            ))
            .with_content(
                Element::new()
                    .with_style(overview_glyph_frame_style())
                    .with_content(
                        Image::new(ImageParams {
                            max_width: overview_glyph_size(),
                            max_height: overview_glyph_size(),
                            image_type: Some(ImageType::Svg),
                            source: ImageSource::BytesSlice(OVERVIEW_ICON),
                        })
                        .fill_color(Some(overview_glyph_color(self.is_active))),
                    ),
            )
    }
}
