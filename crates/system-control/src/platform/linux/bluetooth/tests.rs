use super::device_category::classify_device_properties;
use crate::bluetooth::BluetoothDeviceCategory;

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
