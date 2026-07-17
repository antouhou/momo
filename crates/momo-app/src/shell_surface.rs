use dailand::{
    ShellAnchors, ShellBackend, ShellExclusiveZone, ShellInputRegion, ShellKeyboardInteractivity,
    ShellLayer, ShellMargins, ShellOutputTarget, ShellRunnerOptions, ShellSurfaceOptions,
    ShellSurfaceSize,
};

#[cfg(test)]
mod tests;

/// Standard top-layer shell with on demand keyboard access
pub fn desktop_shell_runner_options(namespace: impl Into<String>) -> ShellRunnerOptions {
    shell_runner_options(namespace, ShellKeyboardInteractivity::OnDemand)
}

/// Options for fullscreen greeter with exclusive keyboard access
pub fn greeter_shell_runner_options(namespace: impl Into<String>) -> ShellRunnerOptions {
    shell_runner_options(namespace, ShellKeyboardInteractivity::Exclusive)
}

fn shell_runner_options(
    namespace: impl Into<String>,
    keyboard_interactivity: ShellKeyboardInteractivity,
) -> ShellRunnerOptions {
    ShellRunnerOptions {
        backend: ShellBackend::WlrLayerShell,
        surface: ShellSurfaceOptions {
            namespace: namespace.into(),
            layer: ShellLayer::Top,
            size: ShellSurfaceSize::default(),
            anchors: ShellAnchors::all(),
            margins: ShellMargins::default(),
            output_target: ShellOutputTarget::CompositorDefault,
            keyboard_interactivity,
            exclusive_zone: ShellExclusiveZone::None,
            input_region: ShellInputRegion::Unspecified,
        },
    }
}
