use std::cmp::Reverse;

use crate::components::home::model::{
    HOME_BLUETOOTH_HANDLE_ID, HOME_BLUETOOTH_OBSERVATION_ID, HOME_BLUETOOTH_STATE_ID,
};
use daiko::AppContext;
use daiko::Id;
use daiko::component::ComponentContext;
use daiko::state_management::StateHandle;
use system_control::{
    BluetoothConnectionState, BluetoothDevice, BluetoothDeviceCategory, BluetoothFeatureState,
    BluetoothHandle, BluetoothObservation, BluetoothPowerState, FeatureState,
};

#[derive(Clone)]
pub(crate) struct BluetoothState {
    pub(crate) is_enabled: bool,
    pub(crate) can_toggle_power: bool,
    pub(crate) recent_devices: BluetoothDeviceSection,
    pub(crate) nearby_devices: BluetoothDeviceSection,
}

impl Default for BluetoothState {
    fn default() -> Self {
        Self {
            is_enabled: false,
            can_toggle_power: false,
            recent_devices: BluetoothDeviceSection::Loading,
            nearby_devices: BluetoothDeviceSection::Loading,
        }
    }
}

#[derive(Clone)]
pub(crate) enum BluetoothDeviceSection {
    Loading,
    Unavailable,
    Ready(Vec<BluetoothDeviceState>),
}

#[derive(Clone)]
pub(crate) struct BluetoothDeviceState {
    pub(crate) device_identifier: system_control::BluetoothDeviceId,
    pub(crate) tag: String,
    pub(crate) display_name: String,
    pub(crate) category: BluetoothDeviceCategory,
    pub(crate) connection_state: BluetoothConnectionState,
}

pub(crate) fn initialize_bluetooth_state(
    app_context: &mut AppContext,
    bluetooth_handle: BluetoothHandle,
) {
    let bluetooth_handle_state =
        app_context.peek_global_state(Id::new(HOME_BLUETOOTH_HANDLE_ID), move || bluetooth_handle);
    let bluetooth_state =
        app_context.peek_global_state(Id::new(HOME_BLUETOOTH_STATE_ID), BluetoothState::default);
    let bluetooth_observation = app_context
        .peek_global_state(Id::new(HOME_BLUETOOTH_OBSERVATION_ID), || {
            None::<BluetoothObservation>
        });

    *bluetooth_state.write_silent() =
        build_bluetooth_state(bluetooth_handle_state.read().clone().current_state());

    if bluetooth_observation.read().is_none() {
        let bluetooth_state_handle = bluetooth_state.clone();
        let observation = bluetooth_handle_state
            .read()
            .clone()
            .observe(move |next_state| {
                *bluetooth_state_handle.write() = build_bluetooth_state(next_state);
            });
        *bluetooth_observation.write_silent() = Some(observation);
    }
}

pub(crate) fn bluetooth_handle(ctx: &mut ComponentContext) -> BluetoothHandle {
    ctx.use_global_state(Id::new(HOME_BLUETOOTH_HANDLE_ID), || -> BluetoothHandle {
        panic!("Bluetooth handle must be initialized before quick settings render")
    })
    .read()
    .clone()
}

pub(crate) fn bluetooth_state(ctx: &mut ComponentContext) -> StateHandle<BluetoothState> {
    ctx.use_global_state(Id::new(HOME_BLUETOOTH_STATE_ID), BluetoothState::default)
}

