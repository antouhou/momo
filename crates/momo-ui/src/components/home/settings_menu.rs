use crate::components::home::model::home_top_row_settings_focus_key;
use crate::components::quick_settings::SETTINGS_MENU_STATE_ID;
use crate::components::quick_settings::state::SettingsMenuState;
use daiko::Element;
use daiko::Id;
use daiko::component::{Component, ComponentContext};
use daiko::navigation::FocusOrigin;
use daiko::style::{Border, BorderRadius, Color, Indent, Stroke, Style};
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
        let just_activated = pointer.just_clicked() || focusable.just_activated();
        focusable.set_focus_key(home_top_row_settings_focus_key());

        if pointer.just_pressed() {
            focusable.request_focus(FocusOrigin::Pointer);
        }

        if just_activated {
            let next_is_open = !state_snapshot.is_open;
            *state.write() = SettingsMenuState {
                is_open: next_is_open,
                just_opened: next_is_open,
                opened_from_trigger_press: next_is_open,
            };
        }

        let trigger_state = HeaderSettingsTriggerState {
            is_pressed: pointer.is_pressed(),
            is_hovered: pointer.is_hovering(),
            is_focused: focusable.is_focus_visible(),
        };

        Element::new()
            .with_tag("header-settings-button")
            .with_style(settings_trigger_button_style(&trigger_state))
            .with_content(trigger_label())
    }
}

#[derive(Clone, Copy)]
struct HeaderSettingsTriggerState {
    is_pressed: bool,
    is_hovered: bool,
    is_focused: bool,
}

fn trigger_label() -> Image {
    Image::new(ImageParams {
        max_width: 24,
        max_height: 24,
        image_type: Some(ImageType::Svg),
        source: ImageSource::BytesSlice(include_bytes!("../../../assets/gear-solid-full.svg")),
    })
    .fill_color(Some(Color::from_rgb(232, 238, 250)))
}

fn settings_trigger_button_style(trigger_state: &HeaderSettingsTriggerState) -> Style {
    let background = if trigger_state.is_pressed {
        Color::from_rgb(42, 54, 76)
    } else if trigger_state.is_hovered || trigger_state.is_focused {
        Color::from_rgb(33, 45, 66)
    } else {
        Color::from_rgb(22, 31, 48)
    };
    let border_color = if trigger_state.is_hovered || trigger_state.is_focused {
        Color::from_rgb(112, 141, 189)
    } else {
        Color::from_rgb(64, 81, 110)
    };

    Style::new()
        .with_direction(daiko::layout::FlexDirection::Row)
        .with_align_items(daiko::layout::AlignItems::Center)
        .with_justify_content(daiko::layout::JustifyContent::Center)
        .with_padding(Indent::from((14.0, 12.0)))
        .with_background_color(background)
        .with_border(Border::uniform(Stroke::new(1.0, border_color)))
        .with_border_radius(BorderRadius::all(16.0))
}
