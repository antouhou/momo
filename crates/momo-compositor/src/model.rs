use daikore::integration::input::Key;
use std::str::FromStr;
use thiserror::Error;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CapabilitySet {
    pub workspace_control: bool,
    pub view_management: bool,
    pub output_management: bool,
    pub plugin_activation: bool,
    pub global_shortcuts: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShortcutId(u64);

impl ShortcutId {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShortcutTrigger {
    pub keys: Vec<Key>,
}

impl ShortcutTrigger {
    pub fn super_key() -> Self {
        Self {
            keys: vec![Key::Super],
        }
    }
}

impl FromStr for ShortcutTrigger {
    type Err = ShortcutTriggerParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.trim().is_empty() {
            return Err(ShortcutTriggerParseError::Empty);
        }
        let mut keys = Vec::new();

        for component in value.split('+').map(str::trim) {
            if component.is_empty() {
                return Err(ShortcutTriggerParseError::EmptyComponent);
            }
            keys.push(parse_shortcut_key(component)?);
        }

        Ok(Self { keys })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShortcutRegistration {
    pub id: ShortcutId,
    pub trigger: ShortcutTrigger,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ShortcutTriggerParseError {
    #[error("shortcut cannot be empty")]
    Empty,
    #[error("shortcut contains an empty component")]
    EmptyComponent,
    #[error("unsupported shortcut key: {0}")]
    UnsupportedKey(String),
    #[error("function key must be between F1 and F24")]
    InvalidFunctionKey,
}

fn parse_shortcut_key(component: &str) -> Result<Key, ShortcutTriggerParseError> {
    if component.eq_ignore_ascii_case("ctrl") || component.eq_ignore_ascii_case("control") {
        return Ok(Key::Control);
    } else if component.eq_ignore_ascii_case("alt") {
        return Ok(Key::Alt);
    } else if component.eq_ignore_ascii_case("shift") {
        return Ok(Key::Shift);
    } else if component.eq_ignore_ascii_case("super") {
        return Ok(Key::Super);
    }
    if component.len() == 1 {
        let character = component
            .chars()
            .next()
            .expect("a one-byte shortcut component contains one character");
        if character.is_ascii_alphanumeric() {
            return Ok(alphanumeric_key(character));
        }
    }
    if let Some(function_number) = component
        .strip_prefix('F')
        .or_else(|| component.strip_prefix('f'))
        .and_then(|number| number.parse::<u8>().ok())
    {
        return function_key(function_number).ok_or(ShortcutTriggerParseError::InvalidFunctionKey);
    }
    let key = if component.eq_ignore_ascii_case("space") {
        Key::Space
    } else if component.eq_ignore_ascii_case("enter") {
        Key::Enter
    } else if component.eq_ignore_ascii_case("tab") {
        Key::Tab
    } else if component.eq_ignore_ascii_case("escape") || component.eq_ignore_ascii_case("esc") {
        Key::Escape
    } else if component.eq_ignore_ascii_case("backspace") {
        Key::Backspace
    } else if component.eq_ignore_ascii_case("up") {
        Key::ArrowUp
    } else if component.eq_ignore_ascii_case("down") {
        Key::ArrowDown
    } else if component.eq_ignore_ascii_case("left") {
        Key::ArrowLeft
    } else if component.eq_ignore_ascii_case("right") {
        Key::ArrowRight
    } else if component.eq_ignore_ascii_case("home") {
        Key::Home
    } else if component.eq_ignore_ascii_case("end") {
        Key::End
    } else if component.eq_ignore_ascii_case("pageup") {
        Key::PageUp
    } else if component.eq_ignore_ascii_case("pagedown") {
        Key::PageDown
    } else if component.eq_ignore_ascii_case("insert") {
        Key::Insert
    } else if component.eq_ignore_ascii_case("delete") {
        Key::Delete
    } else {
        return Err(ShortcutTriggerParseError::UnsupportedKey(
            component.to_string(),
        ));
    };
    Ok(key)
}

fn alphanumeric_key(character: char) -> Key {
    match character.to_ascii_uppercase() {
        '0' => Key::Digit0,
        '1' => Key::Digit1,
        '2' => Key::Digit2,
        '3' => Key::Digit3,
        '4' => Key::Digit4,
        '5' => Key::Digit5,
        '6' => Key::Digit6,
        '7' => Key::Digit7,
        '8' => Key::Digit8,
        '9' => Key::Digit9,
        'A' => Key::A,
        'B' => Key::B,
        'C' => Key::C,
        'D' => Key::D,
        'E' => Key::E,
        'F' => Key::F,
        'G' => Key::G,
        'H' => Key::H,
        'I' => Key::I,
        'J' => Key::J,
        'K' => Key::K,
        'L' => Key::L,
        'M' => Key::M,
        'N' => Key::N,
        'O' => Key::O,
        'P' => Key::P,
        'Q' => Key::Q,
        'R' => Key::R,
        'S' => Key::S,
        'T' => Key::T,
        'U' => Key::U,
        'V' => Key::V,
        'W' => Key::W,
        'X' => Key::X,
        'Y' => Key::Y,
        'Z' => Key::Z,
        _ => unreachable!("shortcut parser validates alphanumeric characters"),
    }
}

fn function_key(number: u8) -> Option<Key> {
    Some(match number {
        1 => Key::F1,
        2 => Key::F2,
        3 => Key::F3,
        4 => Key::F4,
        5 => Key::F5,
        6 => Key::F6,
        7 => Key::F7,
        8 => Key::F8,
        9 => Key::F9,
        10 => Key::F10,
        11 => Key::F11,
        12 => Key::F12,
        13 => Key::F13,
        14 => Key::F14,
        15 => Key::F15,
        16 => Key::F16,
        17 => Key::F17,
        18 => Key::F18,
        19 => Key::F19,
        20 => Key::F20,
        21 => Key::F21,
        22 => Key::F22,
        23 => Key::F23,
        24 => Key::F24,
        0 | 25.. => return None,
    })
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CompositorSnapshot {
    pub outputs: Vec<Output>,
    pub views: Vec<ViewSummary>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Output {
    pub name: String,
    pub workspaces: Vec<Workspace>,
    pub focused_workspace: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Workspace {
    pub identifier: String,
    pub label: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ViewSummary {
    pub identifier: u64,
    pub title: String,
    pub app_id: Option<String>,
    pub output_name: Option<String>,
    pub workspace_identifier: Option<String>,
    pub is_focused: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompositorCommand {
    RefreshSnapshot,
    FocusView {
        view_id: u64,
    },
    CloseView {
        view_id: u64,
    },
    SwitchWorkspace {
        output_name: String,
        workspace_identifier: String,
    },
    ActivatePluginBinding {
        binding_name: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompositorEvent {
    Connected,
    Disconnected,
    SnapshotChanged(CompositorSnapshot),
    ViewFocused {
        view_id: u64,
    },
    WorkspaceChanged {
        output_name: String,
        workspace_identifier: String,
    },
    ShortcutActivated(ShortcutId),
}
