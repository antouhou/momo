use std::time::Duration;

use ::dbus::blocking::{Connection, Proxy};
use thiserror::Error;

const DBUS_CALL_TIMEOUT: Duration = Duration::from_secs(5);

#[derive(Debug, Error)]
pub(super) enum SystemBusError {
    #[error("failed to connect to system D-Bus")]
    ConnectionFailed {
        #[source]
        source: ::dbus::Error,
    },
}

pub(super) fn system_bus_connection() -> Result<Connection, SystemBusError> {
    Connection::new_system().map_err(|source| SystemBusError::ConnectionFailed { source })
}

pub(super) fn system_bus_proxy<'connection>(
    connection: &'connection Connection,
    destination: &'static str,
    path: &'static str,
) -> Proxy<'connection, &'connection Connection> {
    connection.with_proxy(destination, path, DBUS_CALL_TIMEOUT)
}
