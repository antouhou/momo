use std::str::FromStr;

use bluer::{Adapter, Device};
use thiserror::Error;

use crate::bluetooth::{
    BluetoothConnectionState, BluetoothDevice, BluetoothDeviceCategory, BluetoothDeviceId,
};

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

async fn classify_device(device: &Device) -> BluetoothDeviceCategory {
    let icon = device.icon().await.ok().flatten();
    let class = device.class().await.ok().flatten();
    let appearance = device.appearance().await.ok().flatten();

    classify_device_properties(icon.as_deref(), class, appearance)
}

fn classify_device_properties(
    icon: Option<&str>,
    class: Option<u32>,
    appearance: Option<u16>,
) -> BluetoothDeviceCategory {
    class
        .and_then(classify_device_class)
        .or_else(|| icon.and_then(classify_device_icon))
        .or_else(|| appearance.and_then(classify_device_appearance))
        .unwrap_or(BluetoothDeviceCategory::Unknown)
}

fn classify_device_icon(icon: &str) -> Option<BluetoothDeviceCategory> {
    let category = match icon {
        "audio-card" => BluetoothDeviceCategory::Audio,
        "audio-headphones" => BluetoothDeviceCategory::Headphones,
        "audio-headset" => BluetoothDeviceCategory::Headset,
        "audio-input-microphone" | "microphone-sensitivity-high" => {
            BluetoothDeviceCategory::Microphone
        }
        "audio-speakers" | "audio-volume-high" => BluetoothDeviceCategory::Speaker,
        "camera-photo" | "camera-video" => BluetoothDeviceCategory::Camera,
        "computer" => BluetoothDeviceCategory::Computer,
        "input-gaming" => BluetoothDeviceCategory::GameController,
        "input-keyboard" => BluetoothDeviceCategory::Keyboard,
        "input-mouse" => BluetoothDeviceCategory::Mouse,
        "input-tablet" => BluetoothDeviceCategory::Tablet,
        "modem" | "network-wireless" => BluetoothDeviceCategory::Network,
        "multimedia-player" => BluetoothDeviceCategory::MediaPlayer,
        "phone" | "smartphone" => BluetoothDeviceCategory::Phone,
        "printer" => BluetoothDeviceCategory::Printer,
        "scanner" => BluetoothDeviceCategory::Scanner,
        "video-display" => BluetoothDeviceCategory::Display,
        "unknown" => BluetoothDeviceCategory::Unknown,
        _ => return None,
    };

    Some(category)
}

fn classify_device_class(class: u32) -> Option<BluetoothDeviceCategory> {
    let category = match bluetooth_major_device_class(class) {
        0x01 => BluetoothDeviceCategory::Computer,
        0x02 => classify_phone_minor_device_class(class),
        0x03 => BluetoothDeviceCategory::Network,
        0x04 => classify_audio_video_minor_device_class(class),
        0x05 => classify_peripheral_minor_device_class(class),
        0x06 => classify_imaging_minor_device_class(class)?,
        0x07 => BluetoothDeviceCategory::Wearable,
        0x08 => BluetoothDeviceCategory::GameController,
        0x09 => BluetoothDeviceCategory::Health,
        _ => return None,
    };

    Some(category)
}

fn bluetooth_major_device_class(class: u32) -> u32 {
    (class & 0x1f00) >> 8
}

fn bluetooth_minor_device_class(class: u32) -> u32 {
    (class & 0xfc) >> 2
}

fn classify_phone_minor_device_class(class: u32) -> BluetoothDeviceCategory {
    match bluetooth_minor_device_class(class) {
        0x04 => BluetoothDeviceCategory::Network,
        _ => BluetoothDeviceCategory::Phone,
    }
}

fn classify_audio_video_minor_device_class(class: u32) -> BluetoothDeviceCategory {
    match bluetooth_minor_device_class(class) {
        0x01 | 0x02 => BluetoothDeviceCategory::Headset,
        0x04 => BluetoothDeviceCategory::Microphone,
        0x05 => BluetoothDeviceCategory::Speaker,
        0x06 => BluetoothDeviceCategory::Headphones,
        0x07 | 0x09 => BluetoothDeviceCategory::MediaPlayer,
        0x08 => BluetoothDeviceCategory::CarAudio,
        0x0b..=0x0d => BluetoothDeviceCategory::Camera,
        0x0e..=0x10 => BluetoothDeviceCategory::Display,
        _ => BluetoothDeviceCategory::Audio,
    }
}

