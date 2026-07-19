use super::ShortcutTrigger;
use daikore::integration::input::Key;

#[test]
fn parses_modifier_only_shortcut() {
    let shortcut = "Super".parse::<ShortcutTrigger>().unwrap();

    assert_eq!(shortcut, ShortcutTrigger::super_key());
}

#[test]
fn parses_configurable_modifier_and_key_shortcut() {
    let shortcut = "Ctrl+Alt+L".parse::<ShortcutTrigger>().unwrap();

    assert_eq!(
        shortcut,
        ShortcutTrigger {
            keys: vec![Key::Control, Key::Alt, Key::L],
        }
    );
}

#[test]
fn parses_shortcuts_with_multiple_simultaneous_keys() {
    let shortcut = "Super+Space+L".parse::<ShortcutTrigger>().unwrap();

    assert_eq!(
        shortcut,
        ShortcutTrigger {
            keys: vec![Key::Super, Key::Space, Key::L],
        }
    );
}
