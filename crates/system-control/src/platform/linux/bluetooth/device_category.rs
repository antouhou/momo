use crate::bluetooth::BluetoothDeviceCategory;

pub(super) fn classify_device_properties(
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
