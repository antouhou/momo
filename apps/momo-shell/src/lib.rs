use daiko::hot_reloading::DynApp;
use momo_app::{ShellApp, ShellConfiguration, ShellMode};
use momo_ui::MomoUi;
use momo_wayfire::WayfireBackend;
use system_control::SystemControl;

/// For the hot-reloading system to work, the function must have this exact name and signature.
/// The app needs to be wrapped in DynApp. For production builds, you can create the app directly in
/// main.rs and run it without hot-reloading.
#[unsafe(no_mangle)]
pub fn create_app() -> DynApp {
    // App setup
    let configuration = ShellConfiguration {
        mode: ShellMode::Standalone,
    };

    let backend = WayfireBackend::disconnected();
    let app = ShellApp::new(configuration, backend);
    let system_control =
        SystemControl::new().expect("failed to initialize system control services");
    let ui = MomoUi::new(app.initial_view_model(), system_control);

    DynApp::new(ui)
}
