mod configuration;

use daiko::hot_reloading::DynApp;
pub use momo_greeter_lib::{GreeterMode, create_greeter};

pub use configuration::GreeterLaunchConfiguration;

/// For the hot-reloading system to work, the function must have this exact name and signature.
/// The app needs to be wrapped in DynApp. For production builds, you can create the app directly in
/// main.rs and run it without hot-reloading.
#[unsafe(no_mangle)]
pub fn create_app() -> DynApp {
    let launch_configuration = GreeterLaunchConfiguration::from_env();
    let mode = launch_configuration.mode;
    let greeter = create_greeter(launch_configuration.into_greeter_arguments(), mode)
        .expect("failed to initialize system control services");
    DynApp::new(greeter)
}
