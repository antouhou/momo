mod style;

use crate::{
    app_state::{AppCommand, AppEntry, use_apps_state},
    components::home::{
        app_icon::app_icon,
        app_tile::style::{tile_style, tile_title_style},
        model::{
            HOME_LAUNCH_CONTROLLER_REQUEST_CHANNEL_ID, LaunchControllerRequest, LaunchRequest,
            LaunchRestoreFocus, TILE_HEIGHT, TILE_ICON_GLYPH_SIZE, TILE_ICON_SIZE, TILE_WIDTH,
            tile_focus_transform, tile_icon_origin, transformed_local_rect,
        },
    },
};
use daiko::{
    Element, Vec2,
    component::{Component, ComponentContext},
    layout::Layout,
    navigation::FocusKey,
    style::{Color, CursorIcon, Style},
    widgets::{
        container::{Container, Fit},
        text::Text,
    },
};
use momo_kit::interaction::ButtonBehavior;
use std::{path::PathBuf, sync::Arc};
pub(crate) use style::{AppButtonSurfaceMetrics, app_button_surface_style};

#[derive(Clone)]
pub(crate) struct AppInfo {
    pub(crate) name: Arc<String>,
    pub(crate) id: Arc<String>,
    pub(crate) icon: Arc<Option<PathBuf>>,
    pub(crate) accent: Color,
}

impl AppInfo {
    pub fn new(entry: &AppEntry) -> Self {
        Self {
            name: Arc::clone(&entry.name),
            id: Arc::clone(&entry.id),
            icon: Arc::clone(&entry.icon),
            accent: entry.accent,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Clone)]
pub(super) struct AppTile {
    pub app: AppInfo,
    pub preferred_focus: bool,
    pub interactions_disabled: bool,
    pub is_hidden_for_launch: bool,
}

pub(crate) fn send_app_launch_request(
    ctx: &mut ComponentContext,
    app: AppInfo,
    restore_focus: LaunchRestoreFocus,
    layout: Layout,
    transform: &daiko::style::Transform,
    icon_origin: Vec2,
    icon_size: Vec2,
) {
    let apps_state = use_apps_state(ctx);

    {
        let apps = apps_state.write_silent();
        if let Some(sender) = apps.command_sender.as_ref() {
            let _ = sender.send(AppCommand::LaunchApp(app.id.to_string()));
        }
    }

    let (surface_position, surface_size) = transformed_local_rect(
        layout.position_absolute,
        transform,
        Vec2::zero(),
        layout.size,
    );
    let (icon_position, icon_size) =
        transformed_local_rect(layout.position_absolute, transform, icon_origin, icon_size);
    let launch_controller_request_channel =
        ctx.use_channel_with_id(HOME_LAUNCH_CONTROLLER_REQUEST_CHANNEL_ID);
    let _ = launch_controller_request_channel.send(LaunchControllerRequest::BeginLaunchAnimation(
        LaunchRequest {
            app,
            restore_focus,
            position: surface_position,
            size: surface_size,
            icon_position,
            icon_size,
        },
    ));
}

impl Component for AppTile {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let button = ButtonBehavior::new(ctx)
            .with_focus_key(FocusKey::new(self.app.id()))
            .with_preferred_focus(self.preferred_focus)
            .with_enabled(!self.interactions_disabled)
            .apply();

        if self.is_hidden_for_launch {
            return Element::new().with_tag(self.app.id()).with_style(
                Style::new()
                    .with_fixed_size(TILE_WIDTH, TILE_HEIGHT)
                    .with_background_color(Color::TRANSPARENT),
            );
        }

        let accent = self.app.accent;
        let tile_transform = tile_focus_transform(
            Vec2::new(TILE_WIDTH, TILE_HEIGHT),
            button.is_focus_visible,
            ctx,
        );

        if button.just_activated
            && let Some(layout) = button.layout
        {
            send_app_launch_request(
                ctx,
                self.app.clone(),
                LaunchRestoreFocus::AppGrid,
                layout,
                &tile_transform,
                tile_icon_origin(),
                Vec2::new(TILE_ICON_SIZE, TILE_ICON_SIZE),
            );
        }

        let mut style = tile_style(
            ctx,
            accent,
            &tile_transform,
            button.is_hovering,
            button.is_focus_visible,
        );

        if button.is_hovering {
            style.set_cursor(CursorIcon::PointingHand)
        }

        let icon = Element::new()
            .with_style(
                Style::new()
                    .with_fixed_size(TILE_ICON_SIZE, TILE_ICON_SIZE)
                    .with_centered_content(),
            )
            .with_content(app_icon(&self.app.icon, TILE_ICON_GLYPH_SIZE));

        let meta = Container::vertical()
            .with_fit(Fit::new().exact_content_size())
            .align_items_center()
            .with_spacing((4.0, 4.0))
            .build()
            .with_content(Text::new(Arc::clone(&self.app.name)).with_style(tile_title_style()));

        Element::new()
            .with_tag(self.app.id())
            .with_style(style)
            .with_content(icon)
            .with_content(meta)
    }
}
