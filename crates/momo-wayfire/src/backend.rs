use crate::{
    binding::wayfire_binding,
    client::WayfireClient,
    protocol::{BindingRegistrationResponse, MethodCall, WayfireEvent, WayfireView, object},
};
use momo_compositor::{
    BackendMetadata, CapabilitySet, CompositorBackend, CompositorCommand, CompositorError,
    CompositorEvent, CompositorSession, CompositorSnapshot, CompositorStartupConfiguration,
    EventLoopShutdownReceiver, ShortcutId, ViewSummary,
};
use serde_json::{Value, json};
use std::{
    collections::HashMap,
    env,
    path::PathBuf,
    sync::{Arc, mpsc::Sender},
};
use tokio::sync::mpsc::UnboundedReceiver;

const WAYFIRE_SOCKET_ENVIRONMENT_VARIABLE: &str = "WAYFIRE_SOCKET";
const VIEW_EVENTS: [&str; 5] = [
    "view-mapped",
    "view-unmapped",
    "view-focused",
    "view-title-changed",
    "view-app-id-changed",
];
pub struct WayfireBackend {
    ipc_configuration: WayfireIpcConfiguration,
}

struct CompositorShortcut {
    id: ShortcutId,
    binding: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct WayfireIpcConfiguration {
    pub socket_path: Option<PathBuf>,
}

impl WayfireBackend {
    pub fn new(ipc_configuration: WayfireIpcConfiguration) -> Self {
        Self { ipc_configuration }
    }

    fn socket_path(&self) -> Result<PathBuf, CompositorError> {
        self.ipc_configuration
            .socket_path
            .clone()
            .or_else(|| env::var_os(WAYFIRE_SOCKET_ENVIRONMENT_VARIABLE).map(PathBuf::from))
            .ok_or_else(|| {
                CompositorError::new(
                    "WAYFIRE_SOCKET is not set and no Wayfire socket path was configured",
                )
            })
    }
}

impl Default for WayfireBackend {
    fn default() -> Self {
        Self::new(WayfireIpcConfiguration::default())
    }
}

impl CompositorBackend for WayfireBackend {
    fn metadata(&self) -> BackendMetadata {
        BackendMetadata { name: "wayfire" }
    }

    fn capabilities(&self) -> CapabilitySet {
        CapabilitySet {
            workspace_control: false,
            view_management: true,
            output_management: false,
            plugin_activation: false,
            global_shortcuts: true,
        }
    }

