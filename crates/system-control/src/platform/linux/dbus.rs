use std::time::Duration;

use ::dbus::blocking::{Connection, Proxy};

const DBUS_CALL_TIMEOUT: Duration = Duration::from_secs(5);

pub(super) fn system_bus_connection() -> Result<Connection, String> {
    Connection::new_system().map_err(|error| format!("failed to connect to system D-Bus: {error}"))
}

pub(super) fn system_bus_proxy<'connection>(
    connection: &'connection Connection,
    destination: &'static str,
    path: &'static str,
) -> Proxy<'connection, &'connection Connection> {
    connection.with_proxy(destination, path, DBUS_CALL_TIMEOUT)
}
