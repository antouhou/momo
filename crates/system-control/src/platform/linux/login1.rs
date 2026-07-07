use super::dbus::{SystemBusError, system_bus_connection, system_bus_proxy};
use crate::{power::PowerAction, session::SessionAction};
use std::{fs, io, num::ParseIntError};
use thiserror::Error;

const LOGIN1_DESTINATION: &str = "org.freedesktop.login1";
const LOGIN1_PATH: &str = "/org/freedesktop/login1";
const LOGIN1_MANAGER_INTERFACE: &str = "org.freedesktop.login1.Manager";
const INTERACTIVE_AUTHORIZATION: bool = true;

pub(super) struct Login1Client {
    connection: ::dbus::blocking::Connection,
}

#[derive(Debug, Error)]
pub(super) enum Login1Error {
    #[error(transparent)]
    SystemBus(#[from] SystemBusError),
    #[error("failed to request {action:?} through login1")]
    PowerActionRequest {
        action: PowerAction,
        #[source]
        source: ::dbus::Error,
    },
    #[error("failed to terminate login1 session")]
    TerminateSession {
        #[source]
        source: ::dbus::Error,
    },
    #[error("failed to terminate login1 user")]
    TerminateUser {
        #[source]
        source: ::dbus::Error,
    },
    #[error("failed to read current process status")]
    CurrentProcessStatusRead {
        #[source]
        source: io::Error,
    },
    #[error("current process status does not contain a Uid field")]
    CurrentUidMissing,
    #[error("current process status Uid field is missing the effective uid")]
    CurrentUidMalformed,
    #[error("failed to parse current effective uid")]
    CurrentUidParse {
        #[source]
        source: ParseIntError,
    },
}

impl Login1Client {
    pub(super) fn new() -> Result<Self, Login1Error> {
        Ok(Self {
            connection: system_bus_connection()?,
        })
    }

    pub(super) fn request_power_action(&self, action: PowerAction) -> Result<(), Login1Error> {
        let method = power_method_name(action);
        let proxy = self.proxy();
        let _: () = proxy
            .method_call(
                LOGIN1_MANAGER_INTERFACE,
                method,
                (INTERACTIVE_AUTHORIZATION,),
            )
            .map_err(|source| Login1Error::PowerActionRequest { action, source })?;
        Ok(())
    }

    pub(super) fn request_session_action(&self, action: SessionAction) -> Result<(), Login1Error> {
        match action {
            SessionAction::LogOut => self.log_out(),
        }
    }

    fn log_out(&self) -> Result<(), Login1Error> {
        if let Some(session_id) = non_empty_env("XDG_SESSION_ID") {
            return self.terminate_session(&session_id);
        }

        self.terminate_current_user()
    }

    fn terminate_session(&self, session_id: &str) -> Result<(), Login1Error> {
        let proxy = self.proxy();
        let _: () = proxy
            .method_call(LOGIN1_MANAGER_INTERFACE, "TerminateSession", (session_id,))
            .map_err(|source| Login1Error::TerminateSession { source })?;
        Ok(())
    }

    fn terminate_current_user(&self) -> Result<(), Login1Error> {
        let uid = current_uid()?;
        let proxy = self.proxy();
        let _: () = proxy
            .method_call(LOGIN1_MANAGER_INTERFACE, "TerminateUser", (uid,))
            .map_err(|source| Login1Error::TerminateUser { source })?;
        Ok(())
    }

    fn proxy(&self) -> ::dbus::blocking::Proxy<'_, &::dbus::blocking::Connection> {
        system_bus_proxy(&self.connection, LOGIN1_DESTINATION, LOGIN1_PATH)
    }
}

fn power_method_name(action: PowerAction) -> &'static str {
    match action {
        PowerAction::Shutdown => "PowerOff",
        PowerAction::Suspend => "Suspend",
        PowerAction::Reboot => "Reboot",
    }
}

fn non_empty_env(key: &str) -> Option<String> {
    let value = std::env::var(key).ok()?;
    (!value.is_empty()).then_some(value)
}

fn current_uid() -> Result<u32, Login1Error> {
    let status = fs::read_to_string("/proc/self/status")
        .map_err(|source| Login1Error::CurrentProcessStatusRead { source })?;
    let uid_line = status
        .lines()
        .find(|line| line.starts_with("Uid:"))
        .ok_or(Login1Error::CurrentUidMissing)?;
    let uid = uid_line
        .split_whitespace()
        .nth(1)
        .ok_or(Login1Error::CurrentUidMalformed)?;

    uid.parse()
        .map_err(|source| Login1Error::CurrentUidParse { source })
}
