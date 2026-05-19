use daiko::Id;
use daiko::component::ComponentContext;

#[derive(Clone, Copy, Default)]
pub struct SettingsMenuState {
    pub is_open: bool,
    pub just_opened: bool,
    pub opened_from_trigger_press: bool,
}

pub(crate) const SETTINGS_MENU_STATE_ID: &str = "momo_home_settings_menu_state";

pub fn is_settings_menu_open(ctx: &mut ComponentContext) -> bool {
    let state = ctx.use_shared_state(Id::new(SETTINGS_MENU_STATE_ID), SettingsMenuState::default);
    state.read().is_open
}
