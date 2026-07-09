#[cfg(target_os = "android")]
use momo_app::{ShellApp, ShellConfiguration, ShellMode};
#[cfg(target_os = "android")]
use momo_ui::MomoUi;
#[cfg(target_os = "android")]
use momo_wayfire::WayfireBackend;
#[cfg(target_os = "android")]
use winit::platform::android::activity::AndroidApp;

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
pub fn android_main(android_app: AndroidApp) {
    momo_tracing::initialize_tracing("momo").expect("failed to initialize tracing");

    let configuration = ShellConfiguration {
        mode: ShellMode::Standalone,
    };

    let backend = WayfireBackend::disconnected();
    let app = ShellApp::new(configuration, backend);
    let system_control =
        system_control::SystemControl::new().expect("failed to initialize system control services");
    let app = MomoUi::new(app.initial_view_model(), system_control);
    daiko::run_android(app, android_app);
}
