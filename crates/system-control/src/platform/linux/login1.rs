use super::dbus::{system_bus_connection, system_bus_proxy};
use crate::power::PowerAction;
use crate::session::SessionAction;
use std::fs;

const LOGIN1_DESTINATION: &str = "org.freedesktop.login1";
const LOGIN1_PATH: &str = "/org/freedesktop/login1";
const LOGIN1_MANAGER_INTERFACE: &str = "org.freedesktop.login1.Manager";
const INTERACTIVE_AUTHORIZATION: bool = true;

pub(super) struct Login1Client {
    connection: ::dbus::blocking::Connection,
}

impl Login1Client {
    pub(super) fn new() -> Result<Self, String> {
        Ok(Self {
            connection: system_bus_connection()?,
        })
    }

    pub(super) fn request_power_action(&self, action: PowerAction) -> Result<(), String> {
        let method = power_method_name(action);
        let proxy = self.proxy();
        let _: () = proxy
            .method_call(
                LOGIN1_MANAGER_INTERFACE,
                method,
                (INTERACTIVE_AUTHORIZATION,),
            )
            .map_err(|error| format!("login1 {method} failed: {error}"))?;
        Ok(())
    }

    pub(super) fn request_session_action(&self, action: SessionAction) -> Result<(), String> {
        match action {
            SessionAction::LogOut => self.log_out(),
        }
    }

    fn log_out(&self) -> Result<(), String> {
        if let Some(session_id) = non_empty_env("XDG_SESSION_ID") {
            return self.terminate_session(&session_id);
        }

        self.terminate_current_user()
    }

    fn terminate_session(&self, session_id: &str) -> Result<(), String> {
        let proxy = self.proxy();
        let _: () = proxy
            .method_call(LOGIN1_MANAGER_INTERFACE, "TerminateSession", (session_id,))
            .map_err(|error| format!("login1 TerminateSession failed: {error}"))?;
        Ok(())
    }

    fn terminate_current_user(&self) -> Result<(), String> {
        let uid = current_uid()?;
        let proxy = self.proxy();
        let _: () = proxy
            .method_call(LOGIN1_MANAGER_INTERFACE, "TerminateUser", (uid,))
            .map_err(|error| format!("login1 TerminateUser failed: {error}"))?;
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

fn current_uid() -> Result<u32, String> {
    let status = fs::read_to_string("/proc/self/status")
        .map_err(|error| format!("failed to read /proc/self/status: {error}"))?;
    let uid_line = status
        .lines()
        .find(|line| line.starts_with("Uid:"))
        .ok_or_else(|| "failed to find current uid in /proc/self/status".to_string())?;
    let uid = uid_line
        .split_whitespace()
        .nth(1)
        .ok_or_else(|| "failed to parse current uid from /proc/self/status".to_string())?;

    uid.parse()
        .map_err(|error| format!("failed to parse current uid from /proc/self/status: {error}"))
}
