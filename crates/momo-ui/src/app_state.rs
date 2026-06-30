use appkeeper::app_launcher::{AppLauncher, LaunchError, LaunchOptions};
use appkeeper::app_provider::AppProvider;
use daiko::component::ComponentContext;
use daiko::state_management::StateHandle;
use daiko::style::Color;
use daiko::{AppContext, Id};
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::mpsc::Sender;
use tracing::error;

pub(crate) const APPS_STATE_ID: &str = "momo_apps_state";

pub enum AppCommand {
    LaunchApp(String),
}

pub enum AppOpResult {
    LaunchSpawned(String),
    LaunchFailed(String, LaunchError),
}

impl AppOpResult {
    pub fn is_for_app(&self, app_id: &str) -> bool {
        match self {
            AppOpResult::LaunchSpawned(id) | AppOpResult::LaunchFailed(id, _) => id == app_id,
        }
    }
}

pub(crate) struct AppsState {
    pub app_entries: Vec<AppEntry>,
    pub is_loading: bool,
    pub command_sender: Option<Sender<AppCommand>>,
    pub app_ops_results: Vec<AppOpResult>,
}

impl Default for AppsState {
    fn default() -> Self {
        Self {
            app_entries: vec![],
            is_loading: true,
            command_sender: None,
            app_ops_results: vec![],
        }
    }
}

pub(crate) fn init_app_state(ctx: &mut AppContext) {
    let state = ctx.peek_global_state(Id::new(APPS_STATE_ID), AppsState::default);

    std::thread::spawn(move || {
        let provider = appkeeper::app_provider();
        let entries = provider.list();
        let launcher = appkeeper::app_launcher();
        let (sender, receiver) = std::sync::mpsc::channel::<AppCommand>();
        {
            let mut guard = state.write();
            guard.command_sender = Some(sender);
        }

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

        while let Ok(cmd) = receiver.recv() {
            match cmd {
                AppCommand::LaunchApp(id) => {
                    // TODO: this is stupid, refactor this
                    if let Some(entry) = provider.entry(id.clone()) {
                        // TODO: do something with options
                        let launch_result = launcher.launch(
                            &entry,
                            LaunchOptions {
                                files: vec![],
                                urls: vec![],
                            },
                        );
                        match launch_result {
                            Ok(()) => {
                                state
                                    .write()
                                    .app_ops_results
                                    .push(AppOpResult::LaunchSpawned(id.clone()));
                            }
                            Err(err) => {
                                state
                                    .write()
                                    .app_ops_results
                                    .push(AppOpResult::LaunchFailed(id.clone(), err));
                            }
                        }
                    } else {
                        error!("Can't find app {} in the list for launch", id);
                    }
                }
            }
        }
    });
}

pub(crate) fn use_apps_state(ctx: &mut ComponentContext) -> StateHandle<AppsState> {
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
