mod icon;
mod style;

use crate::app_state::use_apps_state;
use crate::components::home::app_tile::AppInfo;
use daiko::Element;
use daiko::component::{Component, ComponentContext};
use daiko::navigation::FocusKey;
use std::sync::Arc;

pub struct Dock {
    pub(crate) interactions_disabled: bool,
    pub(crate) hidden_app_id: Option<Arc<String>>,
    pub(crate) preferred_focus_key: Option<FocusKey>,
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

        Element::new()
            .with_style(style::dock_outer_container())
            .with_content(dock)
    }
}

fn dock_app_focus_key(app_id: &str) -> FocusKey {
    FocusKey::new(format!("dock-{app_id}"))
}
