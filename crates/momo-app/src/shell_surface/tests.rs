use super::{desktop_shell_runner_options, greeter_shell_runner_options};
use dailand::{
    ShellBackend, ShellInputRegion, ShellKeyboardInteractivity, ShellLayer, ShellOutputTarget,
};

#[test]
fn creates_an_on_demand_desktop_shell_surface() {
    let runner_options = desktop_shell_runner_options("momo-shell");

    assert_eq!(runner_options.backend, ShellBackend::WlrLayerShell);
    assert_eq!(runner_options.surface.namespace, "momo-shell");
    assert_eq!(runner_options.surface.layer, ShellLayer::Top);
    assert_eq!(
        runner_options.surface.keyboard_interactivity,
        ShellKeyboardInteractivity::OnDemand
    );
    assert_eq!(
        runner_options.surface.output_target,
        ShellOutputTarget::CompositorDefault
    );
    assert_eq!(
        runner_options.surface.input_region,
        ShellInputRegion::Unspecified
    );
}
