use daikore::integration::input::Key;
use std::sync::Arc;

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

    pub fn alt_tab() -> Self {
        Self {
            keys: vec![Key::Alt, Key::Tab],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShortcutRegistration {
    pub id: ShortcutId,
    pub trigger: ShortcutTrigger,
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
    pub title: Arc<String>,
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
