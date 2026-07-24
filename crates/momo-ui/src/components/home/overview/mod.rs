mod state;
mod style;
mod window_controls;

use self::{
    state::{OverviewCarouselAction, OverviewCarouselState},
    style::{
        OverviewCardFrame, overview_card_layout_style, overview_card_stage_style,
        overview_card_surface_style, overview_card_target_frame, overview_carousel_style,
        overview_empty_state_style, overview_empty_state_text_style, overview_style,
    },
    window_controls::OverviewWindowControls,
};
use super::compositor::use_compositor_integration_state;
use super::state::{HomeView, use_home_view_request_channel};
use super::surface_layer_controller::use_hide_shell_channel;
use daiko::{
    Element, Id, Vec2,
    animation::SmoothFollowConfig,
    channel::Channel,
    component::{Component, ComponentContext},
    navigation::NavigationInputAction,
    widgets::text::Text,
};
use momo_compositor::{CompositorCommand, CompositorCommandSender};
use momo_kit::interaction::{ButtonBehavior, PageScrollDirection, ScrollPagingBehavior};
use std::{sync::Arc, time::Duration};

const OVERVIEW_SCROLL_STATE_ID: &str = "momo_home_overview_scroll_state";
const OVERVIEW_CARD_POSITION_MOTION_ID: &str = "momo_home_overview_card_position_motion";
const OVERVIEW_CARD_SIZE_MOTION_ID: &str = "momo_home_overview_card_size_motion";
const OVERVIEW_FALLBACK_VIEWPORT_SIZE: Vec2 = Vec2::new(1200.0, 360.0);
const OVERVIEW_PAGE_MOTION_DURATION_MS: u64 = 260;
const OVERVIEW_EMPTY_STATE_MESSAGE: &str = "No apps are currently open";
const WINDOW_SWITCH_REQUEST_CHANNEL_ID: &str = "momo_home_window_switch_request_channel";

#[derive(Clone, Copy)]
pub(super) enum WindowSwitchRequest {
    Begin,
    CyclePrevious,
    CycleNext,
    Commit,
}

pub(super) fn use_window_switch_request_channel(
    ctx: &mut ComponentContext,
) -> Channel<WindowSwitchRequest> {
    ctx.use_channel_with_id(WINDOW_SWITCH_REQUEST_CHANNEL_ID)
}

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

pub(super) struct Overview;

impl Component for Overview {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let home_view_request_channel = use_home_view_request_channel(ctx);
        let focus_scope = ctx.focus_scope();
        focus_scope.capture_when_contains_focus(&[
            NavigationInputAction::Cancel,
            NavigationInputAction::Back,
        ]);
        if focus_scope.drain_captured_actions().next().is_some() {
            let _ = home_view_request_channel.send(HomeView::Apps);
        }

        Element::new()
            .with_tag("overview")
            .with_style(overview_style())
            .with_content(OverviewCarousel)
    }
}

struct OverviewCarousel;

impl Component for OverviewCarousel {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let carousel_state = ctx.use_local_state(OverviewCarouselState::default);
        let action_channel: Channel<OverviewCarouselAction> = ctx.create_channel();
        let window_switch_request_channel = use_window_switch_request_channel(ctx);
        let home_view_request_channel = use_home_view_request_channel(ctx);
        let hide_shell_channel = use_hide_shell_channel(ctx);
        let compositor_integration = use_compositor_integration_state(ctx);
        let compositor_integration = compositor_integration.read();
        let views = &compositor_integration.snapshot.views;
        let card_count = views.len();
        let preferred_card_index = views.iter().position(|view| view.is_focused);
        ctx.focus_scope();
        let current_state = *carousel_state.read();
        let mut next_state = current_state;
        next_state.reconcile(card_count, preferred_card_index);

        for action in action_channel.iter() {
            next_state.apply(action, card_count);
        }

        let mut commit_window_switch_requested = false;
        for request in window_switch_request_channel.iter() {
            match request {
                WindowSwitchRequest::Begin => {
                    next_state.begin_window_switch(card_count, preferred_card_index);
                }
                WindowSwitchRequest::CyclePrevious => {
                    next_state.apply(OverviewCarouselAction::CyclePrevious, card_count);
                }
                WindowSwitchRequest::CycleNext => {
                    next_state.apply(OverviewCarouselAction::CycleNext, card_count);
                }
                WindowSwitchRequest::Commit => {
                    commit_window_switch_requested = true;
                }
            }
        }

