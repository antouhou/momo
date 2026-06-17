use appkeeper::app_provider::AppProvider;
use daiko::component::ComponentContext;
use daiko::state_management::StateHandle;
use daiko::style::Color;
use daiko::{AppContext, Id};
use std::path::PathBuf;
use std::sync::Arc;

const APPS_STATE_ID: &str = "momo_apps_state";

pub(crate) struct AppsState {
    pub app_entries: Vec<AppEntry>,
    pub is_loading: bool,
}

impl Default for AppsState {
    fn default() -> Self {
        Self {
            app_entries: vec![],
            is_loading: true,
        }
    }
}

pub(crate) fn init_app_state(ctx: &mut AppContext) {
    let state = ctx.peek_global_state(Id::new(APPS_STATE_ID), AppsState::default);
    std::thread::spawn(move || {
        let provider = appkeeper::app_provider();
        let entries = provider.list();

        {
            let mut guard = state.write();
            guard.is_loading = false;
            guard.app_entries = entries
                .into_iter()
                .map(|entry| {
                    AppEntry {
                        id: Arc::new(entry.id.clone()),
                        name: Arc::new(entry.name.clone()),
                        icon: Arc::new(entry.icon_for_size(256).map(|icon| icon.path.clone())),
                        // TODO
                        accent: Color::from_rgb(0, 125, 215),
                    }
                })
                .collect();
        }

        // let subscription_state = state.clone();
        // provider.subscribe(move |event| {
        //     let mut guard = subscription_state.write();
        // });
    });
}

pub(crate) fn apps_state(ctx: &mut ComponentContext) -> StateHandle<AppsState> {
    ctx.use_global_state(Id::new(APPS_STATE_ID), AppsState::default)
}

#[derive(Clone)]
pub(crate) struct AppEntry {
    pub id: Arc<String>,
    pub name: Arc<String>,
    pub icon: Arc<Option<PathBuf>>,
    pub accent: Color,
}

impl AppEntry {
    pub(crate) fn id(&self) -> &str {
        self.id.as_str()
    }
}
