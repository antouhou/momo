use momo_app::ShellMode;
use momo_shell::ShellLaunchConfiguration;
#[cfg(not(debug_assertions))]
use momo_shell::create_ui;
#[cfg(debug_assertions)]
use {daiko::hot_reloading::HotReloadApp, std::path::PathBuf};

fn run_configured_app<T: daiko::App + Send + 'static>(
    app: T,
    launch_configuration: ShellLaunchConfiguration,
) {
    match launch_configuration.mode {
        ShellMode::Standalone => daiko::run(app),
        ShellMode::Shell => run_shell(app, launch_configuration),
    }
}

#[cfg(target_os = "linux")]
fn run_shell<T: daiko::App + Send + 'static>(
    app: T,
    launch_configuration: ShellLaunchConfiguration,
) {
    dailand::run(app, launch_configuration.shell_runner_options());
}

#[cfg(not(target_os = "linux"))]
fn run_shell<T: daiko::App + Send + 'static>(
    _app: T,
    _launch_configuration: ShellLaunchConfiguration,
) {
    unreachable!("shell mode is rejected during launch configuration parsing on non-Linux targets");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up basic logging
    momo_tracing::initialize_tracing("momo")?;
    let launch_configuration = ShellLaunchConfiguration::from_env()?;

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

        let app = HotReloadApp::new(app_crate_path).watch_path(momo_ui_lib_path);

        run_configured_app(app, launch_configuration);
    }
    #[cfg(not(debug_assertions))]
    {
        let ui = create_ui(launch_configuration)?;

        run_configured_app(ui, launch_configuration);
    }

    Ok(())
}
