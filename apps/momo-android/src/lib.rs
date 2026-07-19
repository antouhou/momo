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

    let backend = WayfireBackend::default();
    let started_app = ShellApp::new(configuration, backend)
        .start()
        .expect("failed to start shell application services");
    let system_control =
        system_control::SystemControl::new().expect("failed to initialize system control services");
    let app = MomoUi::new(
        started_app.view_model,
        system_control,
        started_app.compositor_session,
    );
    daiko::run_android(app, android_app);
}
