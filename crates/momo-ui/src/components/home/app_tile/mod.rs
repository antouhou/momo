mod style;

use crate::components::home::app_tile::style::tile_style;
use crate::components::home::model::{
    HOME_LAUNCH_CHANNEL_ID, LaunchRequest, MockApp, TILE_BORDER_RADIUS,
    TILE_FOCUS_ANIMATION_DURATION_MS, TILE_HEIGHT, TILE_ICON_OFFSET, TILE_ICON_SIZE, TILE_WIDTH,
    color, tile_focus_transform, transformed_local_rect,
};
use daiko::Element;
use daiko::Vec2;
use daiko::animation::{AnimationParameters, transition};
use daiko::component::{Component, ComponentContext};
use daiko::navigation::{FocusKey, FocusOrigin, NavigationDirection};
use daiko::style::{Border, BorderRadius, Color, CursorIcon, Stroke, Style};
use daiko::widgets::container::{Container, Fit};
use daiko::widgets::text::{Text, TextStyle, TextWrap};
use std::time::Duration;
use tracing::info;

#[derive(Clone)]
pub(super) struct AppTile {
    pub app: MockApp,
    pub preferred_focus: bool,
    pub interactions_disabled: bool,
    pub is_hidden_for_launch: bool,
    pub focus_left_app_id: Option<&'static str>,
    pub focus_right_app_id: Option<&'static str>,
    pub focus_down_key: Option<FocusKey>,
}

impl Component for AppTile {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let mut pointer = ctx.pointer();
        let focusable = ctx.focusable();
        let layout = ctx
            .peek_element_layout(&ctx.element_id())
            .copied()
            .or_else(|| ctx.layout());

        focusable.set_navigation_enabled(!self.interactions_disabled);
        focusable.set_focus_key(FocusKey::new(self.app.id));
        focusable.set_preferred_focus(self.preferred_focus);
        focusable.set_focus_directional_override(
            NavigationDirection::Left,
            self.focus_left_app_id.map(FocusKey::new),
        );
        focusable.set_focus_directional_override(
            NavigationDirection::Right,
            self.focus_right_app_id.map(FocusKey::new),
        );
        focusable.set_focus_directional_override(NavigationDirection::Down, self.focus_down_key);

        let pointer_activated = !self.interactions_disabled && pointer.just_pressed();
        let focus_activated = !self.interactions_disabled && focusable.just_activated();
        let just_activated = pointer_activated || focus_activated;

        if pointer_activated {
            focusable.request_focus(FocusOrigin::Pointer);
        }

        if self.is_hidden_for_launch {
            return Element::new().with_tag(self.app.id).with_style(
                Style::new()
                    .with_fixed_size(TILE_WIDTH, TILE_HEIGHT)
                    .with_background_color(Color::TRANSPARENT),
            );
        }

        let is_hovering = !self.interactions_disabled && pointer.is_hovering();
        let _is_pressed = !self.interactions_disabled && pointer.is_pressed();
        let paint_decorations = focusable.is_focus_visible() || is_hovering;
        let accent = color(self.app.accent);
        let icon_background = accent.gamma_multiply(0.2);
        let icon_text_color = accent.gamma_multiply(1.1);

        let tile_transform =
            tile_focus_transform(Vec2::new(TILE_WIDTH, TILE_HEIGHT), paint_decorations, ctx);

        let mut style = tile_style(ctx, accent, &tile_transform, paint_decorations);

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
                    Vec2::new(TILE_ICON_OFFSET, TILE_ICON_OFFSET),
                    Vec2::new(TILE_ICON_SIZE, TILE_ICON_SIZE),
                );
                let launch_channel = ctx.use_channel_with_id(HOME_LAUNCH_CHANNEL_ID);
                let _ = launch_channel.send(LaunchRequest {
                    app: self.app,
                    position: surface_position,
                    size: surface_size,
                    icon_position,
                    icon_size,
                });
            }
            println!("Activated app: {}", self.app.name);
        }

        if is_hovering {
            style.set_cursor(CursorIcon::PointingHand)
        }

        let icon = Element::new()
            .with_style(
                Style::new()
                    .with_fixed_size(72.0, 72.0)
                    .with_centered_content()
                    .with_background_color(icon_background)
                    .with_border_radius(BorderRadius::all(14.0)),
            )
            .with_content(
                Text::new(self.app.badge).with_style(
                    TextStyle::default()
                        .with_font_size(28.0)
                        .with_font_color(icon_text_color)
                        .with_center_alignment(),
                ),
            );

        let meta = Container::vertical()
            .with_fit(Fit::new().at_least_parent_width().at_least_content_height())
            .with_spacing((4.0, 4.0))
            .build()
            .with_content(
                Text::new(self.app.name).with_style(
                    TextStyle::default()
                        .with_font_size(18.0)
                        .with_font_color(Color::from_rgb(240, 245, 255))
                        .with_wrap(TextWrap::NoWrap),
                ),
            );

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
            .with_tag(self.app.id)
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
