mod components;

use std::sync::Once;
use crate::components::home::Home;
use daiko::{App, AppContext};
use tracing_subscriber::EnvFilter;
use momo_app::ShellViewModel;

static INIT: Once = Once::new();

pub fn init_tracing() {
    INIT.call_once(|| {
        let filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
        #[cfg(target_os = "android")]
        {
            use tracing_subscriber::layer::SubscriberExt;
            use tracing_subscriber::util::SubscriberInitExt;
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
}

impl MomoUi {
    pub fn new(view_model: ShellViewModel) -> Self {
        Self { view_model }
    }

    pub fn view_model(&self) -> &ShellViewModel {
        &self.view_model
    }
}

impl App for MomoUi {
    type RootComponent = Home;

    fn create(&mut self, _app_context: &mut AppContext) -> Self::RootComponent {
        Home::new()
    }

    fn stop(&mut self, _app_context: &mut AppContext) {
        println!("Stopping MomoUi");
    }
}
