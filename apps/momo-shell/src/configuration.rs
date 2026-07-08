#[cfg(target_os = "linux")]
use dailand::{
    ShellAnchors, ShellBackend, ShellExclusiveZone, ShellKeyboardInteractivity, ShellLayer,
    ShellRunnerOptions, ShellSurfaceOptions,
};
use momo_app::ShellMode;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShellLaunchConfiguration {
    pub mode: ShellMode,
}

impl Default for ShellLaunchConfiguration {
    fn default() -> Self {
        Self {
            mode: ShellMode::Standalone,
        }
    }
}

impl ShellLaunchConfiguration {
    pub fn from_env() -> Result<Self, ShellLaunchConfigurationError> {
        Self::from_args(std::env::args().skip(1))
    }

    pub fn from_args(
        args: impl IntoIterator<Item = String>,
    ) -> Result<Self, ShellLaunchConfigurationError> {
        let mut configuration = Self::default();

        for arg in args {
            match arg.as_str() {
                "--shell" => {
                    #[cfg(target_os = "linux")]
                    {
                        configuration.mode = ShellMode::Shell;
                    }
                    #[cfg(not(target_os = "linux"))]
                    {
                        return Err(ShellLaunchConfigurationError::ShellModeUnsupported);
                    }
                }
                _ => {
                    return Err(ShellLaunchConfigurationError::UnknownArgument(arg));
                }
            }
        }

        Ok(configuration)
    }

    #[cfg(target_os = "linux")]
    pub fn shell_runner_options(self) -> ShellRunnerOptions {
        ShellRunnerOptions {
            backend: ShellBackend::WlrLayerShell,
            surface: ShellSurfaceOptions {
                namespace: "momo-shell".to_string(),
                layer: ShellLayer::Background,
                anchors: ShellAnchors::all(),
                keyboard_interactivity: ShellKeyboardInteractivity::OnDemand,
                exclusive_zone: ShellExclusiveZone::None,
                request_initial_keyboard_focus: true,
            },
        }
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ShellLaunchConfigurationError {
    #[error("unknown argument: {0}")]
    UnknownArgument(String),
    #[error("shell mode is only supported on Linux")]
    ShellModeUnsupported,
}

#[cfg(test)]
mod tests;
