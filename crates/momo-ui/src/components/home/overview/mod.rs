mod state;
mod style;

use self::{
    state::{OVERVIEW_CARD_COUNT, OverviewCarouselAction, OverviewCarouselState},
    style::{
        OverviewCardFrame, overview_card_layout_style, overview_card_stage_style,
        overview_card_surface_style, overview_card_target_frame, overview_carousel_style,
        overview_style, overview_window_close_button_style, overview_window_close_target_position,
        overview_window_close_text_style,
    },
};
use crate::components::home::paging::scroll_page_delta;
use daiko::{
    Element, Id, Vec2,
    animation::SmoothFollowConfig,
    channel::Channel,
    component::{Component, ComponentContext},
    navigation::{FocusBoundary, NavigationInputAction},
    widgets::text::Text,
};
use momo_kit::interaction::ButtonBehavior;
use std::time::Duration;

const OVERVIEW_SCROLL_STATE_ID: &str = "momo_home_overview_scroll_state";
const OVERVIEW_CARD_POSITION_MOTION_ID: &str = "momo_home_overview_card_position_motion";
const OVERVIEW_CARD_SIZE_MOTION_ID: &str = "momo_home_overview_card_size_motion";
const OVERVIEW_WINDOW_CLOSE_POSITION_MOTION_ID: &str =
    "momo_home_overview_window_close_position_motion";
const OVERVIEW_FALLBACK_VIEWPORT_SIZE: Vec2 = Vec2::new(1200.0, 360.0);
const OVERVIEW_PAGE_MOTION_DURATION_MS: u64 = 260;

#[derive(Clone, Copy)]
pub(super) enum OverviewCardPosition {
    Previous,
    Active,
    Next,
    Hidden,
}

impl OverviewCardPosition {
    fn tag(self) -> &'static str {
        match self {
            Self::Previous => "overview-card-previous",
            Self::Active => "overview-card-active",
            Self::Next => "overview-card-next",
            Self::Hidden => "overview-card-hidden",
        }
    }

    fn action(self) -> Option<OverviewCarouselAction> {
        match self {
            Self::Previous => Some(OverviewCarouselAction::ShowPrevious),
            Self::Next => Some(OverviewCarouselAction::ShowNext),
            Self::Active | Self::Hidden => None,
        }
    }

    fn is_interactive(self) -> bool {
        !matches!(self, Self::Hidden)
    }
}

pub(super) struct Overview {
    show_apps_channel: Channel<()>,
}

impl Overview {
    pub(super) fn new(show_apps_channel: Channel<()>) -> Self {
        Self { show_apps_channel }
    }
}

impl Component for Overview {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let focus_scope = ctx.focus_scope();
        focus_scope.set_boundary(FocusBoundary::Escape);
        focus_scope.capture_when_contains_focus(&[
            NavigationInputAction::Cancel,
            NavigationInputAction::Back,
        ]);
        if focus_scope.drain_captured_actions().next().is_some() {
            let _ = self.show_apps_channel.send(());
        }

        Element::new()
            .with_tag("overview")
            .with_style(overview_style())
            .with_content(OverviewCarousel)
    }
}

#[derive(Clone, Copy)]
struct OverviewCarousel;

impl Component for OverviewCarousel {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let carousel_state = ctx.use_local_state(OverviewCarouselState::default);
        let action_channel: Channel<OverviewCarouselAction> = ctx.create_channel();
        let focus_scope = ctx.focus_scope();
        focus_scope.set_boundary(FocusBoundary::Escape);
        let current_state = *carousel_state.read();
        let mut next_state = current_state;

        for action in action_channel.iter() {
            next_state.apply(action);
        }

        if let Some(page_delta) = scroll_page_delta(ctx, Id::new(OVERVIEW_SCROLL_STATE_ID)) {
            let action = if page_delta.is_negative() {
                OverviewCarouselAction::ShowPrevious
            } else {
                OverviewCarouselAction::ShowNext
            };
            next_state.apply(action);
        }

        if next_state != current_state {
            *carousel_state.write() = next_state;
        }

        let active_card_index = next_state.active_card_index();
        let viewport_size = ctx
            .layout()
            .map(|layout| layout.size)
            .filter(|size| size.x > 0.0 && size.y > 0.0)
            .unwrap_or(OVERVIEW_FALLBACK_VIEWPORT_SIZE);
        let mut card_stage = Element::new()
            .with_tag("overview-card-stage")
            .with_style(overview_card_stage_style());

