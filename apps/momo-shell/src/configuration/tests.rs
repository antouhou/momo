use super::{ShellLaunchConfiguration, ShellLaunchConfigurationError};
#[cfg(target_os = "linux")]
use dailand::{ShellBackend, ShellKeyboardInteractivity, ShellLayer};
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
    assert_eq!(
        runner_options.surface.keyboard_interactivity,
        ShellKeyboardInteractivity::OnDemand
    );
    assert!(runner_options.surface.request_initial_keyboard_focus);
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
