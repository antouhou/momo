use crate::{
    feature_state::FeatureState,
    volume::{VolumeFeatureState, VolumeRequestError, VolumeState, VolumeUnavailableReason},
};
use std::process::Command;

const DEFAULT_AUDIO_SINK: &str = "@DEFAULT_AUDIO_SINK@";

pub(super) fn read_linux_volume_state() -> VolumeFeatureState {
    let output = match Command::new("wpctl")
        .args(["get-volume", DEFAULT_AUDIO_SINK])
        .output()
    {
        Ok(output) => output,
        Err(error) => {
            return FeatureState::Unavailable(VolumeUnavailableReason::BackendUnavailable {
                message: format!("failed to run wpctl: {error}"),
            });
        }
    };

    if !output.status.success() {
        return FeatureState::Unavailable(VolumeUnavailableReason::BackendUnavailable {
            message: command_error_message("wpctl get-volume", &output.stderr),
        });
    }

    match parse_wpctl_volume_output(&String::from_utf8_lossy(&output.stdout)) {
        Some(state) => FeatureState::Ready(state),
        None => FeatureState::Unavailable(VolumeUnavailableReason::BackendUnavailable {
            message: "failed to parse wpctl volume output".to_string(),
        }),
    }
}

pub(super) fn set_linux_output_volume(output_percentage: u8) -> Result<(), VolumeRequestError> {
    let output_percentage = output_percentage.min(100);
    let volume_argument = format!("{output_percentage}%");
    let output = Command::new("wpctl")
        .args(["set-volume", DEFAULT_AUDIO_SINK, &volume_argument])
        .output()
        .map_err(|_| VolumeRequestError::RuntimeUnavailable)?;

    if output.status.success() {
        Ok(())
    } else {
        Err(VolumeRequestError::RuntimeUnavailable)
    }
}

fn parse_wpctl_volume_output(output: &str) -> Option<VolumeState> {
    let trimmed = output.trim();
    let value = trimmed
        .strip_prefix("Volume:")
        .unwrap_or(trimmed)
        .split_whitespace()
        .next()?
        .parse::<f32>()
        .ok()?;
    let output_percentage = (value * 100.0).round().clamp(0.0, 100.0) as u8;
    let is_muted = trimmed.contains("[MUTED]");

    Some(VolumeState::new(output_percentage, is_muted))
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
