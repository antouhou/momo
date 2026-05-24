use daiko::hot_reloading::DynApp;
use momo_app::{ShellApp, ShellConfiguration, ShellMode};
use momo_ui::MomoUi;
use momo_wayfire::WayfireBackend;
use std::sync::Once;
use system_control::SystemControl;
use tracing_subscriber::EnvFilter;

static INIT: Once = Once::new();

pub fn init_tracing() {
    INIT.call_once(|| {
        let filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
        #[cfg(target_os = "android")]
        {
            use tracing_subscriber::layer::SubscriberExt;
            use tracing_subscriber::util::SubscriberInitExt;
            let android_layer =
                tracing_android::layer("deko").expect("failed to initialize Android log layer");

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
