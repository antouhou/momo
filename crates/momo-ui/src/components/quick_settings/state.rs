use daiko::Id;
use daiko::component::ComponentContext;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum SettingsMenuViewType {
    #[default]
    Main,
    Bluetooth,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SettingsMenuState {
    pub is_open: bool,
    pub just_opened: bool,
    pub opened_from_trigger_press: bool,
    pub is_animating: bool,
    pub last_active_view: SettingsMenuViewType,
    pub active_view: SettingsMenuViewType,
}

impl Default for SettingsMenuState {
    fn default() -> Self {
        Self {
            is_open: false,
            just_opened: false,
            opened_from_trigger_press: false,
            is_animating: false,
            last_active_view: SettingsMenuViewType::Main,
            active_view: SettingsMenuViewType::Main,
        }
    }
}

impl SettingsMenuState {
    pub(crate) fn set_active_view(&mut self, active_view: SettingsMenuViewType) {
        self.last_active_view = self.active_view;
        self.active_view = active_view;
    }

    pub(crate) fn complete_view_focus_handoff(&mut self) {
        self.last_active_view = self.active_view;
    }

    pub(crate) fn reset_active_view_to_main(&mut self) {
        self.last_active_view = SettingsMenuViewType::Main;
        self.active_view = SettingsMenuViewType::Main;
    }
}

pub(crate) const SETTINGS_MENU_STATE_ID: &str = "momo_home_settings_menu_state";
pub(crate) const SETTINGS_VIEW_TRANSITION_ID: &str = "momo_home_settings_view_transition";

pub fn should_render_settings_menu(ctx: &mut ComponentContext) -> bool {
    let state = ctx.use_shared_state(Id::new(SETTINGS_MENU_STATE_ID), SettingsMenuState::default);
    let guard = state.read();
    guard.is_open || guard.is_animating
}
