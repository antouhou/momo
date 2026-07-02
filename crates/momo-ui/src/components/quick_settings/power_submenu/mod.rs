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
use daiko::Element;
use daiko::component::{Component, ComponentContext};
use daiko::navigation::FocusOrigin;

const POWER_ICON: &[u8] = include_bytes!("../../../../assets/power.svg");
const MOON_ICON: &[u8] = include_bytes!("../../../../assets/moon.svg");
const REBOOT_ICON: &[u8] = include_bytes!("../../../../assets/plug-circle-bolt.svg");

pub(super) const POWER_BACK_BUTTON_TAG: &str = "header-settings-power-back-button";
pub(super) const POWER_SUBMENU_TAG: &str = "header-settings-power-submenu";
pub(super) const POWER_SHUTDOWN_BUTTON_TAG: &str = "header-settings-power-shutdown-button";
pub(super) const POWER_SUSPEND_BUTTON_TAG: &str = "header-settings-power-suspend-button";
pub(super) const POWER_REBOOT_BUTTON_TAG: &str = "header-settings-power-reboot-button";

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
}

const POWER_ACTIONS: [PowerActionSpec; 3] = [
    PowerActionSpec {
        tag: POWER_SHUTDOWN_BUTTON_TAG,
        label: "Shut down",
        glyph: QuickSettingsGlyph::Asset(POWER_ICON),
        is_danger: true,
    },
    PowerActionSpec {
        tag: POWER_SUSPEND_BUTTON_TAG,
        label: "Suspend",
        glyph: QuickSettingsGlyph::Asset(MOON_ICON),
        is_danger: false,
    },
    PowerActionSpec {
        tag: POWER_REBOOT_BUTTON_TAG,
        label: "Reboot",
        glyph: QuickSettingsGlyph::Asset(REBOOT_ICON),
        is_danger: false,
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
        let mut pointer = ctx.pointer();
        let focusable = ctx.focusable();

        if pointer.just_pressed() {
            focusable.request_focus(FocusOrigin::Pointer);
        }

        if pointer.just_pressed() || focusable.just_activated() {
            println!("Power action selected: {}", self.spec.label);
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
                is_hovered: pointer.is_hovering(),
                is_focused: focusable.is_focused(),
            },
            surface: SubmenuButtonSurface::Standard,
            state: SubmenuButtonState::Enabled,
            leading: submenu_button_glyph(self.spec.glyph, foreground_color),
            trailing: None,
        }
        .to_element(ctx)
    }
}
