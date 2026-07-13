use super::GreeterLaunchConfiguration;
use momo_greeter_lib::GreeterMode;

fn arguments(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}

#[test]
fn defaults_to_the_shell_surface_runner() {
    let configuration = GreeterLaunchConfiguration::from_args(arguments(&[]));

    assert_eq!(configuration.mode, GreeterMode::Shell);
}

#[test]
fn standalone_test_flag_selects_the_standalone_runner() {
    let configuration = GreeterLaunchConfiguration::from_args(arguments(&["--standalone-test"]));

    assert_eq!(configuration.mode, GreeterMode::Standalone);
    assert!(configuration.into_greeter_arguments().is_empty());
}

#[test]
fn standalone_test_flag_is_not_passed_to_the_greeter() {
    let configuration = GreeterLaunchConfiguration::from_args(arguments(&[
        "--mock-users",
        "--standalone-test",
        "--session-command",
        "wayfire",
    ]));

    assert_eq!(configuration.mode, GreeterMode::Standalone);
    assert_eq!(
        configuration.into_greeter_arguments(),
        arguments(&["--mock-users", "--session-command", "wayfire"])
    );
}
