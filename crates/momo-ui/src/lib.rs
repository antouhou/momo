mod components;

use daiko::{App, AppContext};
use momo_app::ShellViewModel;
use crate::components::home::Home;

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

    fn create(&mut self, app_context: &mut AppContext) -> Self::RootComponent {
        Home
    }

    fn stop(&mut self, app_context: &mut AppContext) {
        println!("Stopping MomoUi");
    }
}
