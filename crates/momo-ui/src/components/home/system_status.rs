use crate::components::home::model::{
    HOME_BATTERY_HANDLE_ID, HOME_BATTERY_OBSERVATION_ID, HOME_BATTERY_STATE_ID,
    HOME_VOLUME_HANDLE_ID, HOME_VOLUME_OBSERVATION_ID, HOME_VOLUME_STATE_ID,
};
use daiko::AppContext;
use daiko::Id;
use daiko::component::ComponentContext;
use daiko::state_management::StateHandle;
use system_control::{
    BatteryChargingState, BatteryFeatureState, BatteryHandle, BatteryObservation, FeatureState,
    VolumeFeatureState, VolumeHandle, VolumeObservation,
};

const FALLBACK_VOLUME_PERCENTAGE: u8 = 40;

#[derive(Clone, Copy)]
pub(crate) struct UiVolumeState {
    pub(crate) output_percentage: u8,
}

impl Default for UiVolumeState {
    fn default() -> Self {
        Self {
            output_percentage: FALLBACK_VOLUME_PERCENTAGE,
        }
    }
}

#[derive(Clone, Copy, Default)]
pub(crate) struct UiBatteryState {
    pub(crate) percentage: Option<u8>,
    pub(crate) charging_state: Option<BatteryChargingState>,
}

pub(crate) fn initialize_system_status_state(
    app_context: &mut AppContext,
    volume_handle: VolumeHandle,
    battery_handle: BatteryHandle,
) {
    let volume_handle_state =
        app_context.peek_global_state(Id::new(HOME_VOLUME_HANDLE_ID), move || volume_handle);
    let battery_handle_state =
        app_context.peek_global_state(Id::new(HOME_BATTERY_HANDLE_ID), move || battery_handle);
    let volume_state =
        app_context.peek_global_state(Id::new(HOME_VOLUME_STATE_ID), UiVolumeState::default);
    let battery_state =
        app_context.peek_global_state(Id::new(HOME_BATTERY_STATE_ID), UiBatteryState::default);
    let battery_observation = app_context
        .peek_global_state(Id::new(HOME_BATTERY_OBSERVATION_ID), || {
            None::<BatteryObservation>
        });
    let volume_observation = app_context
        .peek_global_state(Id::new(HOME_VOLUME_OBSERVATION_ID), || {
            None::<VolumeObservation>
        });

    if volume_observation.read().is_none() {
        let volume_state_handle = volume_state;
        let observation = volume_handle_state
            .read()
            .clone()
            .observe(move |next_state| {
                *volume_state_handle.write() = build_volume_state(next_state);
            });
        *volume_observation.write_silent() = Some(observation);
    }

    if battery_observation.read().is_none() {
        let battery_state_handle = battery_state;
        let observation = battery_handle_state
            .read()
            .clone()
            .observe(move |next_state| {
                *battery_state_handle.write() = build_battery_state(next_state);
            });
        *battery_observation.write_silent() = Some(observation);
    }
}

pub(crate) fn volume_state(ctx: &mut ComponentContext) -> StateHandle<UiVolumeState> {
    ctx.use_global_state(Id::new(HOME_VOLUME_STATE_ID), UiVolumeState::default)
}

pub(crate) fn battery_state(ctx: &mut ComponentContext) -> StateHandle<UiBatteryState> {
    ctx.use_global_state(Id::new(HOME_BATTERY_STATE_ID), UiBatteryState::default)
}

pub(crate) fn volume_handle(ctx: &mut ComponentContext) -> VolumeHandle {
    ctx.use_global_state(Id::new(HOME_VOLUME_HANDLE_ID), || -> VolumeHandle {
        panic!("Volume handle must be initialized before quick settings render")
    })
    .read()
    .clone()
}

fn build_volume_state(feature_state: VolumeFeatureState) -> UiVolumeState {
    match feature_state {
        FeatureState::Ready(state) => UiVolumeState {
            output_percentage: state.output_percentage,
        },
        FeatureState::Loading | FeatureState::Unsupported(_) | FeatureState::Unavailable(_) => {
            UiVolumeState::default()
        }
    }
}

fn build_battery_state(feature_state: BatteryFeatureState) -> UiBatteryState {
    match feature_state {
        FeatureState::Ready(state) => UiBatteryState {
            percentage: Some(state.percentage),
            charging_state: Some(state.charging_state),
        },
        FeatureState::Loading | FeatureState::Unsupported(_) | FeatureState::Unavailable(_) => {
            UiBatteryState {
                percentage: None,
                charging_state: None,
            }
        }
    }
}
