use momo_app::{ShellApp, ShellConfiguration, ShellMode};
use momo_ui::{MomoUi};
use momo_wayfire::WayfireBackend;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up basic logging
    momo_ui::init_tracing();

    // Set a panic hook that exits the process with if any of the threads panic
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        original_hook(panic_info);
        std::process::exit(1);
    }));

    // App setup
    let configuration = ShellConfiguration {
        mode: ShellMode::Standalone,
    };

    let backend = WayfireBackend::disconnected();
    let app = ShellApp::new(configuration, backend);
    let ui = MomoUi::new(app.initial_view_model());

    daiko::run(ui);

    Ok(())
}
