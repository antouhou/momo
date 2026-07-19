use crate::{
    binding::wayfire_binding,
    client::WayfireClient,
    protocol::{BindingRegistrationResponse, MethodCall, WayfireEvent, object},
};
use momo_compositor::{
    BackendMetadata, CapabilitySet, CompositorBackend, CompositorError, CompositorEvent,
    CompositorSession, CompositorSnapshot, CompositorStartupConfiguration,
    EventLoopShutdownReceiver, ShortcutId,
};
use serde_json::Value;
use std::{collections::HashMap, env, path::PathBuf, sync::mpsc::Sender};

const WAYFIRE_SOCKET_ENVIRONMENT_VARIABLE: &str = "WAYFIRE_SOCKET";
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
            view_management: false,
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
            move |event_sender, shutdown_receiver| {
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
    if event_sender.send(CompositorEvent::Connected).is_err() {
        return Ok(());
    }
    while let Some(message) = client.take_queued_event() {
        if !forward_message(&binding_shortcuts, message, &event_sender)? {
            return Ok(());
        }
    }

    loop {
        tokio::select! {
            () = shutdown_receiver.wait() => return Ok(()),
            message = client.receive_message() => {
                let message = message.map_err(|error| CompositorError::new(error.to_string()))?;
                if !forward_message(&binding_shortcuts, message, &event_sender)? {
                    return Ok(());
                }
            }
        }
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
    event_sender: &Sender<CompositorEvent>,
) -> Result<bool, CompositorError> {
    let Some(event) = compositor_event(binding_shortcuts, message)? else {
        return Ok(true);
    };
    Ok(event_sender.send(event).is_ok())
}

fn compositor_event(
    binding_shortcuts: &HashMap<u64, ShortcutId>,
    message: Value,
) -> Result<Option<CompositorEvent>, CompositorError> {
    let event: WayfireEvent = serde_json::from_value(message)
        .map_err(|error| CompositorError::new(format!("invalid Wayfire event: {error}")))?;
    if event.event != "command-binding" {
        return Ok(None);
    }
    Ok(event
        .binding_id
        .and_then(|binding_id| binding_shortcuts.get(&binding_id).copied())
        .map(CompositorEvent::ShortcutActivated))
}
