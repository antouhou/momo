mod style;

use crate::components::home::app_icon::{
    app_icon, app_icon_background_color, app_icon_foreground_color,
};
use crate::components::home::app_tile::style::{tile_style, tile_title_style};
use crate::components::home::model::{
    AppEntry, AppLaunch, HOME_LAUNCH_CHANNEL_ID, LaunchRequest, TILE_HEIGHT, TILE_ICON_GLYPH_SIZE,
    TILE_ICON_SIZE, TILE_WIDTH, color, tile_focus_transform, tile_icon_origin,
    transformed_local_rect,
};
use daiko::Element;
use daiko::Vec2;
use daiko::component::{Component, ComponentContext};
use daiko::navigation::{FocusKey, FocusOrigin, NavigationDirection};
use daiko::style::{BorderRadius, Color, CursorIcon, Style};
use daiko::widgets::container::{Container, Fit};
use daiko::widgets::text::Text;
use std::rc::Rc;

#[derive(Clone)]
pub(super) struct AppTile {
    pub app: Rc<AppEntry>,
    pub preferred_focus: bool,
    pub interactions_disabled: bool,
    pub is_hidden_for_launch: bool,
    pub focus_left_key: Option<FocusKey>,
    pub focus_right_key: Option<FocusKey>,
    pub focus_down_key: Option<FocusKey>,
}

impl Component for AppTile {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let mut pointer = ctx.pointer();
        let focusable = ctx.focusable();
        let layout = ctx.layout();

        focusable.set_navigation_enabled(!self.interactions_disabled);
        focusable.set_focus_key(FocusKey::new(self.app.id()));
        focusable.set_preferred_focus(self.preferred_focus);
        focusable.set_focus_directional_override(NavigationDirection::Left, self.focus_left_key);
        focusable.set_focus_directional_override(NavigationDirection::Right, self.focus_right_key);
        focusable.set_focus_directional_override(NavigationDirection::Down, self.focus_down_key);

        let pointer_activated = !self.interactions_disabled && pointer.just_pressed();
        let focus_activated = !self.interactions_disabled && focusable.just_activated();
        let just_activated = pointer_activated || focus_activated;

        if pointer_activated {
            focusable.request_focus(FocusOrigin::Pointer);
        }

        if self.is_hidden_for_launch {
            return Element::new().with_tag(self.app.id()).with_style(
                Style::new()
                    .with_fixed_size(TILE_WIDTH, TILE_HEIGHT)
                    .with_background_color(Color::TRANSPARENT),
            );
        }

        let is_hovering = !self.interactions_disabled && pointer.is_hovering();
        let is_focus_visible = focusable.is_focus_visible();
        let _is_pressed = !self.interactions_disabled && pointer.is_pressed();
        let paint_decorations = is_focus_visible || is_hovering;
        let accent = color(self.app.accent);
        let icon_background = app_icon_background_color(accent);
        let icon_text_color = app_icon_foreground_color(accent);

        let tile_transform =
            tile_focus_transform(Vec2::new(TILE_WIDTH, TILE_HEIGHT), is_focus_visible, ctx);

        let mut style = tile_style(ctx, accent, &tile_transform, is_hovering, is_focus_visible);

        if just_activated {
            if let Some(layout) = layout {
                let (surface_position, surface_size) = transformed_local_rect(
                    layout.position_absolute,
                    &tile_transform,
                    Vec2::zero(),
                    layout.size,
                );
                let (icon_position, icon_size) = transformed_local_rect(
                    layout.position_absolute,
                    &tile_transform,
                    tile_icon_origin(),
                    Vec2::new(TILE_ICON_SIZE, TILE_ICON_SIZE),
                );
                let launch_channel = ctx.use_channel_with_id(HOME_LAUNCH_CHANNEL_ID);
                let _ = launch_channel.send(LaunchRequest {
                    app: Rc::clone(&self.app),
                    position: surface_position,
                    size: surface_size,
                    icon_position,
                    icon_size,
                });
            }
            match &self.app.launch {
                AppLaunch::Mock => println!("Activated app: {}", self.app.name()),
            }
        }

        if is_hovering {
            style.set_cursor(CursorIcon::PointingHand)
        }

        let icon = Element::new()
            .with_style(
                Style::new()
                    .with_fixed_size(TILE_ICON_SIZE, TILE_ICON_SIZE)
                    .with_centered_content()
                    .with_background_color(icon_background)
                    .with_border_radius(BorderRadius::all(14.0)),
            )
            .with_content(app_icon(
                &self.app.icon,
                TILE_ICON_GLYPH_SIZE,
                icon_text_color,
            ));

        let meta = Container::vertical()
            .with_fit(Fit::new().exact_content_size())
            .align_items_center()
            .with_spacing((4.0, 4.0))
            .build()
            .with_content(Text::new(Rc::clone(&self.app.name)).with_style(tile_title_style()));

        // TODO: better focus ring
        // if is_focus_visible
        //     && let Some(focus_ring) = adorners::focus_outline(
        //         ctx.layout(),
        //         BorderRadius::all(18.0),
        //         ctx.theme().focus.outline,
        //     )
        // {
        //     tile.add_content(focus_ring);
        // }

        let element = Element::new()
            .with_tag(self.app.id())
            .with_style(style)
            .with_content(icon)
            .with_content(meta);

        if paint_decorations {
            // let color = transition(
            //     border_color,
            //     AnimationParameters::default()
            //         .with_duration(Duration::from_millis(TILE_FOCUS_ANIMATION_DURATION_MS))
            //         .to_transition_options(),
            //     ctx,
            // );

            // element.set_effect(
            //     BoxShadow::new()
            //         .with_offset([0.0, 0.0])
            //         .with_color(Color::BLACK)
            //         .with_blur_radius(10.0)
            //         .with_corner_radius(18.0),
            // )
        }

        element
    }
}