        if let Some(page_scroll_direction) =
            ScrollPagingBehavior::new(ctx, Id::new(OVERVIEW_SCROLL_STATE_ID)).apply()
        {
            let action = match page_scroll_direction {
                PageScrollDirection::Previous => OverviewCarouselAction::ShowPrevious,
                PageScrollDirection::Next => OverviewCarouselAction::ShowNext,
            };
            next_state.apply(action, card_count);
        }

        if next_state != current_state {
            *carousel_state.write() = next_state;
        }

        let active_card_index = next_state.active_card_index();
        if commit_window_switch_requested
            && let Some(active_card_index) = active_card_index
            && let Some(command_sender) = &compositor_integration.command_sender
        {
            focus_view_and_hide_shell(
                views[active_card_index].identifier,
                command_sender,
                &home_view_request_channel,
                &hide_shell_channel,
            );
        }
        let viewport_size = ctx
            .layout()
            .map(|layout| layout.size)
            .filter(|size| size.x > 0.0 && size.y > 0.0)
            .unwrap_or(OVERVIEW_FALLBACK_VIEWPORT_SIZE);
        let mut card_stage = Element::new()
            .with_tag("overview-card-stage")
            .with_style(overview_card_stage_style());

        if views.is_empty() {
            card_stage.add_content(
                Element::new()
                    .with_tag("overview-empty-state")
                    .with_style(overview_empty_state_style())
                    .with_content(
                        Text::new(OVERVIEW_EMPTY_STATE_MESSAGE)
                            .with_style(overview_empty_state_text_style()),
                    ),
            );
        }

        for (card_index, view) in views.iter().enumerate() {
            let position = overview_card_position(card_index, next_state, card_count);
            card_stage.add_content(OverviewCard {
                card_index,
                active_card_index: active_card_index.unwrap_or(0),
                view_id: view.identifier,
                position,
                viewport_size,
                action_channel: action_channel.clone(),
                command_sender: compositor_integration.command_sender.clone(),
            });
        }

        if let Some(active_card_index) = active_card_index {
            let active_card_frame =
                overview_card_target_frame(viewport_size, active_card_index, active_card_index);
            let active_view = &views[active_card_index];
            card_stage.add_content(OverviewWindowControls {
                view_id: active_view.identifier,
                window_title: Arc::clone(&active_view.title),
                command_sender: compositor_integration.command_sender.clone(),
                active_card_frame,
            });
        }

        Element::new()
            .with_tag("overview-carousel")
            .with_style(overview_carousel_style())
            .with_content(card_stage)
    }
}

fn overview_card_position(
    card_index: usize,
    state: OverviewCarouselState,
    card_count: usize,
) -> OverviewCardPosition {
    if state.active_card_index() == Some(card_index) {
        OverviewCardPosition::Active
    } else if state.previous_card_index() == Some(card_index) {
        OverviewCardPosition::Previous
    } else if state.next_card_index(card_count) == Some(card_index) {
        OverviewCardPosition::Next
    } else {
        OverviewCardPosition::Hidden
    }
}

struct OverviewCard {
    card_index: usize,
    active_card_index: usize,
    view_id: u64,
    position: OverviewCardPosition,
    viewport_size: Vec2,
    action_channel: Channel<OverviewCarouselAction>,
    command_sender: Option<CompositorCommandSender>,
}

impl Component for OverviewCard {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let home_view_request_channel = use_home_view_request_channel(ctx);
        let hide_shell_channel = use_hide_shell_channel(ctx);
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
        if button.just_activated && matches!(self.position, OverviewCardPosition::Active) {
            if let Some(command_sender) = &self.command_sender {
                focus_view_and_hide_shell(
                    self.view_id,
                    command_sender,
                    &home_view_request_channel,
                    &hide_shell_channel,
                );
            }
        } else if (button.just_activated || focused_from_navigation)
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

fn focus_view_and_hide_shell(
    view_id: u64,
    command_sender: &CompositorCommandSender,
    home_view_request_channel: &Channel<HomeView>,
    hide_shell_channel: &Channel<()>,
) {
    let _ = command_sender.send(CompositorCommand::FocusView { view_id });
    let _ = home_view_request_channel.send(HomeView::Apps);
    let _ = hide_shell_channel.send(());
}

pub(super) fn overview_page_motion_config() -> SmoothFollowConfig {
    SmoothFollowConfig::new(
        Duration::from_millis(OVERVIEW_PAGE_MOTION_DURATION_MS),
        0.3,
        0.36,
    )
}
