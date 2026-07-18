use crate::{
    client::WayfireClient,
    protocol::{BindingRegistrationResponse, MethodCall, WayfireEvent, object},
};
use momo_compositor::{
    BackendMetadata, CapabilitySet, CompositorAction, CompositorBackend, CompositorCommand,
    CompositorError, CompositorEvent, CompositorSnapshot, ConnectionConfiguration,
};
use serde_json::Value;
use std::{collections::HashMap, env, path::PathBuf, time::Duration};

const WAYFIRE_SOCKET_ENVIRONMENT_VARIABLE: &str = "WAYFIRE_SOCKET";
const EVENT_POLL_TIMEOUT: Duration = Duration::from_millis(100);
const LAUNCHER_BINDING: &str = "<super>";

pub struct WayfireBackend {
    client: Option<WayfireClient>,
    binding_actions: HashMap<u64, CompositorAction>,
    pending_events: Vec<CompositorEvent>,
}

impl WayfireBackend {
    pub fn disconnected() -> Self {
        Self {
            client: None,
            binding_actions: HashMap::new(),
            pending_events: Vec::new(),
        }
    }

    fn client(&self) -> Result<&WayfireClient, CompositorError> {
        self.client
            .as_ref()
            .ok_or_else(|| CompositorError::new("the Wayfire backend is not connected yet"))
    }

    fn client_mut(&mut self) -> Result<&mut WayfireClient, CompositorError> {
        self.client
            .as_mut()
            .ok_or_else(|| CompositorError::new("the Wayfire backend is not connected yet"))
    }

    fn register_action(&mut self, action: CompositorAction) -> Result<(), CompositorError> {
        let binding = match action {
            CompositorAction::ToggleLauncher => LAUNCHER_BINDING,
        };
        let request = MethodCall::new(
            "command/register-binding",
            object([("binding", Value::String(binding.to_string()))]),
        );
        let response = self
            .client_mut()?
            .request(&request)
            .map_err(|error| CompositorError::new(error.to_string()))?;
        let registration: BindingRegistrationResponse = serde_json::from_value(response)
            .map_err(|error| CompositorError::new(format!("invalid binding response: {error}")))?;
        self.binding_actions.insert(registration.binding_id, action);
        Ok(())
    }

    fn compositor_event(&self, message: Value) -> Result<Option<CompositorEvent>, CompositorError> {
        let event: WayfireEvent = serde_json::from_value(message)
            .map_err(|error| CompositorError::new(format!("invalid Wayfire event: {error}")))?;
        if event.event != "command-binding" {
            return Ok(None);
        }
        Ok(event
            .binding_id
            .and_then(|binding_id| self.binding_actions.get(&binding_id).copied())
            .map(CompositorEvent::ActionActivated))
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

    fn connect(&mut self, configuration: &ConnectionConfiguration) -> Result<(), CompositorError> {
        if self.client.is_some() {
            return Ok(());
        }
        let socket_path = configuration
            .socket_path
            .clone()
            .or_else(|| env::var_os(WAYFIRE_SOCKET_ENVIRONMENT_VARIABLE).map(PathBuf::from))
            .ok_or_else(|| {
                CompositorError::new(
                    "WAYFIRE_SOCKET is not set and no Wayfire socket path was configured",
                )
            })?;
        self.client = Some(
            WayfireClient::connect(&socket_path)
                .map_err(|error| CompositorError::new(error.to_string()))?,
        );
        self.pending_events.push(CompositorEvent::Connected);
        Ok(())
    }

    fn snapshot(&self) -> Result<CompositorSnapshot, CompositorError> {
        self.client()?;
        Ok(CompositorSnapshot::default())
    }

    fn dispatch(&mut self, command: CompositorCommand) -> Result<(), CompositorError> {
        match command {
            CompositorCommand::RegisterAction(action) => self.register_action(action),
            unsupported_command => Err(CompositorError::new(format!(
                "the Wayfire backend does not implement {unsupported_command:?} yet"
            ))),
        }
    }

    fn poll_events(&mut self) -> Result<Vec<CompositorEvent>, CompositorError> {
        self.client()?;
        let mut events = self.pending_events.drain(..).collect::<Vec<_>>();
        let message = self
            .client_mut()?
            .poll_event(EVENT_POLL_TIMEOUT)
            .map_err(|error| CompositorError::new(error.to_string()))?;
        if let Some(message) = message
            && let Some(event) = self.compositor_event(message)?
        {
            events.push(event);
        }
        Ok(events)
    }
}
