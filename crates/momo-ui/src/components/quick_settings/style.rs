use daiko::style::{Color, Indent};
use daiko::widgets::text::{TextStyle, TextWrap, Weight};

pub(crate) const SETTINGS_MENU_WIDTH: f32 = 392.0;
pub(crate) const SETTINGS_MENU_EDGE_MARGIN: f32 = 40.0;
pub(crate) const SETTINGS_MENU_TOP_OFFSET: f32 = 96.0;
pub(crate) const SETTINGS_MENU_MIN_HEIGHT: f32 = 160.0;
pub(crate) const SETTINGS_MENU_GAP: f32 = 12.0;
pub(crate) const SETTINGS_MENU_HORIZONTAL_PADDING: f32 = 16.0;
pub(crate) const SETTINGS_MENU_VERTICAL_PADDING: f32 = 18.0;
pub(crate) const SETTINGS_PANEL_BORDER_WIDTH: f32 = 1.0;
pub(crate) const SETTINGS_MENU_INNER_WIDTH: f32 =
    SETTINGS_MENU_WIDTH
        - SETTINGS_MENU_HORIZONTAL_PADDING * 2.0
        - SETTINGS_PANEL_BORDER_WIDTH * 2.0;
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
pub(crate) const SETTINGS_TOP_ACTIONS_GAP: f32 = SETTINGS_MENU_GAP;
pub(crate) const SETTINGS_COMPACT_CONTENT_GAP: f32 = 8.0;
pub(crate) const SETTINGS_STATUS_CHIP_PADDING: Indent = Indent::uniform(10.0);
pub(crate) const SETTINGS_LABEL_TEXT_SIZE: f32 = 18.0;
pub(crate) const SETTINGS_ICON_SIZE: usize = SETTINGS_LABEL_TEXT_SIZE as usize;
pub(crate) const SETTINGS_ICON_FRAME_SIZE: f32 = 20.0;
pub(crate) const SETTINGS_TILE_CONTENT_GAP: f32 = SETTINGS_MENU_GAP;
pub(crate) const SETTINGS_TILE_PADDING: Indent = Indent::uniform(14.0);
pub(crate) const SETTINGS_TILE_TEXT_HEIGHT: f32 = 38.0;
pub(crate) const SETTINGS_SUBMENU_SECTION_LABEL_HEIGHT: f32 = 20.0;
pub(crate) const SETTINGS_SUBMENU_SECTION_TITLE_TEXT_SIZE: f32 = 16.0;
pub(crate) const SETTINGS_SUBMENU_ROW_PADDING: Indent = Indent::new(
    SETTINGS_TILE_PADDING.left,
    0.0,
    SETTINGS_TILE_PADDING.right,
    0.0,
);
pub(crate) const SETTINGS_SUBMENU_BUTTON_PADDING: Indent =
    Indent::new(0.0, 0.0, SETTINGS_TILE_PADDING.right, 0.0);
pub(crate) const SETTINGS_SUBMENU_SECTION_PADDING: Indent = SETTINGS_SUBMENU_ROW_PADDING;
pub(crate) const SETTINGS_SUBMENU_TRAILING_CONTROL_PADDING: f32 = SETTINGS_MENU_GAP * 0.5;
pub(crate) const SETTINGS_SUBMENU_SWITCH_WIDTH: f32 = 56.0;
pub(crate) const SETTINGS_SUBMENU_SWITCH_HEIGHT: f32 = 32.0;
pub(crate) const SETTINGS_SUBMENU_SWITCH_KNOB_SIZE: f32 = 24.0;
pub(crate) const SETTINGS_SUBMENU_SWITCH_INSET: f32 = 4.0;
pub(crate) const SETTINGS_SUBMENU_SWITCH_KNOB_Y: f32 = 4.0;
pub(crate) const SETTINGS_SUBMENU_TOGGLE_PADDING: Indent = Indent::new(
    0.0,
    0.0,
    SETTINGS_SUBMENU_TRAILING_CONTROL_PADDING,
    0.0,
);
pub(crate) const SETTINGS_SUBMENU_DEVICE_ICON_RING_SIZE: f32 = 32.0;
pub(crate) const SETTINGS_SUBMENU_DEVICE_ROW_PADDING: Indent =
    Indent::new(0.0, 0.0, SETTINGS_TILE_PADDING.right, 0.0);
