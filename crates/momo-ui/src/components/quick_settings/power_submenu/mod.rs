mod style;

use self::style::power_actions_style;
use super::common::{
    QuickSettingsControlState, QuickSettingsGlyph, settings_bottom_row, settings_row,
};
use super::state::SettingsMenuViewType;
use super::style::{
    settings_content_container_style, settings_danger_text_color, settings_text_color,
};
use super::submenu::{SubmenuBackButton, handle_submenu_back_navigation};
use super::submenu_button::{
    SubmenuButton, SubmenuButtonState, SubmenuButtonSurface, submenu_button_glyph,
};
use crate::components::home::power::power_handle;
use crate::components::home::session::session_handle;
use daiko::Element;
use daiko::component::{Component, ComponentContext};
use momo_kit::assets::POWER_ICON;
use momo_kit::interaction::ButtonBehavior;
use system_control::{PowerAction, SessionAction};
use tracing::warn;

const MOON_ICON: &[u8] = include_bytes!("../../../../assets/moon.svg");
const REBOOT_ICON: &[u8] = include_bytes!("../../../../assets/plug-circle-bolt.svg");
const LOG_OUT_ICON: &[u8] = include_bytes!("../../../../assets/log-out.svg");

pub(super) const POWER_BACK_BUTTON_TAG: &str = "header-settings-power-back-button";
pub(super) const POWER_SUBMENU_TAG: &str = "header-settings-power-submenu";
pub(super) const POWER_SHUTDOWN_BUTTON_TAG: &str = "header-settings-power-shutdown-button";
pub(super) const POWER_SUSPEND_BUTTON_TAG: &str = "header-settings-power-suspend-button";
pub(super) const POWER_REBOOT_BUTTON_TAG: &str = "header-settings-power-reboot-button";
pub(super) const POWER_LOG_OUT_BUTTON_TAG: &str = "header-settings-power-log-out-button";

#[derive(Clone, Copy)]
pub(super) struct PowerSubmenu;

impl Component for PowerSubmenu {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        handle_submenu_back_navigation(ctx, SettingsMenuViewType::Power);

        Element::new()
            .with_tag(POWER_SUBMENU_TAG)
            .with_style(settings_content_container_style())
            .with_content(settings_row(SubmenuBackButton {
                tag: POWER_BACK_BUTTON_TAG,
                current_view: SettingsMenuViewType::Power,
            }))
            .with_content(settings_bottom_row(PowerActions))
    }
}

#[derive(Clone, Copy)]
struct PowerActionSpec {
    tag: &'static str,
    label: &'static str,
    glyph: QuickSettingsGlyph,
    is_danger: bool,
    action: PowerMenuAction,
}

#[derive(Clone, Copy)]
enum PowerMenuAction {
    Power(PowerAction),
    Session(SessionAction),
}

const POWER_ACTIONS: [PowerActionSpec; 4] = [
    PowerActionSpec {
        tag: POWER_SHUTDOWN_BUTTON_TAG,
        label: "Shut down",
        glyph: QuickSettingsGlyph::Asset(POWER_ICON),
        is_danger: true,
        action: PowerMenuAction::Power(PowerAction::Shutdown),
    },
    PowerActionSpec {
        tag: POWER_SUSPEND_BUTTON_TAG,
        label: "Suspend",
        glyph: QuickSettingsGlyph::Asset(MOON_ICON),
        is_danger: false,
        action: PowerMenuAction::Power(PowerAction::Suspend),
    },
    PowerActionSpec {
        tag: POWER_REBOOT_BUTTON_TAG,
        label: "Reboot",
        glyph: QuickSettingsGlyph::Asset(REBOOT_ICON),
        is_danger: false,
        action: PowerMenuAction::Power(PowerAction::Reboot),
    },
    PowerActionSpec {
        tag: POWER_LOG_OUT_BUTTON_TAG,
        label: "Log out",
        glyph: QuickSettingsGlyph::Asset(LOG_OUT_ICON),
        is_danger: false,
        action: PowerMenuAction::Session(SessionAction::LogOut),
    },
];

#[derive(Clone, Copy)]
struct PowerActions;

impl Component for PowerActions {
    fn to_element(&self, _ctx: &mut ComponentContext) -> Element {
        let mut actions = Element::new().with_style(power_actions_style());

        for spec in POWER_ACTIONS {
            actions.add_content(PowerActionButton { spec });
        }

        actions
    }
}

#[derive(Clone, Copy)]
struct PowerActionButton {
    spec: PowerActionSpec,
}

impl Component for PowerActionButton {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let button = ButtonBehavior::new(ctx).apply();

        if button.just_activated {
            handle_power_menu_action(ctx, self.spec.action);
        }

        let foreground_color = if self.spec.is_danger {
            settings_danger_text_color()
        } else {
            settings_text_color()
        };

        SubmenuButton {
            tag: self.spec.tag.to_string(),
            label: self.spec.label.to_string(),
            label_color: Some(foreground_color),
            control: QuickSettingsControlState {
                is_hovered: button.is_hovering,
                is_focused: button.is_focused,
            },
            surface: SubmenuButtonSurface::Standard,
            state: SubmenuButtonState::Enabled,
            leading: submenu_button_glyph(self.spec.glyph, foreground_color),
            trailing: None,
        }
        .to_element(ctx)
    }
}

fn handle_power_menu_action(ctx: &mut ComponentContext, action: PowerMenuAction) {
    match action {
        PowerMenuAction::Power(action) => {
            if let Err(error) = power_handle(ctx).request(action) {
                warn!("failed to request power action {action:?}: {error}");
            }
        }
        PowerMenuAction::Session(action) => {
            if let Err(error) = session_handle(ctx).request(action) {
                warn!("failed to request session action {action:?}: {error}");
            }
        }
    }
}
