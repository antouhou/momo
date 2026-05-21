use daiko::style::Indent;

pub(crate) const SETTINGS_MENU_WIDTH: f32 = 392.0;
pub(crate) const SETTINGS_MENU_EDGE_MARGIN: f32 = 40.0;
pub(crate) const SETTINGS_MENU_TOP_OFFSET: f32 = 96.0;
pub(crate) const SETTINGS_MENU_GAP: f32 = 12.0;
pub(crate) const SETTINGS_MENU_HORIZONTAL_PADDING: f32 = 16.0;
pub(crate) const SETTINGS_MENU_VERTICAL_PADDING: f32 = 18.0;
pub(crate) const SETTINGS_MENU_INNER_WIDTH: f32 =
    SETTINGS_MENU_WIDTH - SETTINGS_MENU_HORIZONTAL_PADDING * 2.0;
pub(crate) const SETTINGS_MENU_SLIDE_DISTANCE: f32 =
    SETTINGS_MENU_WIDTH + SETTINGS_MENU_EDGE_MARGIN + 36.0;
pub(crate) const SETTINGS_ROUND_BUTTON_SIZE: f32 = 44.0;
pub(crate) const SETTINGS_STATUS_CHIP_WIDTH: f32 = 92.0;
pub(crate) const SETTINGS_STATUS_CHIP_HEIGHT: f32 = 44.0;
pub(crate) const SETTINGS_TILE_WIDTH: f32 = 174.0;
pub(crate) const SETTINGS_TILE_HEIGHT: f32 = 76.0;

pub(crate) const PANEL_RADIUS: f32 = 30.0;
pub(crate) const CONTROL_RADIUS: f32 = 22.0;
pub(crate) const TILE_RADIUS: f32 = 20.0;
pub(crate) const CONTROL_TRANSITION_MS: u64 = 120;
pub(crate) const SETTINGS_MENU_PADDING: Indent = Indent::new(
    SETTINGS_MENU_HORIZONTAL_PADDING,
    SETTINGS_MENU_VERTICAL_PADDING,
    SETTINGS_MENU_HORIZONTAL_PADDING,
    SETTINGS_MENU_VERTICAL_PADDING,
);
pub(crate) const SETTINGS_TOP_ACTIONS_GAP: f32 = 10.0;
pub(crate) const SETTINGS_STATUS_CHIP_CONTENT_GAP: f32 = 8.0;
pub(crate) const SETTINGS_STATUS_CHIP_PADDING: Indent = Indent::uniform(10.0);
pub(crate) const SETTINGS_TILE_CONTENT_GAP: f32 = 12.0;
pub(crate) const SETTINGS_TILE_PADDING: Indent = Indent::uniform(14.0);
pub(crate) const SETTINGS_TILE_TEXT_HEIGHT: f32 = 38.0;
