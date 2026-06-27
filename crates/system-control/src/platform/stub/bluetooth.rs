use crate::SystemControlError;
use crate::bluetooth::{
    BluetoothDeviceId, BluetoothFeatureState, BluetoothOperationReceipt, BluetoothRequestError,
    BluetoothUnsupportedReason, FeatureState,
};

#[derive(Clone)]
pub(crate) struct PlatformBluetoothHandle;

pub(crate) struct PlatformBluetoothObservation;

impl PlatformBluetoothHandle {
    pub(crate) fn new() -> Result<Self, SystemControlError> {
        Ok(Self)
    }

    pub(crate) fn current_state(&self) -> BluetoothFeatureState {
        FeatureState::Unsupported(BluetoothUnsupportedReason::PlatformNotSupported)
    }

    pub(crate) fn observe<F>(&self, observer: F) -> PlatformBluetoothObservation
    where
        F: Fn(BluetoothFeatureState) + Send + 'static,
    {
        observer(self.current_state());
        PlatformBluetoothObservation
    }

    pub(crate) fn set_power_enabled(
        &self,
        _is_enabled: bool,
    ) -> Result<BluetoothOperationReceipt, BluetoothRequestError> {
        Err(BluetoothRequestError::FeatureNotReady)
    }

    pub(crate) fn start_discovery(
        &self,
    ) -> Result<BluetoothOperationReceipt, BluetoothRequestError> {
        Err(BluetoothRequestError::FeatureNotReady)
    }

    pub(crate) fn stop_discovery(
        &self,
    ) -> Result<BluetoothOperationReceipt, BluetoothRequestError> {
        Err(BluetoothRequestError::FeatureNotReady)
    }

    pub(crate) fn connect_device(
        &self,
        _device_identifier: BluetoothDeviceId,
    ) -> Result<BluetoothOperationReceipt, BluetoothRequestError> {
        Err(BluetoothRequestError::FeatureNotReady)
    }

    pub(crate) fn disconnect_device(
        &self,
        _device_identifier: BluetoothDeviceId,
    ) -> Result<BluetoothOperationReceipt, BluetoothRequestError> {
        Err(BluetoothRequestError::FeatureNotReady)
    }
}
