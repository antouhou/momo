use crate::components::home::app_icon::app_icon;
use crate::components::home::app_tile::{
    AppButtonSurfaceMetrics, AppInfo, app_button_surface_style, send_app_launch_request,
};
use crate::components::home::model::{LaunchRestoreFocus, TILE_BORDER_WIDTH, tile_focus_transform};
use daiko::Element;
use daiko::Vec2;
use daiko::component::{Component, ComponentContext};
use daiko::navigation::FocusKey;
use daiko::style::{Color, CursorIcon, Style};
use momo_kit::interaction::ButtonBehavior;

const DOCK_BUTTON_SIZE: f32 = 72.0;
const DOCK_BUTTON_RADIUS: f32 = 16.0;
const DOCK_ICON_SIZE: f32 = 52.0;
const DOCK_ICON_GLYPH_SIZE: usize = 52;

pub struct DockIcon {
    pub(crate) app: AppInfo,
    pub(crate) focus_key: FocusKey,
    pub(crate) preferred_focus: bool,
    pub(crate) interactions_disabled: bool,
    pub(crate) is_hidden_for_launch: bool,
}

impl Component for DockIcon {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let button = ButtonBehavior::new(ctx)
            .with_focus_key(self.focus_key)
            .with_preferred_focus(self.preferred_focus)
            .with_enabled(!self.interactions_disabled)
            .apply();

        if self.is_hidden_for_launch {
            return Element::new()
                .with_tag(format!("dock-app-{}", self.app.id()))
                .with_style(
                    Style::new()
                        .with_fixed_size(DOCK_BUTTON_SIZE, DOCK_BUTTON_SIZE)
                        .with_background_color(Color::TRANSPARENT),
                );
        }

        let icon_origin = Vec2::new(
            (DOCK_BUTTON_SIZE - DOCK_ICON_SIZE) / 2.0,
            (DOCK_BUTTON_SIZE - DOCK_ICON_SIZE) / 2.0,
        );
        let button_transform = tile_focus_transform(
            Vec2::new(DOCK_BUTTON_SIZE, DOCK_BUTTON_SIZE),
            button.is_focus_visible,
            ctx,
        );

        if button.just_activated
            && let Some(layout) = button.layout
        {
            send_app_launch_request(
                ctx,
                self.app.clone(),
                LaunchRestoreFocus::Dock(self.focus_key),
                layout,
                &button_transform,
                icon_origin,
                Vec2::new(DOCK_ICON_SIZE, DOCK_ICON_SIZE),
            );
        }

        let mut style = app_button_surface_style(
            ctx,
            AppButtonSurfaceMetrics {
                width: DOCK_BUTTON_SIZE,
                height: DOCK_BUTTON_SIZE,
                border_radius: DOCK_BUTTON_RADIUS,
                border_width: TILE_BORDER_WIDTH,
            },
            self.app.accent,
            &button_transform,
            button.is_hovering,
            button.is_focus_visible,
        )
        .with_centered_content();

        if button.is_hovering {
            style.set_cursor(CursorIcon::PointingHand);
        }

        Element::new()
            .with_tag(format!("dock-app-{}", self.app.id()))
            .with_style(style)
            .with_content(app_icon(&self.app.icon, DOCK_ICON_GLYPH_SIZE))
    }
}
