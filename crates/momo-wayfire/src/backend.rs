use momo_compositor::{
    BackendMetadata, CapabilitySet, CompositorBackend, CompositorCommand, CompositorError,
    CompositorEvent, CompositorSnapshot, ConnectionConfiguration,
};

pub struct WayfireBackend {
    connected: bool,
}

impl WayfireBackend {
    pub fn disconnected() -> Self {
        Self { connected: false }
    }
}

impl CompositorBackend for WayfireBackend {
    fn metadata(&self) -> BackendMetadata {
        BackendMetadata { name: "wayfire" }
    }

    fn capabilities(&self) -> CapabilitySet {
        CapabilitySet {
            workspace_control: true,
            view_management: true,
            output_management: false,
            plugin_activation: true,
        }
    }

    fn connect(&mut self, _configuration: &ConnectionConfiguration) -> Result<(), CompositorError> {
        self.connected = true;
        Ok(())
    }

    fn snapshot(&self) -> Result<CompositorSnapshot, CompositorError> {
        if self.connected {
            Ok(CompositorSnapshot::default())
        } else {
            Err(CompositorError::new(
                "the Wayfire backend is not connected yet",
            ))
        }
    }

    fn dispatch(&mut self, _command: CompositorCommand) -> Result<(), CompositorError> {
        if self.connected {
            Ok(())
        } else {
            Err(CompositorError::new(
                "cannot dispatch Wayfire commands before connecting",
            ))
        }
    }

    fn poll_events(&mut self) -> Result<Vec<CompositorEvent>, CompositorError> {
        if self.connected {
            Ok(vec![CompositorEvent::Connected])
        } else {
            Ok(Vec::new())
        }
    }
}
