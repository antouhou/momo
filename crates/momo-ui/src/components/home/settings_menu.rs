use crate::components::home::model::{SCREEN_PADDING, home_top_row_settings_focus_key};
use daiko::Element;
use daiko::Id;
use daiko::Vec2;
use daiko::component::{Component, ComponentContext};
use daiko::navigation::{FocusBoundary, FocusOrigin, NavigationInputAction};
use daiko::style::{Border, BorderRadius, Color, Indent, Stroke, Style};
use daiko::widgets::button::Button;
use daiko::widgets::container::{Container, Fit};
use daiko::widgets::image::{Image, ImageParams, ImageSource, ImageType};
use daiko::widgets::text::{Text, TextStyle, TextWrap};

const SETTINGS_MENU_OFFSET_Y: f32 = 12.0;
const SETTINGS_MENU_TOP_ROW_HEIGHT: f32 = 52.0;
const SETTINGS_MENU_WIDTH: f32 = 220.0;
const SETTINGS_MENU_STATE_ID: &str = "momo_home_settings_menu_state";

#[derive(Clone, Copy, Default)]
struct SettingsMenuState {
    is_open: bool,
    just_opened: bool,
    opened_from_trigger_press: bool,
}

pub(super) fn settings_menu_is_open(ctx: &mut ComponentContext) -> bool {
    let state = ctx.use_shared_state(Id::new(SETTINGS_MENU_STATE_ID), SettingsMenuState::default);
    state.read().is_open
}

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

#[derive(Clone, Copy)]
pub struct SettingsMenuPanel;

impl Component for SettingsMenuPanel {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let mut pointer = ctx.pointer();
        let focus_scope = ctx.focus_scope();
        let state =
            ctx.use_shared_state(Id::new(SETTINGS_MENU_STATE_ID), SettingsMenuState::default);
        let state_snapshot = *state.read();
        let just_opened = state_snapshot.just_opened;

        focus_scope.set_boundary(FocusBoundary::Escape);
        focus_scope.capture_when_contains_focus(&[
            NavigationInputAction::Cancel,
            NavigationInputAction::Back,
        ]);

        if just_opened {
            focus_scope.request_focus(FocusOrigin::Navigation);
        }

        let exit_button = Button::new(ctx, "Exit").with_style(settings_exit_button_style);
        let exit_clicked = exit_button.clicked();
        if exit_clicked {
            ctx.app_context.close_app();
        }
        let close_from_navigation = focus_scope.drain_captured_actions().any(|action| {
            matches!(
                action,
                NavigationInputAction::Cancel | NavigationInputAction::Back
            )
        });
        let close_from_focus_leave =
            !just_opened && focus_scope.just_left() && !pointer.is_pressed_anywhere();
        let should_close = exit_clicked
            || close_from_navigation
            || (!just_opened && pointer.just_clicked_outside())
            || close_from_focus_leave;

        if should_close || just_opened {
            if close_from_navigation && state_snapshot.opened_from_trigger_press {
                ctx.navigation().request_focus_by_key(
                    home_top_row_settings_focus_key(),
                    FocusOrigin::Navigation,
                );
            }

            *state.write() = SettingsMenuState {
                is_open: !should_close,
                just_opened: false,
                opened_from_trigger_press: if should_close {
                    false
                } else {
                    state_snapshot.opened_from_trigger_press
                },
            };
        }

        Element::new()
            .with_tag("header-settings-menu")
            .with_style(settings_menu_style())
            .with_content(
                Container::vertical()
                    .with_fit(Fit::new().at_least_parent_width().at_least_content_height())
                    .with_spacing((12.0, 12.0))
                    .build()
                    .with_content(menu_heading())
                    .with_content(
                        Element::new()
                            .with_tag("header-settings-exit-button")
                            .with_content(exit_button),
                    ),
            )
    }
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

fn settings_menu_style() -> Style {
    Style::new()
        .with_fixed_width(daiko::layout::ItemSize::Points(SETTINGS_MENU_WIDTH))
        .with_padding(16.0)
        .with_direction(daiko::layout::FlexDirection::Column)
        .with_background_color(Color::from_rgb(13, 20, 31))
        .with_border(Border::uniform(Stroke::new(
            1.0,
            Color::from_rgb(72, 93, 124),
        )))
        .with_border_radius(BorderRadius::all(20.0))
}

fn menu_heading() -> Text {
    Text::new("Settings").with_style(
        TextStyle::default()
            .with_font_size(16.0)
            .with_font_color(Color::from_rgb(240, 245, 255))
            .with_wrap(TextWrap::NoWrap),
    )
}

fn settings_exit_button_style(
    button_state: &daiko::widgets::button::state::ButtonState,
    _ctx: &mut ComponentContext,
) -> Style {
    let background = if button_state.is_pressed {
        Color::from_rgb(54, 24, 30)
    } else if button_state.is_hovered || button_state.is_focused {
        Color::from_rgb(74, 31, 40)
    } else {
        Color::from_rgb(62, 26, 34)
    };

    Style::new()
        .with_direction(daiko::layout::FlexDirection::Row)
        .with_align_items(daiko::layout::AlignItems::Center)
        .with_justify_content(daiko::layout::JustifyContent::Center)
        .with_fixed_width(daiko::layout::ItemSize::Percent(1.0))
        .with_padding(Indent::from((14.0, 12.0)))
        .with_background_color(background)
        .with_border(Border::uniform(Stroke::new(
            1.0,
            Color::from_rgb(148, 91, 101),
        )))
        .with_border_radius(BorderRadius::all(14.0))
}
