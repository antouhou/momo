mod auth;
mod components;
mod users;

use crate::{
    auth::{auth_handle_for_source, init_greeter_auth_state},
    components::login_screen::LoginScreen,
    users::init_greeter_users_state,
};
pub use auth::{
    GreeterAuthHandle, GreeterAuthRequest, GreeterAuthResult, GreeterAuthSource,
    GreeterAuthenticator, MockGreeterAuthenticator, default_session_command,
    session_command_from_args,
};
use daiko::{App, AppContext};
use std::sync::Once;
use system_control::{SystemControl, SystemControlError};
use tracing_subscriber::EnvFilter;
#[cfg(target_os = "android")]
use tracing_subscriber::layer::SubscriberExt;
#[cfg(target_os = "android")]
use tracing_subscriber::util::SubscriberInitExt;
pub use users::{GreeterUser, GreeterUserSource, mock_users};

static INIT: Once = Once::new();

pub fn init_tracing() {
    INIT.call_once(|| {
        let filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
        #[cfg(target_os = "android")]
        {
            let android_layer =
                tracing_android::layer("momo").expect("failed to initialize Android log layer");

            tracing_subscriber::registry()
                .with(EnvFilter::new(filter))
                .with(android_layer)
                .init();
        }

        #[cfg(not(target_os = "android"))]
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::new(filter))
            .with_writer(std::io::stdout)
            .init();
    });
}

pub struct MomoGreeter {
    system_control: SystemControl,
    user_source: GreeterUserSource,
    auth_handle: GreeterAuthHandle,
    session_command: Vec<String>,
}

pub fn create_greeter(
    args: impl IntoIterator<Item = String>,
) -> Result<MomoGreeter, SystemControlError> {
    let args = args.into_iter().collect::<Vec<_>>();

    let system_control = SystemControl::new()?;
    let user_source = GreeterUserSource::from_args(args.iter().cloned());
    let auth_source = GreeterAuthSource::from_args(args.iter().cloned());
    let auth_handle = auth_handle_for_source(auth_source);
    let session_command = auth::session_command_from_args(&args);

    Ok(MomoGreeter::new(
        system_control,
        user_source,
        auth_handle,
        session_command,
    ))
}

impl MomoGreeter {
    pub fn new(
        system_control: SystemControl,
        user_source: GreeterUserSource,
        auth_handle: GreeterAuthHandle,
        session_command: Vec<String>,
    ) -> Self {
        Self {
            system_control,
            user_source,
            auth_handle,
            session_command,
        }
    }
}

impl App for MomoGreeter {
    type RootComponent = LoginScreen;

    fn create(&mut self, app_context: &mut AppContext) -> Self::RootComponent {
        app_context.set_vsync_enabled(true);
        app_context.set_fullscreen(true);
        init_greeter_users_state(app_context, self.system_control.users(), self.user_source);
        init_greeter_auth_state(
            app_context,
            self.auth_handle.clone(),
            self.session_command.clone(),
        );
        LoginScreen::new()
    }

    fn stop(&mut self, _app_context: &mut AppContext) {
        tracing::debug!("stopping MomoGreeter");
    }
}