fn classify_peripheral_minor_device_class(class: u32) -> BluetoothDeviceCategory {
    let peripheral_kind = (class & 0xc0) >> 6;
    let peripheral_subkind = (class & 0x1e) >> 2;

    match peripheral_kind {
        0x01 => BluetoothDeviceCategory::Keyboard,
        0x02 => match peripheral_subkind {
            0x05 => BluetoothDeviceCategory::Tablet,
            _ => BluetoothDeviceCategory::Mouse,
        },
        0x03 => BluetoothDeviceCategory::Input,
        _ => match peripheral_subkind {
            0x01 | 0x02 => BluetoothDeviceCategory::GameController,
            _ => BluetoothDeviceCategory::Peripheral,
        },
    }
}

fn classify_imaging_minor_device_class(class: u32) -> Option<BluetoothDeviceCategory> {
    if class & 0x80 != 0 {
        Some(BluetoothDeviceCategory::Printer)
    } else if class & 0x40 != 0 {
        Some(BluetoothDeviceCategory::Scanner)
    } else if class & 0x20 != 0 {
        Some(BluetoothDeviceCategory::Camera)
    } else if class & 0x10 != 0 {
        Some(BluetoothDeviceCategory::Display)
    } else {
        None
    }
}

fn classify_device_appearance(appearance: u16) -> Option<BluetoothDeviceCategory> {
    let category = match bluetooth_gap_appearance_category(appearance) {
        0x01 => BluetoothDeviceCategory::Phone,
        0x02 => BluetoothDeviceCategory::Computer,
        0x03 | 0x04 | 0x07 | 0x09 | 0x51 => BluetoothDeviceCategory::Wearable,
        0x05 => BluetoothDeviceCategory::Display,
        0x06 | 0x13 => BluetoothDeviceCategory::Peripheral,
        0x0a => BluetoothDeviceCategory::MediaPlayer,
        0x0b => BluetoothDeviceCategory::Scanner,
        0x0c..=0x0e | 0x10..=0x12 | 0x31..=0x36 => BluetoothDeviceCategory::Health,
        0x0f => classify_hid_gap_appearance(appearance),
        0x14 => BluetoothDeviceCategory::Network,
        0x15 => BluetoothDeviceCategory::Sensor,
        _ => return None,
    };

    Some(category)
}

fn bluetooth_gap_appearance_category(appearance: u16) -> u16 {
    (appearance & 0xffc0) >> 6
}

fn classify_hid_gap_appearance(appearance: u16) -> BluetoothDeviceCategory {
    match appearance & 0x3f {
        0x01 => BluetoothDeviceCategory::Keyboard,
        0x02 => BluetoothDeviceCategory::Mouse,
        0x03 | 0x04 => BluetoothDeviceCategory::GameController,
        0x05 | 0x07 => BluetoothDeviceCategory::Tablet,
        0x08 => BluetoothDeviceCategory::Scanner,
        _ => BluetoothDeviceCategory::Input,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_audio_video_minor_classes() {
        assert_eq!(
            classify_device_properties(Some("audio-card"), Some(0x002414), None),
            BluetoothDeviceCategory::Speaker
        );
        assert_eq!(
            classify_device_properties(Some("audio-card"), Some(0x002418), None),
            BluetoothDeviceCategory::Headphones
        );
        assert_eq!(
            classify_device_properties(Some("audio-card"), Some(0x002410), None),
            BluetoothDeviceCategory::Microphone
        );
    }

    #[test]
    fn classifies_bluez_icon_names() {
        assert_eq!(
            classify_device_properties(Some("network-wireless"), None, None),
            BluetoothDeviceCategory::Network
        );
        assert_eq!(
            classify_device_properties(Some("multimedia-player"), None, None),
            BluetoothDeviceCategory::MediaPlayer
        );
        assert_eq!(
            classify_device_properties(Some("video-display"), None, None),
            BluetoothDeviceCategory::Display
        );
    }

    #[test]
    fn classifies_gap_appearances() {
        assert_eq!(
            classify_device_properties(None, None, Some(0x03c1)),
            BluetoothDeviceCategory::Keyboard
        );
        assert_eq!(
            classify_device_properties(None, None, Some(0x03c4)),
            BluetoothDeviceCategory::GameController
        );
        assert_eq!(
            classify_device_properties(None, None, Some(0x0340)),
            BluetoothDeviceCategory::Health
        );
    }
}
