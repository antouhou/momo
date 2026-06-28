#[derive(Copy, Clone, Default)]
pub(super) struct ClockButtonLocalState {
    pub(crate) lost_focus_due_to_settings_menu_open: bool,
}
