#[cfg(debug_assertions)]
use {daiko::hot_reloading::HotReloadApp, std::path::PathBuf};
#[cfg(not(debug_assertions))]
use {
    momo_greeter::{ShellApp, ShellConfiguration, ShellMode},
    momo_greeter_lib::{GreeterUserSource, MomoGreeter},
    momo_wayfire::WayfireBackend,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up basic logging
    momo_greeter_lib::init_tracing();

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
            .join("momo-greeter-lib")
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
        let system_control = system_control::SystemControl::new()
            .expect("failed to initialize system control services");
        let user_source = GreeterUserSource::from_args(std::env::args().skip(1));
        let ui = MomoGreeter::new(app.initial_view_model(), system_control, user_source);

        daiko::run(ui);
    }

    Ok(())
}
