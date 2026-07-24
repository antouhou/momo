mod style;

use self::style::{
    OVERVIEW_APPS_BUTTON_SIZE, overview_apps_button_band_style, overview_apps_button_style,
};
use crate::components::home::{
    model::tile_focus_transform,
    state::{HomeView, use_home_view_request_channel},
};
use daiko::{
    Element, Vec2,
    component::{Component, ComponentContext},
};
use momo_kit::{assets::GRID_HORIZONTAL_ICON, components::svg_icon, interaction::ButtonBehavior};

pub(super) struct OverviewAppsButton;

impl Component for OverviewAppsButton {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let home_view_request_channel = use_home_view_request_channel(ctx);
        let button = ButtonBehavior::new(ctx).apply();
        let button_transform = tile_focus_transform(
            Vec2::new(OVERVIEW_APPS_BUTTON_SIZE, OVERVIEW_APPS_BUTTON_SIZE),
            button.is_focus_visible,
            ctx,
        );

        if button.just_activated {
            let _ = home_view_request_channel.send(HomeView::Apps);
        }

        Element::new()
            .with_tag("overview-apps-button-band")
            .with_style(overview_apps_button_band_style())
            .with_content(
                Element::new()
                    .with_tag("overview-apps-button")
                    .with_style(overview_apps_button_style(
                        ctx,
                        &button_transform,
                        button.is_hovering,
                        button.is_focus_visible,
                    ))
                    .with_content(svg_icon(
                        GRID_HORIZONTAL_ICON,
                        style::OVERVIEW_APPS_BUTTON_ICON_SIZE,
                        style::overview_apps_button_icon_color(),
                    )),
            )
    }
}
