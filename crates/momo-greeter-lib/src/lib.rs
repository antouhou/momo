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
use system_control::{SystemControl, SystemControlError};
pub use users::{GreeterUser, GreeterUserSource, mock_users};

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
