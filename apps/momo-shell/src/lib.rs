mod configuration;

pub use configuration::{ShellLaunchConfiguration, ShellLaunchConfigurationError};
use daiko::hot_reloading::DynApp;
use momo_app::{ShellApp, ShellConfiguration, ShellMode};
use momo_ui::MomoUi;
use momo_wayfire::WayfireBackend;
use system_control::SystemControl;

pub fn create_ui(
    launch_configuration: ShellLaunchConfiguration,
) -> Result<MomoUi, Box<dyn std::error::Error>> {
    create_ui_with_wayfire_backend(launch_configuration.mode)
}

fn create_ui_with_wayfire_backend(mode: ShellMode) -> Result<MomoUi, Box<dyn std::error::Error>> {
    let configuration = ShellConfiguration { mode };

    let backend = WayfireBackend::default();
    let app = ShellApp::new(configuration, backend);
    let started_app = app.start()?;

    let system_control = SystemControl::new()?;
    Ok(MomoUi::new(
        started_app.view_model,
        system_control,
        started_app.compositor_session,
    ))
}

/// For the hot-reloading system to work, the function must have this exact name and signature.
/// The app needs to be wrapped in DynApp. For production builds, you can create the app directly in
/// main.rs and run it without hot-reloading.
#[unsafe(no_mangle)]
pub fn create_app() -> DynApp {
    let launch_configuration = ShellLaunchConfiguration::from_env()
        .expect("failed to parse momo-shell launch configuration");
    let ui = create_ui(launch_configuration).expect("failed to create momo-shell UI");

    DynApp::new(ui)
}
