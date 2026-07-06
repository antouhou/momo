use daiko::hot_reloading::DynApp;
use momo_greetd::{GreetdAuthRequest, GreetdAuthenticator};
use momo_greeter_lib::{
    GreeterAuthHandle, GreeterAuthRequest, GreeterAuthResult, GreeterAuthSource,
    GreeterAuthenticator, GreeterUserSource, MockGreeterAuthenticator, MomoGreeter,
    session_command_from_args,
};
use system_control::SystemControl;

#[derive(Clone)]
enum AppGreeterAuthenticator {
    System(GreetdAuthenticator),
    Mock(MockGreeterAuthenticator),
}

impl GreeterAuthenticator for AppGreeterAuthenticator {
    fn authenticate(&self, request: &GreeterAuthRequest) -> GreeterAuthResult {
        match self {
            Self::System(authenticator) => authenticate_with_greetd(authenticator, request),
            Self::Mock(authenticator) => authenticator.authenticate(request),
        }
    }
}

fn authenticate_with_greetd(
    authenticator: &GreetdAuthenticator,
    request: &GreeterAuthRequest,
) -> GreeterAuthResult {
    let request = GreetdAuthRequest {
        username: request.username.clone(),
        secret: request.secret.clone(),
        session_command: request.session_command.clone(),
        env: request.env.clone(),
    };

    match authenticator.authenticate(request) {
        Ok(()) => GreeterAuthResult::Started,
        Err(error) => {
            log::warn!("greetd authentication failed: {error}");
            GreeterAuthResult::Failed(error.user_message())
        }
    }
}

pub fn create_greeter(args: impl IntoIterator<Item = String>) -> MomoGreeter {
    let args = args.into_iter().collect::<Vec<_>>();

    let system_control =
        SystemControl::new().expect("failed to initialize system control services");
    let user_source = GreeterUserSource::from_args(args.iter().cloned());
    let auth_source = GreeterAuthSource::from_args(args.iter().cloned());
    let session_command = session_command_from_args(&args);
    let authenticator = authenticator_for_source(auth_source);
    let auth_handle = GreeterAuthHandle::spawn(authenticator);

    MomoGreeter::new(system_control, user_source, auth_handle, session_command)
}

fn authenticator_for_source(source: GreeterAuthSource) -> AppGreeterAuthenticator {
    match source {
        GreeterAuthSource::System => {
            AppGreeterAuthenticator::System(GreetdAuthenticator::from_environment())
        }
        GreeterAuthSource::Mock => AppGreeterAuthenticator::Mock(MockGreeterAuthenticator),
    }
}

/// For the hot-reloading system to work, the function must have this exact name and signature.
/// The app needs to be wrapped in DynApp. For production builds, you can create the app directly in
/// main.rs and run it without hot-reloading.
#[unsafe(no_mangle)]
pub fn create_app() -> DynApp {
    DynApp::new(create_greeter(std::env::args().skip(1)))
}
