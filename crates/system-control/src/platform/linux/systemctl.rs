use std::ffi::OsStr;
use std::process::Command;

pub(super) fn run_systemctl_command<I, S>(arguments: I) -> Result<(), String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let arguments = arguments.into_iter().collect::<Vec<_>>();
    let output = Command::new("systemctl")
        .args(&arguments)
        .output()
        .map_err(|error| format!("failed to run {}: {error}", command_label(&arguments)))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(command_error_message(
            &command_label(&arguments),
            &output.stderr,
        ))
    }
}

fn command_label<S>(arguments: &[S]) -> String
where
    S: AsRef<OsStr>,
{
    let arguments = arguments
        .iter()
        .map(|argument| argument.as_ref().to_string_lossy())
        .collect::<Vec<_>>()
        .join(" ");

    if arguments.is_empty() {
        "systemctl".to_string()
    } else {
        format!("systemctl {arguments}")
    }
}

fn command_error_message(command: &str, stderr: &[u8]) -> String {
    let stderr = String::from_utf8_lossy(stderr);
    let stderr = stderr.trim();
    if stderr.is_empty() {
        format!("{command} failed")
    } else {
        format!("{command} failed: {stderr}")
    }
}
