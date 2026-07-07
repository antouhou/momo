#[cfg(not(debug_assertions))]
use momo_greeter::create_greeter;
use momo_greeter_lib::init_tracing;
#[cfg(debug_assertions)]
use {daiko::hot_reloading::HotReloadApp, std::path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up basic logging
    init_tracing();

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

        daiko::run(app)
    }
    #[cfg(not(debug_assertions))]
    {
        let ui = create_greeter(std::env::args().skip(1))?;
        daiko::run(ui);
    }

    Ok(())
}
