use std::str::FromStr;

use bluer::{Adapter, Device};
use thiserror::Error;

use super::device_category::classify_device_properties;
use crate::bluetooth::{BluetoothConnectionState, BluetoothDevice, BluetoothDeviceId};

#[derive(Debug, Error)]
pub(super) enum LinuxBluetoothDeviceError {
    #[error("invalid Bluetooth device identifier {device_identifier}: {message}")]
    InvalidIdentifier {
        device_identifier: String,
        message: String,
    },
    #[error("failed to access Bluetooth device {device_identifier}: {message}")]
    OpenDevice {
        device_identifier: String,
        message: String,
    },
}

pub(super) async fn load_device(
    adapter: &Adapter,
    device_identifier: String,
) -> Option<BluetoothDevice> {
    let device_identifier = BluetoothDeviceId(device_identifier);
    let device = device_from_identifier(adapter, &device_identifier).ok()?;
    let is_paired = device.is_paired().await.ok().unwrap_or(false);
    let is_trusted = device.is_trusted().await.ok().unwrap_or(false);
    let is_connected = device.is_connected().await.ok().unwrap_or(false);

    let alias = device.alias().await.ok().filter(|alias| !alias.is_empty());
    let remote_name = device
        .name()
        .await
        .ok()
        .flatten()
        .filter(|name| !name.is_empty());
    let display_name = if is_paired || is_trusted {
        alias.or(remote_name)
    } else {
        remote_name.or(alias)
    }
    .unwrap_or_else(|| device_identifier.0.clone());
    let signal_strength_dbm = device.rssi().await.ok().flatten();

    Some(BluetoothDevice {
        device_identifier,
        display_name,
        category: classify_device(&device).await,
        is_paired,
        is_trusted,
        connection_state: if is_connected {
            BluetoothConnectionState::Connected
        } else {
            BluetoothConnectionState::Disconnected
        },
        signal_strength_dbm,
        battery_percentage: device.battery_percentage().await.ok().flatten(),
    })
}

pub(super) fn device_from_identifier(
    adapter: &Adapter,
    device_identifier: &BluetoothDeviceId,
) -> Result<Device, LinuxBluetoothDeviceError> {
    let address = bluer::Address::from_str(&device_identifier.0).map_err(|error| {
        LinuxBluetoothDeviceError::InvalidIdentifier {
            device_identifier: device_identifier.0.clone(),
            message: error.to_string(),
        }
    })?;
    adapter
        .device(address)
        .map_err(|error| LinuxBluetoothDeviceError::OpenDevice {
            device_identifier: device_identifier.0.clone(),
            message: error.to_string(),
        })
}

pub(super) fn sort_devices(devices: &mut [BluetoothDevice]) {
    devices.sort_by(|left, right| {
        left.display_name
            .cmp(&right.display_name)
            .then(left.device_identifier.cmp(&right.device_identifier))
    });
}

async fn classify_device(device: &Device) -> crate::bluetooth::BluetoothDeviceCategory {
    let icon = device.icon().await.ok().flatten();
    let class = device.class().await.ok().flatten();
    let appearance = device.appearance().await.ok().flatten();

    classify_device_properties(icon.as_deref(), class, appearance)
}
