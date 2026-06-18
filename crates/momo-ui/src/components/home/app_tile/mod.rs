mod style;

use crate::app_state::AppEntry;
use crate::components::home::app_icon::app_icon;
use crate::components::home::app_tile::style::{tile_style, tile_title_style};
use crate::components::home::model::{
    HOME_LAUNCH_CHANNEL_ID, LaunchRequest, LaunchRestoreFocus, TILE_HEIGHT, TILE_ICON_GLYPH_SIZE,
    TILE_ICON_SIZE, TILE_WIDTH, tile_focus_transform, tile_icon_origin, transformed_local_rect,
};
use daiko::Element;
use daiko::Vec2;
use daiko::component::{Component, ComponentContext};
use daiko::layout::Layout;
use daiko::navigation::{FocusKey, FocusOrigin};
use daiko::style::{Color, CursorIcon, Style};
use daiko::widgets::container::{Container, Fit};
use daiko::widgets::text::Text;
use std::path::PathBuf;
use std::sync::Arc;

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

pub(crate) struct AppButtonBehavior {
    pub(crate) focus_key: FocusKey,
    pub(crate) preferred_focus: bool,
    pub(crate) interactions_disabled: bool,
}

pub(crate) struct AppButtonState {
    pub(crate) is_hovering: bool,
    pub(crate) is_focus_visible: bool,
    pub(crate) just_activated: bool,
    pub(crate) layout: Option<Layout>,
}

pub(crate) fn use_app_button_behavior(
    ctx: &mut ComponentContext,
    behavior: AppButtonBehavior,
) -> AppButtonState {
    let mut pointer = ctx.pointer();
    let focusable = ctx.focusable();
    let layout = ctx.layout();

    focusable.set_navigation_enabled(!behavior.interactions_disabled);
    focusable.set_focus_key(behavior.focus_key);
    focusable.set_preferred_focus(behavior.preferred_focus);

    let pointer_activated = !behavior.interactions_disabled && pointer.just_pressed();
    let focus_activated = !behavior.interactions_disabled && focusable.just_activated();

    if pointer_activated {
        focusable.request_focus(FocusOrigin::Pointer);
    }

    let is_hovering = !behavior.interactions_disabled && pointer.is_hovering();
    let is_focus_visible = focusable.is_focus_visible();

    AppButtonState {
        is_hovering,
        is_focus_visible,
        just_activated: pointer_activated || focus_activated,
        layout,
    }
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
    let (surface_position, surface_size) = transformed_local_rect(
        layout.position_absolute,
        transform,
        Vec2::zero(),
        layout.size,
    );
    let (icon_position, icon_size) =
        transformed_local_rect(layout.position_absolute, transform, icon_origin, icon_size);
    let launch_channel = ctx.use_channel_with_id(HOME_LAUNCH_CHANNEL_ID);
    let _ = launch_channel.send(LaunchRequest {
        app,
        restore_focus,
        position: surface_position,
        size: surface_size,
        icon_position,
        icon_size,
    });
}

impl Component for AppTile {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let button = use_app_button_behavior(
            ctx,
            AppButtonBehavior {
                focus_key: FocusKey::new(self.app.id()),
                preferred_focus: self.preferred_focus,
                interactions_disabled: self.interactions_disabled,
            },
        );

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
        // TODO: use id to actually launch an app using app launcher
        // match &self.app.launch {
        //     AppLaunch::Mock => println!("Activated app: {}", self.app.name()),
        // }

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
