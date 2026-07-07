mod app_state;
mod components;

use std::sync::Once;
use daiko::{App, AppContext};
use momo_app::ShellViewModel;
use system_control::SystemControl;
use tracing_subscriber::EnvFilter;
#[cfg(target_os = "android")]
use tracing_subscriber::layer::SubscriberExt;
#[cfg(target_os = "android")]
use tracing_subscriber::util::SubscriberInitExt;
#[cfg(feature = "bench-support")]
pub use crate::components::home::benchmark_support;
use crate::{
    app_state::init_app_state,
    components::home::{
        Home, bluetooth::initialize_bluetooth_state, power::initialize_power_state,
        session::initialize_session_state, system_status::initialize_system_status_state,
    },
};

static INIT: Once = Once::new();

pub fn init_tracing() {
    INIT.call_once(|| {
        let filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
        #[cfg(target_os = "android")]
        {
            let android_layer =
                tracing_android::layer("momo").expect("failed to initialize Android log layer");

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

pub struct MomoUi {
    view_model: ShellViewModel,
    system_control: SystemControl,
}

impl MomoUi {
    pub fn new(view_model: ShellViewModel, system_control: SystemControl) -> Self {
        Self {
            view_model,
            system_control,
        }
    }

    pub fn view_model(&self) -> &ShellViewModel {
        &self.view_model
    }
}

impl App for MomoUi {
    type RootComponent = Home;

    fn create(&mut self, app_context: &mut AppContext) -> Self::RootComponent {
        app_context.set_vsync_enabled(true);
        app_context.set_fullscreen(true);
        initialize_bluetooth_state(app_context, self.system_control.bluetooth());
        initialize_power_state(app_context, self.system_control.power());
        initialize_session_state(app_context, self.system_control.session());
        initialize_system_status_state(
            app_context,
            self.system_control.volume(),
            self.system_control.battery(),
        );
        init_app_state(app_context);
        Home::new()
    }

    fn stop(&mut self, _app_context: &mut AppContext) {
        println!("Stopping MomoUi");
    }
}
