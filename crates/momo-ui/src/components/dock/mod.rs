mod icon;
mod overview_button;
mod style;

use crate::{app_state::use_apps_state, components::home::app_tile::AppInfo};
use daiko::{
    Element,
    channel::Channel,
    component::{Component, ComponentContext},
    navigation::FocusKey,
};
use std::sync::Arc;

pub struct Dock {
    pub(crate) interactions_disabled: bool,
    pub(crate) hidden_app_id: Option<Arc<String>>,
    pub(crate) preferred_focus_key: Option<FocusKey>,
    pub(crate) overview_toggle_channel: Channel<()>,
    pub(crate) overview_is_active: bool,
}

impl Component for Dock {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let apps_handle = use_apps_state(ctx);
        let apps = apps_handle.read();
        let docked_apps = apps.app_entries.iter().take(3);

        ctx.focus_scope();

        let mut dock = Element::new().with_style(style::dock_style());

        for app in docked_apps {
            let focus_key = dock_app_focus_key(app.id());
            let preferred_focus = self.preferred_focus_key.as_ref() == Some(&focus_key);
            dock.add_content(icon::DockIcon {
                app: AppInfo::new(app),
                focus_key,
                preferred_focus,
                interactions_disabled: self.interactions_disabled,
                is_hidden_for_launch: self.hidden_app_id.as_deref().map(String::as_str)
                    == Some(app.id()),
            });
        }

        dock.add_content(overview_button::OverviewDockButton {
            activation_channel: self.overview_toggle_channel.clone(),
            interactions_disabled: self.interactions_disabled,
            is_active: self.overview_is_active,
        });

        Element::new()
            .with_style(style::dock_outer_container())
            .with_content(dock)
    }
}

fn dock_app_focus_key(app_id: &str) -> FocusKey {
    FocusKey::new(format!("dock-{app_id}"))
}
