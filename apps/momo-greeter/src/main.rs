#[cfg(target_os = "linux")]
use momo_app::greeter_shell_runner_options;
#[cfg(not(debug_assertions))]
use momo_greeter::create_greeter;
use momo_greeter::{GreeterLaunchConfiguration, GreeterMode};
#[cfg(debug_assertions)]
use {daiko::hot_reloading::HotReloadApp, std::path::PathBuf};

#[cfg(target_os = "linux")]
fn run_greeter<T: daiko::App + Send + 'static>(
    app: T,
    launch_mode: GreeterMode,
) -> Result<(), Box<dyn std::error::Error>> {
    match launch_mode {
        GreeterMode::Shell => {
            dailand::run(app, greeter_shell_runner_options("momo-greeter"))?;
            Ok(())
        }
        GreeterMode::Standalone => {
            daiko::run(app);
            Ok(())
        }
    }
}

#[cfg(not(target_os = "linux"))]
fn run_greeter<T: daiko::App + Send + 'static>(
    app: T,
    _launch_mode: GreeterMode,
) -> Result<(), Box<dyn std::error::Error>> {
    daiko::run(app);
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up basic logging
    momo_tracing::initialize_tracing("momo-greeter")?;
    let launch_configuration = GreeterLaunchConfiguration::from_env();
    let launch_mode = launch_configuration.mode;

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

        let app = HotReloadApp::new(app_crate_path).watch_path(momo_ui_lib_path);

        run_greeter(app, launch_mode)?;
    }
    #[cfg(not(debug_assertions))]
    {
        let ui = create_greeter(launch_configuration.into_greeter_arguments(), launch_mode)?;
        run_greeter(ui, launch_mode)?;
    }

    Ok(())
}
