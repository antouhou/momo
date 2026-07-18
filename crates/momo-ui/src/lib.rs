mod app_state;
mod components;

#[cfg(feature = "bench-support")]
pub use crate::components::home::benchmark_support;
use crate::{
    app_state::init_app_state,
    components::home::{
        Home, bluetooth::initialize_bluetooth_state, compositor::initialize_compositor_actions,
        power::initialize_power_state, session::initialize_session_state,
        system_status::initialize_system_status_state,
    },
};
use daiko::{App, AppContext};
use momo_app::{CompositorRuntime, ShellMode, ShellViewModel};
use system_control::SystemControl;

pub struct MomoUi {
    view_model: ShellViewModel,
    system_control: SystemControl,
    compositor_runtime: Option<CompositorRuntime>,
}

impl MomoUi {
    pub fn new(
        view_model: ShellViewModel,
        system_control: SystemControl,
        compositor_runtime: Option<CompositorRuntime>,
    ) -> Self {
        Self {
            view_model,
            system_control,
            compositor_runtime,
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
        match self.view_model.mode {
            ShellMode::Standalone => {
                // TODO: make fullscreen
            }
            ShellMode::Shell => {}
        }
        initialize_bluetooth_state(app_context, self.system_control.bluetooth());
        initialize_power_state(app_context, self.system_control.power());
        initialize_session_state(app_context, self.system_control.session());
        initialize_system_status_state(
            app_context,
            self.system_control.volume(),
            self.system_control.battery(),
        );
        let compositor_event_receiver = self
            .compositor_runtime
            .as_mut()
            .and_then(CompositorRuntime::take_event_receiver);
        initialize_compositor_actions(app_context, compositor_event_receiver);
        init_app_state(app_context);
        Home::new()
    }

    fn stop(&mut self, _app_context: &mut AppContext) {
        if let Some(compositor_runtime) = self.compositor_runtime.as_mut() {
            compositor_runtime.stop();
        }
    }
}
