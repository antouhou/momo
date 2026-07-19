use super::{WayfireBindingError, wayfire_binding};
use momo_compositor::{Key, ShortcutTrigger};

#[test]
fn serializes_multiple_regular_keys_and_modifiers() {
    let binding = wayfire_binding(&ShortcutTrigger {
        keys: vec![Key::Super, Key::Shift, Key::Space, Key::L],
    })
    .unwrap();

    assert_eq!(binding, "<super> <shift> KEY_SPACE KEY_L");
}

#[test]
fn serializes_extended_evdev_key_categories() {
    let binding = wayfire_binding(&ShortcutTrigger {
        keys: vec![
            Key::NumPad7,
            Key::AudioVolumeUp,
            Key::BrowserBack,
            Key::LaunchMail,
            Key::BrightnessDown,
            Key::ColorF0Red,
        ],
    })
    .unwrap();

    assert_eq!(
        binding,
        "KEY_KP7 KEY_VOLUMEUP KEY_BACK KEY_MAIL KEY_BRIGHTNESSDOWN KEY_RED"
    );
}

#[test]
fn serializes_specialized_keys_to_the_closest_defined_evdev_code() {
    let trigger = ShortcutTrigger {
        keys: vec![
            Key::NumPadBackspace,
            Key::NumpadHash,
            Key::SpeechInputToggle,
            Key::TVInputHDMI1,
            Key::TVDataService,
            Key::OnDemand,
            Key::Pairing,
        ],
    };

    assert_eq!(
        wayfire_binding(&trigger),
        Ok(
            "KEY_BACKSPACE KEY_NUMERIC_POUND KEY_DICTATE KEY_VIDEO KEY_DATA KEY_VOD KEY_CONNECT"
                .to_owned()
        )
    );
}

#[test]
fn reports_only_keys_without_a_mapping_as_unmapped() {
    let error = wayfire_binding(&ShortcutTrigger {
        keys: vec![Key::Unidentified],
    })
    .unwrap_err();

    assert_eq!(error, WayfireBindingError::UnmappedKey(Key::Unidentified));
}