fn build_bluetooth_state(feature_state: BluetoothFeatureState) -> BluetoothState {
    match &feature_state {
        FeatureState::Ready(state) => {
            let is_enabled = matches!(
                state.adapter.power_state,
                BluetoothPowerState::On | BluetoothPowerState::TurningOn { .. }
            );
            let can_toggle_power = state.adapter.capabilities.can_change_power;
            let recent_devices = sorted_recent_devices(&state.devices)
                .into_iter()
                .map(build_bluetooth_device_state)
                .collect();
            let nearby_devices = sorted_nearby_devices(&state.devices)
                .into_iter()
                .map(build_bluetooth_device_state)
                .collect();

            BluetoothState {
                is_enabled,
                can_toggle_power,
                recent_devices: BluetoothDeviceSection::Ready(recent_devices),
                nearby_devices: BluetoothDeviceSection::Ready(nearby_devices),
            }
        }
        FeatureState::Loading => BluetoothState::default(),
        FeatureState::Unsupported(_) | FeatureState::Unavailable(_) => BluetoothState {
            is_enabled: false,
            can_toggle_power: false,
            recent_devices: BluetoothDeviceSection::Unavailable,
            nearby_devices: BluetoothDeviceSection::Unavailable,
        },
    }
}

fn build_bluetooth_device_state(device: &BluetoothDevice) -> BluetoothDeviceState {
    BluetoothDeviceState {
        device_identifier: device.device_identifier.clone(),
        tag: bluetooth_device_tag(device),
        display_name: device.display_name.clone(),
        category: device.category,
        connection_state: device.connection_state.clone(),
    }
}

fn bluetooth_device_tag(device: &BluetoothDevice) -> String {
    format!(
        "header-settings-bluetooth-device-{}",
        device
            .device_identifier
            .0
            .chars()
            .map(|character| if character.is_ascii_alphanumeric() {
                character
            } else {
                '-'
            })
            .collect::<String>()
    )
}

fn sorted_recent_devices(devices: &[BluetoothDevice]) -> Vec<&BluetoothDevice> {
    let mut recent_devices = devices
        .iter()
        .filter(|device| is_recent_device(device))
        .collect::<Vec<_>>();
    recent_devices.sort_by_key(|device| !is_device_connected(device));
    recent_devices
}

fn sorted_nearby_devices(devices: &[BluetoothDevice]) -> Vec<&BluetoothDevice> {
    let mut nearby_devices = devices
        .iter()
        .filter(|device| is_nearby_device(device))
        .collect::<Vec<_>>();
    nearby_devices.sort_by_key(|device| Reverse(device.signal_strength_dbm));
    nearby_devices
}

fn is_device_connected(device: &BluetoothDevice) -> bool {
    matches!(
        device.connection_state,
        BluetoothConnectionState::Connected
            | BluetoothConnectionState::Connecting { .. }
            | BluetoothConnectionState::Disconnecting { .. }
    )
}

fn is_recent_device(device: &BluetoothDevice) -> bool {
    device.is_paired || is_device_connected(device)
}

fn is_nearby_device(device: &BluetoothDevice) -> bool {
    !device.is_paired
        && !is_device_connected(device)
        && device.signal_strength_dbm.is_some()
        && has_presentable_device_name(&device.display_name)
}

fn has_presentable_device_name(display_name: &str) -> bool {
    let trimmed = display_name.trim();
    if trimmed.is_empty() {
        return false;
    }

    !looks_like_hardware_identifier(trimmed)
}

fn looks_like_hardware_identifier(value: &str) -> bool {
    let separator = if value.contains('-') {
        '-'
    } else if value.contains(':') {
        ':'
    } else {
        return false;
    };

    let parts = value.split(separator).collect::<Vec<_>>();
    parts.len() >= 3
        && parts.iter().all(|part| {
            part.len() == 2 && part.chars().all(|character| character.is_ascii_hexdigit())
        })
}

#[cfg(test)]
mod tests {
    use super::has_presentable_device_name;

    #[test]
    fn bluetooth_name_filter_keeps_regular_names() {
        assert!(has_presentable_device_name("Sony WH-1000XM5"));
        assert!(has_presentable_device_name("Magic Keyboard"));
    }

    #[test]
    fn bluetooth_name_filter_drops_address_like_names() {
        assert!(!has_presentable_device_name("0D-48-AC-11-22-33"));
        assert!(!has_presentable_device_name("0d:48:ac:11:22:33"));
        assert!(!has_presentable_device_name("  0D-48-AC-11-22-33  "));
    }
}
