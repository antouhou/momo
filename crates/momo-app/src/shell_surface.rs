use dailand::{
    ShellAnchors, ShellBackend, ShellExclusiveZone, ShellInputRegion, ShellKeyboardInteractivity,
    ShellLayer, ShellMargins, ShellOutputTarget, ShellRunnerOptions, ShellSurfaceOptions,
    ShellSurfaceSize,
};

#[cfg(test)]
mod tests;

/// Builds the standard full-output layer-shell configuration for Momo system surfaces.
pub fn shell_runner_options(namespace: impl Into<String>) -> ShellRunnerOptions {
    ShellRunnerOptions {
        backend: ShellBackend::WlrLayerShell,
        surface: ShellSurfaceOptions {
            namespace: namespace.into(),
            layer: ShellLayer::Background,
            size: ShellSurfaceSize::default(),
            anchors: ShellAnchors::all(),
            margins: ShellMargins::default(),
            output_target: ShellOutputTarget::CompositorDefault,
            keyboard_interactivity: ShellKeyboardInteractivity::OnDemand,
            exclusive_zone: ShellExclusiveZone::None,
            input_region: ShellInputRegion::Unspecified,
            request_initial_keyboard_focus: true,
        },
    }
}
