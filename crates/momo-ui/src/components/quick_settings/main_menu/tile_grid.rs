use daiko::{
    Element, Id,
    component::{Component, ComponentContext},
    navigation::{FocusEntryPolicy, FocusKey},
};
use tracing::warn;
use super::{
    super::{
        settings_tile_button::{SettingsTileButton, SettingsTileSpec},
        state::{SETTINGS_MENU_STATE_ID, SettingsMenuState, SettingsMenuViewType},
    },
    style::{settings_tile_grid_style, settings_tile_row_style},
    tile_specs::{BLUETOOTH_TILE_FOCUS_KEY_ID, TILE_ROWS},
};
use crate::components::home::bluetooth::{bluetooth_handle, bluetooth_state};

#[derive(Clone, Copy)]
pub(super) struct SettingsTileGrid;

impl Component for SettingsTileGrid {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let focus_scope = ctx.focus_scope();
        focus_scope.set_entry_policy(FocusEntryPolicy::Remembered);

        let state =
            ctx.use_shared_state(Id::new(SETTINGS_MENU_STATE_ID), SettingsMenuState::default);
        let is_main_view = state.read().active_view == SettingsMenuViewType::Main;
        focus_scope.set_navigation_enabled(is_main_view);
        let bluetooth_is_enabled = bluetooth_state(ctx).read().is_enabled;
        let should_restore_bluetooth_focus = should_restore_bluetooth_tile_focus(ctx);

        complete_bluetooth_focus_handoff(ctx, focus_scope.focused_child_key());

        let mut grid = Element::new().with_style(settings_tile_grid_style());

        for row in TILE_ROWS {
            let mut row_element = Element::new().with_style(settings_tile_row_style());
            for tile in row {
                let mut button =
                    SettingsTileButton::new(ctx, tile, is_tile_active(tile, bluetooth_is_enabled));
                if should_restore_bluetooth_focus
                    && tile.focus_key_id == BLUETOOTH_TILE_FOCUS_KEY_ID
                {
                    button.request_focus();
                }
                handle_tile_activation(ctx, tile, button.activated(), is_main_view);
                row_element.add_content(button);
            }
            grid.add_content(row_element);
        }

        grid
    }
}

fn should_restore_bluetooth_tile_focus(ctx: &mut ComponentContext) -> bool {
    let state = ctx.use_shared_state(Id::new(SETTINGS_MENU_STATE_ID), SettingsMenuState::default);
    {
        let state = state.read();
        state.last_active_view == SettingsMenuViewType::Bluetooth
            && state.active_view == SettingsMenuViewType::Main
    }
}

fn complete_bluetooth_focus_handoff(
    ctx: &mut ComponentContext,
    focused_child_key: Option<FocusKey>,
) {
    if focused_child_key != Some(bluetooth_tile_focus_key()) {
        return;
    }

    let state = ctx.use_shared_state(Id::new(SETTINGS_MENU_STATE_ID), SettingsMenuState::default);
    let is_bluetooth_focus_handoff_pending = {
        let state = state.read();
        state.last_active_view == SettingsMenuViewType::Bluetooth
            && state.active_view == SettingsMenuViewType::Main
    };

    if is_bluetooth_focus_handoff_pending {
        state.write_silent().complete_view_focus_handoff();
    }
}

fn handle_tile_activation(
    ctx: &mut ComponentContext,
    tile: SettingsTileSpec,
    was_activated: bool,
    is_main_view: bool,
) {
    if !is_main_view || !was_activated {
        return;
    }

    if tile.focus_key_id == BLUETOOTH_TILE_FOCUS_KEY_ID {
        open_bluetooth_submenu(ctx);
    }
}

fn open_bluetooth_submenu(ctx: &mut ComponentContext) {
    if bluetooth_state(ctx).read().is_enabled
        && let Err(error) = bluetooth_handle(ctx).start_discovery()
    {
        warn!("failed to start Bluetooth discovery: {error:?}");
    }

    let state = ctx.use_shared_state(Id::new(SETTINGS_MENU_STATE_ID), SettingsMenuState::default);
    state
        .write()
        .set_active_view(SettingsMenuViewType::Bluetooth);
}

fn is_tile_active(tile: SettingsTileSpec, bluetooth_is_enabled: bool) -> bool {
    if tile.focus_key_id == BLUETOOTH_TILE_FOCUS_KEY_ID {
        bluetooth_is_enabled
    } else {
        tile.is_active
    }
}

fn bluetooth_tile_focus_key() -> FocusKey {
    FocusKey::new(BLUETOOTH_TILE_FOCUS_KEY_ID)
}