    fn start(
        self,
        configuration: CompositorStartupConfiguration,
    ) -> Result<CompositorSession, CompositorError> {
        let socket_path = self.socket_path()?;
        let shortcut_registrations = configuration
            .shortcuts
            .into_iter()
            .map(|shortcut| {
                wayfire_binding(&shortcut.trigger)
                    .map(|binding| CompositorShortcut {
                        id: shortcut.id,
                        binding,
                    })
                    .map_err(|error| CompositorError::new(error.to_string()))
            })
            .collect::<Result<Vec<_>, _>>()?;
        let metadata = self.metadata();
        let capabilities = self.capabilities();
        let snapshot = CompositorSnapshot::default();
        CompositorSession::spawn(
            metadata,
            capabilities,
            snapshot,
            move |event_sender, command_receiver, shutdown_receiver| {
                let runtime = tokio::runtime::Builder::new_current_thread()
                    .enable_io()
                    .enable_time()
                    .build()
                    .map_err(|error| {
                        CompositorError::new(format!(
                            "failed to create the Wayfire IPC runtime: {error}"
                        ))
                    })?;
                runtime.block_on(run_wayfire(
                    socket_path,
                    shortcut_registrations,
                    event_sender,
                    command_receiver,
                    shutdown_receiver,
                ))
            },
        )
    }
}

async fn run_wayfire(
    socket_path: PathBuf,
    shortcut_registrations: Vec<CompositorShortcut>,
    event_sender: Sender<CompositorEvent>,
    mut command_receiver: UnboundedReceiver<CompositorCommand>,
    mut shutdown_receiver: EventLoopShutdownReceiver,
) -> Result<(), CompositorError> {
    let mut client = tokio::select! {
        () = shutdown_receiver.wait() => return Ok(()),
        result = WayfireClient::connect(&socket_path) => {
            result.map_err(|error| CompositorError::new(error.to_string()))?
        }
    };
    let mut binding_shortcuts = HashMap::new();
    for shortcut in shortcut_registrations {
        if !register_shortcut(
            &mut client,
            &mut binding_shortcuts,
            shortcut,
            &mut shutdown_receiver,
        )
        .await?
        {
            return Ok(());
        }
    }
    watch_view_events(&mut client, &mut shutdown_receiver).await?;
    let mut snapshot = fetch_snapshot(&mut client, &mut shutdown_receiver).await?;
    if event_sender.send(CompositorEvent::Connected).is_err()
        || event_sender
            .send(CompositorEvent::SnapshotChanged(snapshot.clone()))
            .is_err()
    {
        return Ok(());
    }
    while let Some(message) = client.take_queued_event() {
        if !forward_message(&binding_shortcuts, message, &mut snapshot, &event_sender)? {
            return Ok(());
        }
    }

    loop {
        tokio::select! {
            () = shutdown_receiver.wait() => return Ok(()),
            command = command_receiver.recv() => {
                let Some(command) = command else {
                    continue;
                };
                let command_result = execute_command(
                    &mut client,
                    command,
                    &mut snapshot,
                    &event_sender,
                    &mut shutdown_receiver,
                ).await;
                match command_result {
                    Ok(true) => {}
                    Ok(false) => return Ok(()),
                    Err(error) => {
                        tracing::warn!(?error, "Wayfire rejected a compositor command");
                    }
                }
                while let Some(message) = client.take_queued_event() {
                    if !forward_message(
                        &binding_shortcuts,
                        message,
                        &mut snapshot,
                        &event_sender,
                    )? {
                        return Ok(());
                    }
                }
            }
            message = client.receive_message() => {
                let message = message.map_err(|error| CompositorError::new(error.to_string()))?;
                if !forward_message(
                    &binding_shortcuts,
                    message,
                    &mut snapshot,
                    &event_sender,
                )? {
                    return Ok(());
                }
            }
        }
    }
}

async fn watch_view_events(
    client: &mut WayfireClient,
    shutdown_receiver: &mut EventLoopShutdownReceiver,
) -> Result<(), CompositorError> {
    let request = MethodCall::new(
        "window-rules/events/watch",
        object([("events", json!(VIEW_EVENTS))]),
    );
    tokio::select! {
        () = shutdown_receiver.wait() => Ok(()),
        result = client.request(&request) => result
            .map(|_| ())
            .map_err(|error| CompositorError::new(error.to_string())),
    }
}

async fn fetch_snapshot(
    client: &mut WayfireClient,
    shutdown_receiver: &mut EventLoopShutdownReceiver,
) -> Result<CompositorSnapshot, CompositorError> {
    let request = MethodCall::new("window-rules/list-views", Value::Object(Default::default()));
    let response = tokio::select! {
        () = shutdown_receiver.wait() => return Ok(CompositorSnapshot::default()),
        result = client.request(&request) => result
            .map_err(|error| CompositorError::new(error.to_string()))?,
    };
    let views: Vec<WayfireView> = serde_json::from_value(response)
        .map_err(|error| CompositorError::new(format!("invalid Wayfire view list: {error}")))?;
    Ok(CompositorSnapshot {
        outputs: Vec::new(),
        views: views.into_iter().filter_map(view_summary).collect(),
    })
}

async fn execute_command(
    client: &mut WayfireClient,
    command: CompositorCommand,
    snapshot: &mut CompositorSnapshot,
    event_sender: &Sender<CompositorEvent>,
    shutdown_receiver: &mut EventLoopShutdownReceiver,
) -> Result<bool, CompositorError> {
    if matches!(command, CompositorCommand::RefreshSnapshot) {
        *snapshot = fetch_snapshot(client, shutdown_receiver).await?;
        return Ok(event_sender
            .send(CompositorEvent::SnapshotChanged(snapshot.clone()))
            .is_ok());
    }

    let request = match command {
        CompositorCommand::FocusView { view_id } => MethodCall::new(
            "window-rules/focus-view",
            object([("id", Value::from(view_id))]),
        ),
        CompositorCommand::CloseView { view_id } => MethodCall::new(
            "window-rules/close-view",
            object([("id", Value::from(view_id))]),
        ),
        unsupported_command => {
            tracing::warn!(
                ?unsupported_command,
                "unsupported Wayfire compositor command"
            );
            return Ok(true);
        }
    };
    tokio::select! {
        () = shutdown_receiver.wait() => Ok(false),
        result = client.request(&request) => result
            .map(|_| true)
            .map_err(|error| CompositorError::new(error.to_string())),
    }
}

async fn register_shortcut(
    client: &mut WayfireClient,
    binding_shortcuts: &mut HashMap<u64, ShortcutId>,
    shortcut: CompositorShortcut,
    shutdown_receiver: &mut EventLoopShutdownReceiver,
) -> Result<bool, CompositorError> {
    let request = MethodCall::new(
        "command/register-binding",
        object([("binding", Value::String(shortcut.binding))]),
    );
    let response = tokio::select! {
        () = shutdown_receiver.wait() => return Ok(false),
        result = client.request(&request) => {
            result.map_err(|error| CompositorError::new(error.to_string()))?
        }
    };
    let registration: BindingRegistrationResponse = serde_json::from_value(response)
        .map_err(|error| CompositorError::new(format!("invalid binding response: {error}")))?;
    binding_shortcuts.insert(registration.binding_id, shortcut.id);
    Ok(true)
}

fn forward_message(
    binding_shortcuts: &HashMap<u64, ShortcutId>,
    message: Value,
    snapshot: &mut CompositorSnapshot,
    event_sender: &Sender<CompositorEvent>,
) -> Result<bool, CompositorError> {
    let event: WayfireEvent = serde_json::from_value(message)
        .map_err(|error| CompositorError::new(format!("invalid Wayfire event: {error}")))?;

    let should_continue = match event.event.as_str() {
        "command-binding" => handle_command_binding_event(binding_shortcuts, &event, event_sender),
        "view-unmapped" => handle_view_unmapped_event(snapshot, &event, event_sender),
        "view-focused" => handle_view_updated_event(snapshot, &event, true, event_sender),
        "view-mapped" | "view-title-changed" | "view-app-id-changed" => {
            handle_view_updated_event(snapshot, &event, false, event_sender)
        }
        _ => true,
    };
    Ok(should_continue)
}

fn handle_command_binding_event(
    binding_shortcuts: &HashMap<u64, ShortcutId>,
    event: &WayfireEvent,
    event_sender: &Sender<CompositorEvent>,
) -> bool {
    let Some(shortcut_id) = event
        .binding_id
        .and_then(|binding_id| binding_shortcuts.get(&binding_id).copied())
    else {
        return true;
    };
    event_sender
        .send(CompositorEvent::ShortcutActivated(shortcut_id))
        .is_ok()
}

fn handle_view_unmapped_event(
    snapshot: &mut CompositorSnapshot,
    event: &WayfireEvent,
    event_sender: &Sender<CompositorEvent>,
) -> bool {
    if let Some(view) = &event.view {
        snapshot
            .views
            .retain(|summary| summary.identifier != view.id);
    }
    event_sender
        .send(CompositorEvent::SnapshotChanged(snapshot.clone()))
        .is_ok()
}

fn handle_view_updated_event(
    snapshot: &mut CompositorSnapshot,
    event: &WayfireEvent,
    is_focus_event: bool,
    event_sender: &Sender<CompositorEvent>,
) -> bool {
    if is_focus_event {
        for view in &mut snapshot.views {
            view.is_focused = false;
        }
    }
    let summary = event.view.clone().and_then(view_summary);
    let focused_view_id = is_focus_event
        .then(|| summary.as_ref().map(|view| view.identifier))
        .flatten();
    if let Some(mut summary) = summary {
        if is_focus_event {
            summary.is_focused = true;
        }
        if let Some(existing) = snapshot
            .views
            .iter_mut()
            .find(|view| view.identifier == summary.identifier)
        {
            *existing = summary;
        } else {
            snapshot.views.push(summary);
        }
    }
    if let Some(view_id) = focused_view_id
        && event_sender
            .send(CompositorEvent::ViewFocused { view_id })
            .is_err()
    {
        return false;
    }
    event_sender
        .send(CompositorEvent::SnapshotChanged(snapshot.clone()))
        .is_ok()
}

fn view_summary(view: WayfireView) -> Option<ViewSummary> {
    if !view.mapped || view.view_type != "toplevel" {
        return None;
    }
    Some(ViewSummary {
        identifier: view.id,
        title: Arc::new(view.title),
        app_id: view.app_id,
        output_name: view.output_name.filter(|name| name != "null"),
        workspace_identifier: None,
        is_focused: view.activated,
    })
}
