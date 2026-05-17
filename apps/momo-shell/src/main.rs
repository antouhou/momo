use daiko::hot_reloading::HotReloadApp;
use log::info;
use momo_app::{ShellApp, ShellConfiguration, ShellMode};
use momo_ui::MomoUi;
use momo_wayfire::WayfireBackend;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up basic logging
    momo_ui::init_tracing();

    // Set a panic hook that exits the process with if any of the threads panic
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        original_hook(panic_info);
        std::process::exit(1);
    }));

    #[cfg(debug_assertions)]
    {
        // This is going to be a current crate path, but you can change it to any other path
        let app_crate_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let momo_ui_lib_path = app_crate_path
            .join("../../crates")
            .join("momo-ui")
            .canonicalize()?;

        let app = HotReloadApp::new(app_crate_path).watch_path(momo_ui_lib_path.clone());

        daiko::run(app)
    }
    #[cfg(not(debug_assertions))]
    {
        // App setup
        let configuration = ShellConfiguration {
            mode: ShellMode::Standalone,
        };

        let backend = WayfireBackend::disconnected();
        let app = ShellApp::new(configuration, backend);
        let ui = MomoUi::new(app.initial_view_model());

        daiko::run(ui);
    }

    Ok(())
}
