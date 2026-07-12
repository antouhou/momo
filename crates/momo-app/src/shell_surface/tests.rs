use super::shell_runner_options;
use dailand::{
    ShellBackend, ShellInputRegion, ShellKeyboardInteractivity, ShellLayer, ShellOutputTarget,
};

#[test]
fn creates_a_full_output_surface_for_the_requested_namespace() {
    let runner_options = shell_runner_options("momo-greeter");

    assert_eq!(runner_options.backend, ShellBackend::WlrLayerShell);
    assert_eq!(runner_options.surface.namespace, "momo-greeter");
    assert_eq!(runner_options.surface.layer, ShellLayer::Background);
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
    assert!(runner_options.surface.request_initial_keyboard_focus);
}
