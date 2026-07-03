#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum UserProfile {
    Anton,
    Maya,
    Guest,
}

impl UserProfile {
    pub(super) const fn name(self) -> &'static str {
        match self {
            Self::Anton => "Anton",
            Self::Maya => "Maya",
            Self::Guest => "Guest",
        }
    }

    pub(super) const fn initials(self) -> &'static str {
        match self {
            Self::Anton => "A",
            Self::Maya => "M",
            Self::Guest => "G",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ProfileAction {
    Login(UserProfile),
    AddUser,
}

pub(super) const PROFILE_ACTIONS: &[ProfileAction] = &[
    ProfileAction::Login(UserProfile::Anton),
    ProfileAction::Login(UserProfile::Maya),
    ProfileAction::Login(UserProfile::Guest),
    ProfileAction::AddUser,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum GreeterView {
    Profiles,
    Credentials(UserProfile),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct GreeterState {
    pub(super) view: GreeterView,
}

impl Default for GreeterState {
    fn default() -> Self {
        Self {
            view: GreeterView::Profiles,
        }
    }
}
