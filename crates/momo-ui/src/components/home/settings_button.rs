use crate::components::home::header::{
    HEADER_BUTTON_HEIGHT, HEADER_MENU_STATE_ID, HEADER_SETTINGS_BUTTON_WIDTH, HeaderButtonMetrics,
    HeaderButtonState, HeaderMenuState, HeaderMenuTarget, header_button_style,
};
use crate::components::home::model::home_top_row_settings_focus_key;
use crate::components::quick_settings::SETTINGS_MENU_STATE_ID;
use crate::components::quick_settings::state::{SettingsMenuState, SettingsMenuView};
use daiko::Element;
use daiko::Id;
use daiko::component::{Component, ComponentContext};
use daiko::navigation::FocusOrigin;
use daiko::style::{Color, Indent, Style};
use daiko::widgets::image::{Image, ImageParams, ImageSource, ImageType};

#[derive(Clone, Copy)]
pub(super) struct HeaderSettingsTrigger;

impl Component for HeaderSettingsTrigger {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let mut pointer = ctx.pointer();
        let focusable = ctx.focusable();
        let state =
            ctx.use_shared_state(Id::new(SETTINGS_MENU_STATE_ID), SettingsMenuState::default);
        let state_snapshot = *state.read();
        let just_activated = pointer.just_pressed() || focusable.just_activated();
        focusable.set_focus_key(home_top_row_settings_focus_key());

        if pointer.just_entered() || pointer.just_pressed() {
            focusable.request_focus(FocusOrigin::Pointer);
        }

        let menu_state =
            ctx.use_shared_state(Id::new(HEADER_MENU_STATE_ID), HeaderMenuState::default);
        let should_mark_focused = {
            let state = *menu_state.read();
            focusable.is_focused() && state.focused_target != Some(HeaderMenuTarget::Settings)
        };
        if should_mark_focused {
            *menu_state.write() = HeaderMenuState {
                focused_target: Some(HeaderMenuTarget::Settings),
            };
        }

        if (state_snapshot.is_open || !state_snapshot.is_animating) && just_activated {
            let next_is_open = !state_snapshot.is_open;
            *state.write() = SettingsMenuState {
                is_open: next_is_open,
                just_opened: next_is_open,
                opened_from_trigger_press: next_is_open,
                is_animating: true,
                last_active_view: SettingsMenuView::Main,
                active_view: SettingsMenuView::Main,
            };
        }

        let trigger_state = HeaderSettingsTriggerState {
            is_pressed: pointer.is_pressed(),
            is_hovered: pointer.is_hovering(),
            is_focused: focusable.is_focused(),
        };
        let icon_color =
            if trigger_state.is_pressed || trigger_state.is_hovered || trigger_state.is_focused {
                Color::from_rgb(10, 13, 18)
            } else {
                Color::from_rgb(232, 238, 250)
            };

        Element::new()
            .with_tag("header-settings-button")
            .with_style(settings_trigger_button_style(ctx, &trigger_state))
            .with_content(trigger_label(icon_color))
    }
}

#[derive(Clone, Copy)]
struct HeaderSettingsTriggerState {
    is_pressed: bool,
    is_hovered: bool,
    is_focused: bool,
}

fn trigger_label(color: Color) -> Image {
    Image::new(ImageParams {
        max_width: 22,
        max_height: 22,
        image_type: Some(ImageType::Svg),
        source: ImageSource::BytesSlice(include_bytes!("../../../assets/gear-solid-full.svg")),
    })
    .fill_color(Some(color))
}

fn settings_trigger_button_style(
    ctx: &mut ComponentContext,
    trigger_state: &HeaderSettingsTriggerState,
) -> Style {
    let active_state = HeaderButtonState {
        is_active: trigger_state.is_hovered || trigger_state.is_focused || trigger_state.is_pressed,
        is_pressed: trigger_state.is_pressed,
        is_hovered: trigger_state.is_hovered,
        is_focused: trigger_state.is_focused,
    };
    let style = header_button_style(
        ctx,
        HeaderButtonMetrics {
            width: HEADER_SETTINGS_BUTTON_WIDTH,
            height: HEADER_BUTTON_HEIGHT,
            radius: HEADER_BUTTON_HEIGHT / 2.0,
        },
        active_state,
        false,
    );

    style.with_padding(Indent::from((0.0, 0.0)))
}
