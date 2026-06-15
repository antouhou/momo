mod style;

use self::style::{settings_status_chip_style, status_chip_content_style, status_value_style};
use super::common::{QuickSettingsGlyph, control_state, glyph_element, is_menu_view_active};
use super::state::SettingsMenuViewType;
use super::style::{
    SETTINGS_ICON_FRAME_SIZE, SETTINGS_ICON_SIZE, settings_danger_text_color, settings_text_color,
};
use crate::components::home::system_status::battery_state;
use daiko::Element;
use daiko::component::{Component, ComponentContext};
use daiko::widgets::text::Text;
use system_control::BatteryChargingState;

const BATTERY_0_ICON: &[u8] = include_bytes!("../../../../assets/battery-0.svg");
const BATTERY_2_ICON: &[u8] = include_bytes!("../../../../assets/battery-2.svg");
const BATTERY_3_ICON: &[u8] = include_bytes!("../../../../assets/battery-3.svg");
const BATTERY_4_ICON: &[u8] = include_bytes!("../../../../assets/battery-4.svg");
const BATTERY_5_ICON: &[u8] = include_bytes!("../../../../assets/battery-5.svg");
const PLUG_ICON: &[u8] = include_bytes!("../../../../assets/plug.svg");
const PLUG_BOLT_ICON: &[u8] = include_bytes!("../../../../assets/plug-circle-bolt.svg");
const LOW_BATTERY_THRESHOLD_PERCENTAGE: u8 = 20;
pub(super) const SETTINGS_STATUS_CHIP_TAG: &str = "header-settings-status-chip";

#[derive(Clone, Copy)]
pub(super) struct StatusChip;

impl Component for StatusChip {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let focusable = ctx.focusable();
        let is_active = is_menu_view_active(ctx, SettingsMenuViewType::Main);
        focusable.set_preferred_focus(is_active);
        focusable.set_navigation_enabled(is_active);
        let state = control_state(ctx);

        Element::new()
            .with_tag(SETTINGS_STATUS_CHIP_TAG)
            .with_style(settings_status_chip_style(state, ctx))
            .with_content(status_chip_content(ctx))
    }
}

fn status_chip_content(ctx: &mut ComponentContext) -> Element {
    let battery_state = battery_state(ctx);
    let battery_state = battery_state.read();
    let battery_percentage = battery_state.percentage;
    let charging_state = battery_state.charging_state;
    let label = battery_percentage
        .map(|percentage| format!("{percentage}%"))
        .unwrap_or_else(|| "--%".to_string());
    let content_color = if battery_percentage.is_some_and(is_low_battery) {
        settings_danger_text_color()
    } else {
        settings_text_color()
    };

    Element::new()
        .with_style(status_chip_content_style())
        .with_content(glyph_element(
            QuickSettingsGlyph::Asset(battery_icon(battery_percentage, charging_state)),
            SETTINGS_ICON_SIZE,
            SETTINGS_ICON_FRAME_SIZE,
            content_color,
        ))
        .with_content(Text::new(label).with_style(status_value_style(content_color)))
}

fn battery_icon(
    battery_percentage: Option<u8>,
    charging_state: Option<BatteryChargingState>,
) -> &'static [u8] {
    match charging_state {
        Some(BatteryChargingState::Charging) => PLUG_BOLT_ICON,
        Some(BatteryChargingState::Full | BatteryChargingState::NotCharging) => PLUG_ICON,
        Some(BatteryChargingState::Discharging | BatteryChargingState::Unknown) | None => {
            battery_percentage_icon(battery_percentage)
        }
    }
}

fn battery_percentage_icon(battery_percentage: Option<u8>) -> &'static [u8] {
    match battery_percentage {
        Some(percentage) if percentage < LOW_BATTERY_THRESHOLD_PERCENTAGE => BATTERY_0_ICON,
        Some(20..=39) => BATTERY_2_ICON,
        Some(40..=59) => BATTERY_3_ICON,
        Some(60..=79) => BATTERY_4_ICON,
        Some(_) => BATTERY_5_ICON,
        None => BATTERY_0_ICON,
    }
}

fn is_low_battery(battery_percentage: u8) -> bool {
    battery_percentage < LOW_BATTERY_THRESHOLD_PERCENTAGE
}
