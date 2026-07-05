use crate::SystemControlError;
use crate::feature_state::FeatureState;
use crate::users::{SystemUser, UserListFeatureState};

const MIN_LOGIN_UID: u32 = 1000;
const MAX_LOGIN_UID: u32 = 60000;

#[derive(Clone)]
pub(crate) struct PlatformUserHandle;

impl PlatformUserHandle {
    pub(crate) fn new() -> Result<Self, SystemControlError> {
        Ok(Self)
    }

    pub(crate) fn list_users(&self) -> UserListFeatureState {
        let users = sysinfo::Users::new_with_refreshed_list()
            .list()
            .iter()
            .filter_map(system_user_from_sysinfo_user)
            .filter(is_login_user)
            .collect::<Vec<_>>();

        FeatureState::Ready(users)
    }
}

fn system_user_from_sysinfo_user(user: &sysinfo::User) -> Option<SystemUser> {
    let uid = **user.id();

    Some(SystemUser {
        identifier: uid.to_string(),
        uid,
        username: user.name().to_string(),
        display_name: user.name().to_string(),
    })
}

fn is_login_user(user: &SystemUser) -> bool {
    user.uid >= MIN_LOGIN_UID && user.uid < MAX_LOGIN_UID && !is_reserved_login_name(&user.username)
}

fn is_reserved_login_name(username: &str) -> bool {
    matches!(username, "nobody" | "nfsnobody")
}