pub(crate) const SETTINGS_VOLUME_TRACK_HEIGHT: f32 = 22.0;
pub(crate) const SETTINGS_VOLUME_THUMB_SIZE: f32 = 24.0;
pub(crate) const SETTINGS_VOLUME_SLIDER_ROW_HEIGHT: f32 = SETTINGS_VOLUME_THUMB_SIZE;

pub(crate) fn settings_panel_color() -> Color {
    Color::from_rgb(12, 16, 18)
}

pub(crate) fn settings_panel_border_color() -> Color {
    Color::from_rgba_unmultiplied(255, 255, 255, 42)
}

pub(crate) fn settings_surface_color() -> Color {
    Color::from_rgb(24, 28, 31)
}

pub(crate) fn settings_surface_hover_color() -> Color {
    Color::from_rgb(38, 42, 46)
}

pub(crate) fn settings_surface_border_color() -> Color {
    Color::from_rgba_unmultiplied(255, 255, 255, 28)
}

pub(crate) fn settings_surface_border_hover_color() -> Color {
    Color::from_rgba_unmultiplied(255, 255, 255, 92)
}

pub(crate) fn settings_surface_muted_color() -> Color {
    Color::from_rgb(92, 96, 101)
}

pub(crate) fn settings_submenu_device_available_surface_color() -> Color {
    Color::from_rgb(50, 54, 60)
}

pub(crate) fn settings_submenu_device_available_border_color() -> Color {
    Color::from_rgba_unmultiplied(255, 255, 255, 34)
}

pub(crate) fn settings_submenu_device_unavailable_surface_color() -> Color {
    Color::from_rgb(34, 38, 42)
}

pub(crate) fn settings_submenu_device_unavailable_border_color() -> Color {
    Color::from_rgba_unmultiplied(255, 255, 255, 16)
}

pub(crate) fn settings_bright_surface_color() -> Color {
    Color::from_rgb(236, 240, 243)
}

pub(crate) fn settings_bright_surface_muted_color() -> Color {
    Color::from_rgb(214, 220, 226)
}

pub(crate) fn settings_bright_surface_border_color() -> Color {
    Color::from_rgba_unmultiplied(255, 255, 255, 138)
}

pub(crate) fn settings_label_text_style(color: Color) -> TextStyle {
    TextStyle::default()
        .with_font_size(SETTINGS_LABEL_TEXT_SIZE)
        .with_weight(Weight::NORMAL)
        .with_font_color(color)
        .with_wrap(TextWrap::NoWrap)
}

pub(crate) fn settings_text_color() -> Color {
    Color::from_rgb(235, 240, 247)
}

pub(crate) fn settings_inverse_text_color() -> Color {
    Color::from_rgb(12, 16, 20)
}

pub(crate) fn settings_status_chip_border_color() -> Color {
    Color::from_rgba_unmultiplied(255, 255, 255, 48)
}

pub(crate) fn settings_accent_color() -> Color {
    Color::from_rgb(104, 79, 140)
}

pub(crate) fn settings_accent_border_color() -> Color {
    Color::from_rgba_unmultiplied(211, 191, 255, 112)
}

pub(crate) fn settings_accent_text_color() -> Color {
    Color::from_rgb(248, 241, 255)
}

pub(crate) fn settings_tile_icon_background_color(is_active: bool) -> Color {
    if is_active {
        Color::from_rgba_unmultiplied(255, 255, 255, 26)
    } else {
        Color::from_rgba_unmultiplied(255, 255, 255, 12)
    }
}

pub(crate) fn settings_tile_icon_color(is_active: bool) -> Color {
    if is_active {
        Color::from_rgb(246, 237, 255)
    } else {
        Color::from_rgb(232, 238, 247)
    }
}

pub(crate) fn settings_danger_surface_hover_color() -> Color {
    Color::from_rgb(92, 32, 43)
}

pub(crate) fn settings_danger_surface_color() -> Color {
    Color::from_rgb(74, 28, 36)
}

pub(crate) fn settings_danger_surface_border_hover_color() -> Color {
    Color::from_rgba_unmultiplied(255, 189, 198, 184)
}

pub(crate) fn settings_danger_surface_border_color() -> Color {
    Color::from_rgba_unmultiplied(255, 160, 174, 72)
}

pub(crate) fn settings_danger_text_color() -> Color {
    Color::from_rgb(255, 231, 235)
}

pub(crate) fn settings_volume_thumb_border_color() -> Color {
    Color::from_rgba_unmultiplied(255, 255, 255, 110)
}
