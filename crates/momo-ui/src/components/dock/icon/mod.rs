mod style;

pub(super) use self::style::{
    DOCK_BUTTON_SIZE, DOCK_ICON_GLYPH_SIZE, DOCK_ICON_SIZE, dock_button_style,
    hidden_dock_button_style,
};
use crate::components::home::{
    app_icon::app_icon,
    app_tile::{AppInfo, send_app_launch_request},
    model::{LaunchRestoreFocus, tile_focus_transform},
};
use daiko::{
    Element, Vec2,
    component::{Component, ComponentContext},
    navigation::FocusKey,
};
use momo_kit::interaction::ButtonBehavior;

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
                .with_style(hidden_dock_button_style());
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

        let style = dock_button_style(
            ctx,
            self.app.accent,
            &button_transform,
            button.is_hovering,
            button.is_focus_visible,
        );

        Element::new()
            .with_tag(format!("dock-app-{}", self.app.id()))
            .with_style(style)
            .with_content(app_icon(&self.app.icon, DOCK_ICON_GLYPH_SIZE))
    }
}
