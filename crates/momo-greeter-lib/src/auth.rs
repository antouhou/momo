use daiko::component::ComponentContext;
use daiko::state_management::StateHandle;
use daiko::{AppContext, Id};
use std::sync::Arc;
use std::sync::mpsc::{self, Sender};
use thiserror::Error;

const GREETER_AUTH_STATE_ID: &str = "momo_greeter_auth_state";
const DEFAULT_SESSION_COMMAND: &str = "wayfire";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GreeterAuthSource {
    System,
    Mock,
}

impl GreeterAuthSource {
    pub fn from_args(args: impl IntoIterator<Item = String>) -> Self {
        if args
            .into_iter()
            .any(|argument| argument == "--mock-users" || argument == "--mock-auth")
        {
            Self::Mock
        } else {
            Self::System
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GreeterAuthRequest {
    pub username: String,
    pub secret: String,
    pub session_command: Vec<String>,
    pub env: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GreeterAuthResult {
    Started,
    Failed(String),
}

pub trait GreeterAuthenticator: Send + Sync {
    fn authenticate(&self, request: &GreeterAuthRequest) -> GreeterAuthResult;
}

struct GreeterAuthWorkerRequest {
    request: GreeterAuthRequest,
    state: StateHandle<GreeterAuthState>,
}

#[derive(Clone)]
pub struct GreeterAuthHandle {
    request_sender: Sender<GreeterAuthWorkerRequest>,
}

impl GreeterAuthHandle {
    pub fn spawn<TAuthenticator>(authenticator: TAuthenticator) -> Self
    where
        TAuthenticator: GreeterAuthenticator + 'static,
    {
        let (request_sender, request_receiver) = mpsc::channel::<GreeterAuthWorkerRequest>();

        std::thread::spawn(move || {
            while let Ok(worker_request) = request_receiver.recv() {
                let GreeterAuthWorkerRequest { request, state } = worker_request;
                let result = authenticator.authenticate(&request);
                let username = Arc::new(request.username);
                let mut state_guard = state.write();
                state_guard.status = match result {
                    GreeterAuthResult::Started => GreeterAuthStatus::Started { username },
                    GreeterAuthResult::Failed(message) => GreeterAuthStatus::Failed {
                        username,
                        message: Arc::new(message),
                    },
                };
            }
        });

        Self { request_sender }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum GreeterAuthStatus {
    Idle,
    Authenticating {
        username: Arc<String>,
    },
    Started {
        username: Arc<String>,
    },
    Failed {
        username: Arc<String>,
        message: Arc<String>,
    },
}

#[derive(Clone)]
pub(crate) struct GreeterAuthState {
    pub(crate) status: GreeterAuthStatus,
    request_sender: Option<GreeterAuthHandle>,
    session_command: Vec<String>,
    env: Vec<String>,
}

impl Default for GreeterAuthState {
    fn default() -> Self {
        Self {
            status: GreeterAuthStatus::Idle,
            request_sender: None,
            session_command: default_session_command(),
            env: Vec::new(),
        }
    }
}

pub fn default_session_command() -> Vec<String> {
    vec![DEFAULT_SESSION_COMMAND.to_string()]
}

pub fn session_command_from_args(args: &[String]) -> Vec<String> {
    args.iter()
        .enumerate()
        .find_map(|(index, argument)| {
            argument
                .strip_prefix("--session-command=")
                .and_then(split_session_command)
                .or_else(|| {
                    if argument == "--session-command" {
                        let value = args.get(index + 1)?;
                        split_session_command(value)
                    } else {
                        None
                    }
                })
        })
        .unwrap_or_else(default_session_command)
}

pub fn init_greeter_auth_state(
    app_context: &mut AppContext,
    auth_handle: GreeterAuthHandle,
    session_command: Vec<String>,
) {
    let state =
        app_context.peek_global_state(Id::new(GREETER_AUTH_STATE_ID), GreeterAuthState::default);
    let mut state_guard = state.write();
    state_guard.status = GreeterAuthStatus::Idle;
    state_guard.request_sender = Some(auth_handle);
    state_guard.session_command = session_command;
    state_guard.env = Vec::new();
}

pub(crate) fn use_greeter_auth_state(ctx: &mut ComponentContext) -> StateHandle<GreeterAuthState> {
    ctx.use_global_state(Id::new(GREETER_AUTH_STATE_ID), GreeterAuthState::default)
}

pub(crate) fn submit_greeter_auth_request(
    state: &StateHandle<GreeterAuthState>,
    username: Arc<String>,
    secret: String,
) -> Result<(), GreeterAuthSubmitError> {
    if secret.is_empty() {
        return Err(GreeterAuthSubmitError::EmptySecret);
    }

    let (auth_handle, session_command, env) = {
        let mut state_guard = state.write();
        if matches!(state_guard.status, GreeterAuthStatus::Authenticating { .. }) {
            return Err(GreeterAuthSubmitError::AlreadyAuthenticating);
        }
        let auth_handle = state_guard
            .request_sender
            .clone()
            .ok_or(GreeterAuthSubmitError::BackendUnavailable)?;

        state_guard.status = GreeterAuthStatus::Authenticating {
            username: Arc::clone(&username),
        };
        (
            auth_handle,
            state_guard.session_command.clone(),
            state_guard.env.clone(),
        )
    };

    let request = GreeterAuthRequest {
        username: username.as_ref().clone(),
        secret,
        session_command,
        env,
    };

    if auth_handle
        .request_sender
        .send(GreeterAuthWorkerRequest {
            request,
            state: state.clone(),
        })
        .is_err()
    {
        state.write().status = GreeterAuthStatus::Failed {
            username,
            message: Arc::new("Authentication backend is unavailable".to_string()),
        };
        return Err(GreeterAuthSubmitError::BackendUnavailable);
    }

    Ok(())
}

#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub(crate) enum GreeterAuthSubmitError {
    #[error("Enter your password")]
    EmptySecret,
    #[error("Authentication is already in progress")]
    AlreadyAuthenticating,
    #[error("Authentication backend is unavailable")]
    BackendUnavailable,
}

#[derive(Clone)]
pub struct MockGreeterAuthenticator;

impl GreeterAuthenticator for MockGreeterAuthenticator {
    fn authenticate(&self, request: &GreeterAuthRequest) -> GreeterAuthResult {
        if request.secret.is_empty() {
            GreeterAuthResult::Failed("Enter your password".to_string())
        } else {
            GreeterAuthResult::Started
        }
    }
}

fn split_session_command(command: &str) -> Option<Vec<String>> {
    let parts = command
        .split_whitespace()
        .map(str::to_string)
        .collect::<Vec<_>>();

    if parts.is_empty() { None } else { Some(parts) }
}
