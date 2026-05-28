use daiko::Id;
use daiko::component::ComponentContext;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SettingsMenuView {
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
    pub last_active_view: SettingsMenuView,
    pub active_view: SettingsMenuView,
}

impl Default for SettingsMenuState {
    fn default() -> Self {
        Self {
            is_open: false,
            just_opened: false,
            opened_from_trigger_press: false,
            is_animating: false,
            last_active_view: SettingsMenuView::Main,
            active_view: SettingsMenuView::Main,
        }
    }
}

pub(crate) const SETTINGS_MENU_STATE_ID: &str = "momo_home_settings_menu_state";

pub fn should_render_settings_menu(ctx: &mut ComponentContext) -> bool {
    let state = ctx.use_shared_state(Id::new(SETTINGS_MENU_STATE_ID), SettingsMenuState::default);
    let snapshot = *state.read();
    snapshot.is_open || snapshot.is_animating
}
