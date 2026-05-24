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
pub(crate) struct HomeBluetoothState {
    pub(crate) is_enabled: bool,
    pub(crate) can_toggle_power: bool,
    pub(crate) recent_devices: HomeBluetoothDeviceSection,
    pub(crate) nearby_devices: HomeBluetoothDeviceSection,
}

impl Default for HomeBluetoothState {
    fn default() -> Self {
        Self {
            is_enabled: false,
            can_toggle_power: false,
            recent_devices: HomeBluetoothDeviceSection::Loading,
            nearby_devices: HomeBluetoothDeviceSection::Loading,
        }
    }
}

#[derive(Clone)]
pub(crate) enum HomeBluetoothDeviceSection {
    Loading,
    Unavailable,
    Ready(Vec<HomeBluetoothDevice>),
}

#[derive(Clone)]
pub(crate) struct HomeBluetoothDevice {
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
    let bluetooth_state = app_context.peek_global_state(
        Id::new(HOME_BLUETOOTH_STATE_ID),
        HomeBluetoothState::default,
    );
    let bluetooth_observation = app_context
        .peek_global_state(Id::new(HOME_BLUETOOTH_OBSERVATION_ID), || {
            None::<BluetoothObservation>
        });

    *bluetooth_state.write_silent() =
        build_home_bluetooth_state(bluetooth_handle_state.read().clone().current_state());

    if bluetooth_observation.read().is_none() {
        let bluetooth_state_handle = bluetooth_state.clone();
        let observation = bluetooth_handle_state
            .read()
            .clone()
            .observe(move |next_state| {
                *bluetooth_state_handle.write() = build_home_bluetooth_state(next_state);
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

pub(crate) fn bluetooth_state(ctx: &mut ComponentContext) -> StateHandle<HomeBluetoothState> {
    ctx.use_global_state(
        Id::new(HOME_BLUETOOTH_STATE_ID),
        HomeBluetoothState::default,
    )
}

fn build_home_bluetooth_state(feature_state: BluetoothFeatureState) -> HomeBluetoothState {
    match &feature_state {
        FeatureState::Ready(state) => {
            let is_enabled = matches!(
                state.adapter.power_state,
                BluetoothPowerState::On | BluetoothPowerState::TurningOn { .. }
            );
            let can_toggle_power = state.adapter.capabilities.can_change_power;
            let recent_devices = state
                .devices
                .iter()
                .filter(|device| device.is_paired || is_device_connected(device))
                .map(build_home_bluetooth_device)
                .collect();
            let nearby_devices = state
                .devices
                .iter()
                .filter(|device| {
                    !device.is_paired
                        && !is_device_connected(device)
                        && device.signal_strength_dbm.is_some()
                })
                .map(build_home_bluetooth_device)
                .collect();

            HomeBluetoothState {
                is_enabled,
                can_toggle_power,
                recent_devices: HomeBluetoothDeviceSection::Ready(recent_devices),
                nearby_devices: HomeBluetoothDeviceSection::Ready(nearby_devices),
            }
        }
        FeatureState::Loading => HomeBluetoothState::default(),
        FeatureState::Unsupported(_) | FeatureState::Unavailable(_) => HomeBluetoothState {
            is_enabled: false,
            can_toggle_power: false,
            recent_devices: HomeBluetoothDeviceSection::Unavailable,
            nearby_devices: HomeBluetoothDeviceSection::Unavailable,
        },
    }
}

fn build_home_bluetooth_device(device: &BluetoothDevice) -> HomeBluetoothDevice {
    HomeBluetoothDevice {
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

fn is_device_connected(device: &BluetoothDevice) -> bool {
    matches!(
        device.connection_state,
        BluetoothConnectionState::Connected
            | BluetoothConnectionState::Connecting { .. }
            | BluetoothConnectionState::Disconnecting { .. }
    )
}
