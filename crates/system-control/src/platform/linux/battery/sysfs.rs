use std::fs;
use std::path::{Path, PathBuf};

use crate::battery::{
    BatteryChargingState, BatteryFeatureState, BatteryState, BatteryUnavailableReason,
    BatteryUnsupportedReason,
};
use crate::feature_state::FeatureState;

const POWER_SUPPLY_PATH: &str = "/sys/class/power_supply";

pub(super) fn read_linux_battery_state() -> BatteryFeatureState {
    let battery_path = match find_primary_battery_path() {
        Ok(Some(path)) => path,
        Ok(None) => {
            return FeatureState::Unsupported(BatteryUnsupportedReason::NoBatteryPresent);
        }
        Err(error) => {
            return FeatureState::Unavailable(BatteryUnavailableReason::BackendUnavailable {
                message: format!("failed to inspect power supplies: {error}"),
            });
        }
    };

    let capacity = match read_trimmed(battery_path.join("capacity")) {
        Ok(capacity) => capacity,
        Err(error) => {
            return FeatureState::Unavailable(BatteryUnavailableReason::BackendUnavailable {
                message: format!("failed to read battery capacity: {error}"),
            });
        }
    };
    let percentage = match capacity.parse::<u8>() {
        Ok(percentage) => percentage,
        Err(error) => {
            return FeatureState::Unavailable(BatteryUnavailableReason::BackendUnavailable {
                message: format!("failed to parse battery capacity: {error}"),
            });
        }
    };
    let status =
        read_trimmed(battery_path.join("status")).unwrap_or_else(|_| "Unknown".to_string());

    FeatureState::Ready(BatteryState::new(
        percentage,
        parse_battery_charging_state(&status),
    ))
}

fn find_primary_battery_path() -> std::io::Result<Option<PathBuf>> {
    let mut batteries = Vec::new();

    for entry in fs::read_dir(POWER_SUPPLY_PATH)? {
        let entry = entry?;
        let path = entry.path();
        if read_trimmed(path.join("type"))
            .map(|power_supply_type| power_supply_type == "Battery")
            .unwrap_or(false)
        {
            batteries.push(path);
        }
    }

    batteries.sort();
    Ok(batteries.into_iter().next())
}

fn read_trimmed(path: impl AsRef<Path>) -> std::io::Result<String> {
    fs::read_to_string(path).map(|value| value.trim().to_string())
}

fn parse_battery_charging_state(status: &str) -> BatteryChargingState {
    match status {
        "Charging" => BatteryChargingState::Charging,
        "Discharging" => BatteryChargingState::Discharging,
        "Full" => BatteryChargingState::Full,
        "Not charging" => BatteryChargingState::NotCharging,
        _ => BatteryChargingState::Unknown,
    }
}
