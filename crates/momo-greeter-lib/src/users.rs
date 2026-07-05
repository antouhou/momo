use daiko::component::ComponentContext;
use daiko::state_management::StateHandle;
use daiko::{AppContext, Id};
use std::sync::Arc;
use system_control::{FeatureState, UserHandle};

pub(crate) const GREETER_USERS_STATE_ID: &str = "momo_greeter_users_state";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GreeterUserSource {
    System,
    Mock,
}

impl GreeterUserSource {
    pub fn from_args(args: impl IntoIterator<Item = String>) -> Self {
        if args.into_iter().any(|argument| argument == "--mock-users") {
            Self::Mock
        } else {
            Self::System
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GreeterUser {
    pub username: Arc<String>,
    pub display_name: Arc<String>,
    pub initials: Arc<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum GreeterUsersStatus {
    Loading,
    Ready(Vec<GreeterUser>),
    Empty,
    Unavailable(Arc<String>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct GreeterUsersState {
    pub(crate) status: GreeterUsersStatus,
}

impl Default for GreeterUsersState {
    fn default() -> Self {
        Self {
            status: GreeterUsersStatus::Loading,
        }
    }
}

impl GreeterUser {
    pub fn new(username: impl Into<String>, display_name: impl Into<String>) -> Self {
        let username = username.into();
        let display_name = display_name.into();
        let initials = user_initials(&display_name);

        Self {
            username: Arc::new(username),
            display_name: Arc::new(display_name),
            initials: Arc::new(initials),
        }
    }
}

pub fn mock_users() -> Vec<GreeterUser> {
    vec![
        GreeterUser::new("anton", "Anton"),
        GreeterUser::new("maya", "Maya"),
        GreeterUser::new("guest", "Guest"),
    ]
}

pub(crate) fn init_greeter_users_state(
    app_context: &mut AppContext,
    user_handle: UserHandle,
    source: GreeterUserSource,
) {
    let state =
        app_context.peek_global_state(Id::new(GREETER_USERS_STATE_ID), GreeterUsersState::default);

    std::thread::spawn(move || {
        let status = load_greeter_users(user_handle, source);
        state.write().status = status;
    });
}

pub(crate) fn use_greeter_users_state(
    ctx: &mut ComponentContext,
) -> StateHandle<GreeterUsersState> {
    ctx.use_global_state(Id::new(GREETER_USERS_STATE_ID), GreeterUsersState::default)
}

fn load_greeter_users(user_handle: UserHandle, source: GreeterUserSource) -> GreeterUsersStatus {
    match source {
        GreeterUserSource::Mock => GreeterUsersStatus::Ready(mock_users()),
        GreeterUserSource::System => system_greeter_users(user_handle),
    }
}

fn system_greeter_users(user_handle: UserHandle) -> GreeterUsersStatus {
    match user_handle.list_users() {
        FeatureState::Ready(users) if !users.is_empty() => GreeterUsersStatus::Ready(
            users
                .into_iter()
                .map(|user| GreeterUser::new(user.username, user.display_name))
                .collect(),
        ),
        FeatureState::Ready(_) => GreeterUsersStatus::Empty,
        FeatureState::Loading => GreeterUsersStatus::Loading,
        FeatureState::Unsupported(reason) => GreeterUsersStatus::Unavailable(Arc::new(format!(
            "User listing is unsupported: {reason:?}"
        ))),
        FeatureState::Unavailable(reason) => GreeterUsersStatus::Unavailable(Arc::new(format!(
            "User listing is unavailable: {reason}"
        ))),
    }
}

fn user_initials(display_name: &str) -> String {
    let initials = display_name
        .split_whitespace()
        .filter_map(|part| part.chars().next())
        .take(2)
        .flat_map(char::to_uppercase)
        .collect::<String>();

    if initials.is_empty() {
        "?".to_string()
    } else {
        initials
    }
}
