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
                keyboard_interactivity: ShellKeyboardInteractivity::Exclusive,
                exclusive_zone: ShellExclusiveZone::None,
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
mod tests {
    use super::{ShellLaunchConfiguration, ShellLaunchConfigurationError};
    #[cfg(target_os = "linux")]
    use dailand::{ShellBackend, ShellLayer};
    use momo_app::ShellMode;

    fn args(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| (*value).to_string()).collect()
    }

    #[test]
    fn defaults_to_standalone() {
        let configuration = ShellLaunchConfiguration::from_args(args(&[])).unwrap();

        assert_eq!(configuration.mode, ShellMode::Standalone);
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn shell_flag_selects_shell_runner_options() {
        let configuration = ShellLaunchConfiguration::from_args(args(&["--shell"])).unwrap();
        let runner_options = configuration.shell_runner_options();

        assert_eq!(configuration.mode, ShellMode::Shell);
        assert_eq!(runner_options.backend, ShellBackend::WlrLayerShell);
        assert_eq!(runner_options.surface.layer, ShellLayer::Background);
    }

    #[test]
    #[cfg(not(target_os = "linux"))]
    fn shell_flag_is_rejected_on_non_linux_targets() {
        let error = ShellLaunchConfiguration::from_args(args(&["--shell"]))
            .expect_err("shell mode should be Linux-only");

        assert_eq!(error, ShellLaunchConfigurationError::ShellModeUnsupported);
    }

    #[test]
    fn unknown_argument_is_rejected() {
        let error = ShellLaunchConfiguration::from_args(args(&["--standalone"]))
            .expect_err("unknown argument should fail");

        assert_eq!(
            error,
            ShellLaunchConfigurationError::UnknownArgument("--standalone".to_string())
        );
    }
}