        for card_index in 0..OVERVIEW_CARD_COUNT {
            let position = overview_card_position(card_index, next_state);
            card_stage.add_content(OverviewCard {
                card_index,
                active_card_index: active_card_index.unwrap_or(OVERVIEW_CARD_COUNT / 2),
                position,
                viewport_size,
                action_channel: action_channel.clone(),
            });
        }

        if let Some(active_card_index) = active_card_index {
            let active_card_frame =
                overview_card_target_frame(viewport_size, active_card_index, active_card_index);
            card_stage.add_content(OverviewWindowCloseButton {
                action_channel,
                target_position: overview_window_close_target_position(active_card_frame),
            });
        }

        Element::new()
            .with_tag("overview-carousel")
            .with_style(overview_carousel_style())
            .with_content(card_stage)
    }
}

fn overview_card_position(card_index: usize, state: OverviewCarouselState) -> OverviewCardPosition {
    if !state.is_card_visible(card_index) {
        OverviewCardPosition::Hidden
    } else if state.active_card_index() == Some(card_index) {
        OverviewCardPosition::Active
    } else if state.previous_card_index() == Some(card_index) {
        OverviewCardPosition::Previous
    } else if state.next_card_index() == Some(card_index) {
        OverviewCardPosition::Next
    } else {
        OverviewCardPosition::Hidden
    }
}

struct OverviewCard {
    card_index: usize,
    active_card_index: usize,
    position: OverviewCardPosition,
    viewport_size: Vec2,
    action_channel: Channel<OverviewCarouselAction>,
}

impl Component for OverviewCard {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let target_frame =
            overview_card_target_frame(self.viewport_size, self.card_index, self.active_card_index);
        let motion_config = overview_page_motion_config();
        let rendered_position = {
            let mut position_motion = ctx.smooth_follow_with_id::<Vec2>(
                Id::new(OVERVIEW_CARD_POSITION_MOTION_ID),
                motion_config,
            );
            position_motion.follow(target_frame.position)
        };
        let rendered_size = {
            let mut size_motion = ctx.smooth_follow_with_id::<Vec2>(
                Id::new(OVERVIEW_CARD_SIZE_MOTION_ID),
                motion_config,
            );
            size_motion.follow(target_frame.size)
        };
        let button = ButtonBehavior::new(ctx)
            .with_preferred_focus(matches!(self.position, OverviewCardPosition::Active))
            .with_enabled(self.position.is_interactive())
            .without_layout_tracking()
            .apply();

        let focused_from_navigation = button.just_focused && button.is_focus_visible;
        if (button.just_activated || focused_from_navigation)
            && let Some(action) = self.position.action()
        {
            let _ = self.action_channel.send(action);
        }

        Element::new()
            .with_tag(self.position.tag())
            .with_style(overview_card_layout_style(
                self.position,
                OverviewCardFrame {
                    position: rendered_position,
                    size: rendered_size,
                },
            ))
            .with_content(
                Element::new()
                    .with_tag(format!("overview-card-item-{}", self.card_index))
                    .with_style(overview_card_surface_style(
                        ctx,
                        self.card_index,
                        self.position,
                        button.is_pressed,
                        button.is_hovering,
                        button.is_focused,
                    )),
            )
    }
}

struct OverviewWindowCloseButton {
    action_channel: Channel<OverviewCarouselAction>,
    target_position: Vec2,
}

impl Component for OverviewWindowCloseButton {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let rendered_position = {
            let mut position_motion = ctx.smooth_follow_with_id::<Vec2>(
                Id::new(OVERVIEW_WINDOW_CLOSE_POSITION_MOTION_ID),
                overview_page_motion_config(),
            );
            position_motion.follow(self.target_position)
        };
        let button = ButtonBehavior::new(ctx).without_layout_tracking().apply();

        if button.just_activated {
            let _ = self
                .action_channel
                .send(OverviewCarouselAction::CloseActive);
        }

        Element::new()
            .with_tag("overview-window-close")
            .with_style(overview_window_close_button_style(
                ctx,
                rendered_position,
                button.is_pressed,
                button.is_hovering,
                button.is_focused,
            ))
            .with_content(Text::new("×").with_style(overview_window_close_text_style()))
    }
}

fn overview_page_motion_config() -> SmoothFollowConfig {
    SmoothFollowConfig::new(
        Duration::from_millis(OVERVIEW_PAGE_MOTION_DURATION_MS),
        0.3,
        0.36,
    )
}
