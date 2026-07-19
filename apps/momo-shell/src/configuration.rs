#[cfg(target_os = "linux")]
use dailand::ShellRunnerOptions;
use momo_app::ShellMode;
#[cfg(target_os = "linux")]
use momo_app::desktop_shell_runner_options;
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
        let mode = args
            .into_iter()
            .try_fold(ShellMode::Standalone, |_mode, argument| {
                match argument.as_str() {
                    "--shell" => shell_mode(),
                    _ => Err(ShellLaunchConfigurationError::UnknownArgument(argument)),
                }
            })?;

        Ok(Self { mode })
    }

    #[cfg(target_os = "linux")]
    pub fn shell_runner_options(self) -> ShellRunnerOptions {
        desktop_shell_runner_options("momo-shell")
    }
}

#[cfg(target_os = "linux")]
fn shell_mode() -> Result<ShellMode, ShellLaunchConfigurationError> {
    Ok(ShellMode::Shell)
}

#[cfg(not(target_os = "linux"))]
fn shell_mode() -> Result<ShellMode, ShellLaunchConfigurationError> {
    Err(ShellLaunchConfigurationError::ShellModeUnsupported)
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
