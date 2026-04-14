use crate::components::home::model::{
    LaunchRequest, MockApp, TILE_BORDER_RADIUS, TILE_HEIGHT, TILE_WIDTH, color,
};
use daiko::Element;
use daiko::animation::{AnimationParameters, transition};
use daiko::channel::Channel;
use daiko::component::{Component, ComponentContext};
use daiko::navigation::{FocusKey, FocusOrigin};
use daiko::style::{Border, BorderRadius, Color, Stroke, Style};
use daiko::widgets::container::{Container, Fit};
use daiko::widgets::text::{Text, TextStyle, TextWrap};
use std::time::Duration;

#[derive(Clone)]
pub(super) struct AppTile {
    pub app: MockApp,
    pub preferred_focus: bool,
    pub launch_channel: Channel<LaunchRequest>,
    pub interactions_disabled: bool,
    pub is_hidden_for_launch: bool,
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

        if !self.interactions_disabled && pointer.just_entered() {
            focusable.request_focus(FocusOrigin::Pointer);
        }

        let pointer_activated = !self.interactions_disabled && pointer.just_pressed();
        let focus_activated = !self.interactions_disabled && focusable.just_activated();
        let just_activated = pointer_activated || focus_activated;

        if pointer_activated {
            focusable.request_focus(FocusOrigin::Pointer);
        }

        if just_activated {
            if let Some(layout) = layout {
                let _ = self.launch_channel.send(LaunchRequest {
                    app: self.app,
                    position: layout.position_absolute,
                    size: layout.size,
                });
            }
            println!("Activated app: {}", self.app.name);
        }

        if self.is_hidden_for_launch {
            return Element::new().with_tag(self.app.id).with_style(
                Style::new()
                    .with_fixed_size(TILE_WIDTH, TILE_HEIGHT)
                    .with_background_color(Color::TRANSPARENT),
            );
        }

        let is_pressed = !self.interactions_disabled && pointer.is_pressed();
        let is_focus_visible = focusable.is_focus_visible();
        let accent = color(self.app.accent);
        let icon_background = accent.gamma_multiply(0.2);
        let icon_text_color = accent.gamma_multiply(1.1);

        let background = if is_focus_visible {
            Color::from_rgb(30, 41, 60)
        } else {
            Color::from_rgb(20, 26, 38)
        };
        // let background = if is_pressed {
        //     Color::from_rgb(38, 47, 68)
        // } else if is_focus_visible {
        //     Color::from_rgb(30, 41, 60)
        // } else {
        //     Color::from_rgb(20, 26, 38)
        // };

        let border_color = if is_focus_visible {
            accent
        } else {
            Color::from_rgb(52, 65, 89)
        };

        let style = Style::new()
            .with_fixed_size(TILE_WIDTH, TILE_HEIGHT)
            .with_direction(daiko::layout::FlexDirection::Column)
            .with_align_items(daiko::layout::AlignItems::FlexStart)
            .with_padding(16.0)
            .with_spacing((12.0, 12.0))
            .with_background_color(transition(
                background,
                AnimationParameters::default()
                    .with_duration(Duration::from_millis(180))
                    .to_transition_options(),
                ctx,
            ))
            .with_border(Border::uniform(Stroke::new(
                2.0,
                transition(
                    border_color,
                    AnimationParameters::default()
                        .with_duration(Duration::from_millis(180))
                        .to_transition_options(),
                    ctx,
                ),
            )))
            .with_border_radius(BorderRadius::all(TILE_BORDER_RADIUS));

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
            )
            .with_content(
                Text::new(self.app.subtitle).with_style(
                    TextStyle::default()
                        .with_font_size(13.0)
                        .with_font_color(Color::from_rgb(132, 149, 179))
                        .with_wrap(TextWrap::NoWrap),
                ),
            );

        let tile = Element::new()
            .with_tag(self.app.id)
            .with_style(style)
            .with_content(icon)
            .with_content(meta);

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

        tile
    }
}
