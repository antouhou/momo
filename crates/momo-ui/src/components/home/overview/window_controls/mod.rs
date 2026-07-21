mod style;

use super::{OverviewCardFrame, overview_page_motion_config};
use daiko::{
    Element, Id, Vec2,
    component::{Component, ComponentContext},
    widgets::text::Text,
};
use momo_compositor::{CompositorCommand, CompositorCommandSender};
use momo_kit::{
    assets::XMARK_ICON,
    components::{RoundIconButton, RoundIconButtonVariant},
};
use std::sync::Arc;
use style::{window_controls_style, window_title_style};

const OVERVIEW_WINDOW_CONTROLS_POSITION_MOTION_ID: &str =
    "momo_home_overview_window_controls_position_motion";

pub(super) struct OverviewWindowControls {
    pub(super) view_id: u64,
    pub(super) window_title: Arc<String>,
    pub(super) command_sender: Option<CompositorCommandSender>,
    pub(super) active_card_frame: OverviewCardFrame,
}

impl Component for OverviewWindowControls {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let target_position = style::window_controls_target_position(self.active_card_frame);
        let rendered_position = {
            let mut position_motion = ctx.smooth_follow_with_id::<Vec2>(
                Id::new(OVERVIEW_WINDOW_CONTROLS_POSITION_MOTION_ID),
                overview_page_motion_config(),
            );
            position_motion.follow(target_position)
        };
        let mut close_button = RoundIconButton::new(ctx, XMARK_ICON)
            .with_tag("overview-window-close")
            .with_variant(RoundIconButtonVariant::Danger);

        if close_button.has_been_activated()
            && let Some(command_sender) = &self.command_sender
        {
            let _ = command_sender.send(CompositorCommand::CloseView {
                view_id: self.view_id,
            });
        }

        Element::new()
            .with_tag("overview-window-controls")
            .with_style(window_controls_style(
                rendered_position,
                self.active_card_frame.size.x,
            ))
            .with_content(
                Text::new(Arc::clone(&self.window_title)).with_style(window_title_style()),
            )
            .with_content(close_button)
    }
}
