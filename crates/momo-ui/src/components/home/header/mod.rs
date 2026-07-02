use crate::components::home::clock_chip::ClockChip;
use crate::components::home::model::SCREEN_PADDING;
// use crate::components::login_screen::settings_button::HeaderSettingsTrigger;
use daiko::Element;
use daiko::animation::easing::EasingFunction;
use daiko::animation::{AnimationParameters, transition};
use daiko::component::{Child, Component, ComponentContext, IntoChild};
use daiko::layout::{AlignItems, FlexDirection, ItemSize, JustifyContent, SizeConstraint};
use daiko::navigation::{FocusEntryPolicy, TraversalPolicy};
use daiko::style::{Border, BorderRadius, Color, CursorIcon, Indent, Stroke, Style};
use std::time::Duration;

// pub(super) const HEADER_MENU_STATE_ID: &str = "momo_home_header_menu_state";
pub(super) const HEADER_MENU_HEIGHT: f32 = 44.0;
// pub(super) const HEADER_MENU_RADIUS: f32 = 22.0;
pub(super) const HEADER_BUTTON_HEIGHT: f32 = 38.0;
pub(super) const HEADER_BUTTON_RADIUS: f32 = 19.0;
// pub(super) const HEADER_ACTIVE_SCALE: f32 = 1.06;
// pub(super) const HEADER_FOCUS_SCALE: f32 = 1.18;
pub(super) const HEADER_TRANSITION_MS: u64 = 100;
pub(super) const HEADER_CLOCK_WIDTH: f32 = 104.0;
// pub(super) const HEADER_APPS_BUTTON_WIDTH: f32 = 96.0;
// pub(super) const HEADER_SETTINGS_BUTTON_WIDTH: f32 = HEADER_BUTTON_HEIGHT;
// const HEADER_MENU_GAP: f32 = 4.0;
// const HEADER_MENU_PADDING: f32 = 4.0;
// const HEADER_MENU_BORDER_WIDTH: f32 = 1.0;
// const HEADER_INDICATOR_TRANSITION_MS: u64 = 140;

// #[derive(Clone, Copy, PartialEq, Eq)]
// pub(super) enum HeaderMenuTarget {
//     Apps,
//     Settings,
// }
//
// #[derive(Clone, Copy, Default, PartialEq, Eq)]
// pub(super) struct HeaderMenuState {
//     pub focused_target: Option<HeaderMenuTarget>,
// }

pub(super) struct HomeHeader {
    center: Child,
}

impl HomeHeader {
    pub fn new(center: impl IntoChild) -> Self {
        Self {
            center: center.into_child(),
        }
    }
}

impl Component for HomeHeader {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let focus_scope = ctx.focus_scope();
        focus_scope.set_entry_policy(FocusEntryPolicy::Spatial(
            TraversalPolicy::RectilinearDistance,
        ));

        Element::new()
            .with_tag("apps-header")
            .with_style(header_style())
            .with_content(
                Element::new()
                    .with_tag("apps-header-row")
                    .with_style(header_row_style())
                    // TODO: left container placeholder
                    .with_content(
                        Element::new().with_style(
                            Style::new()
                                .with_grow(0.0)
                                .with_size_constraint(SizeConstraint {
                                    min_width: Some(ItemSize::Percent(0.3)),
                                    max_width: Some(ItemSize::Percent(0.3)),
                                    ..SizeConstraint::default()
                                }),
                        ),
                    )
                    // .with_content(HeaderLeftMenu)
                    .with_content(
                        Element::new()
                            .with_content(self.center.clone())
                            .with_style(central_container_style()),
                    )
                    .with_content(
                        Element::new()
                            .with_style(
                                Style::new()
                                    .with_align_items(AlignItems::Center)
                                    .with_justify_content(JustifyContent::FlexEnd)
                                    .with_size_constraint(SizeConstraint {
                                        min_width: Some(ItemSize::Percent(0.3)),
                                        max_width: Some(ItemSize::Percent(0.3)),
                                        ..SizeConstraint::default()
                                    })
                                    .with_grow(0.0),
                            )
                            .with_content(ClockChip),
                    ),
            )
    }
}

// #[derive(Clone, Copy)]
// struct HeaderLeftMenu;

// impl Component for HeaderLeftMenu {
//     fn to_element(&self, ctx: &mut ComponentContext) -> Element {
//         let focus_scope = ctx.focus_scope();
//         focus_scope.set_entry_policy(FocusEntryPolicy::Spatial(
//             TraversalPolicy::RectilinearDistance,
//         ));
//         let contains_focus = focus_scope.contains_focus();
//         let menu_state = *ctx
//             .use_shared_state(Id::new(HEADER_MENU_STATE_ID), HeaderMenuState::default)
//             .read();
//         let focused_target = contains_focus
//             .then_some(menu_state.focused_target)
//             .flatten();
//         let indicator_target = focused_target.or(Some(HeaderMenuTarget::Apps));
//         let apps_is_selected = matches!(indicator_target, Some(HeaderMenuTarget::Apps));
//         let pill_is_focused = focused_target.is_some() && contains_focus;
//
//         Container::horizontal()
//             .with_fit(Fit::new().exact_content_size())
//             .align_items_center()
//             .build()
//             .with_tag("apps-header-menu")
//             .with_style(header_menu_style())
//             .with_content(
//                 Element::new()
//                     .with_tag("apps-header-menu-content")
//                     .with_style(header_menu_content_style())
//                     .with_content(header_menu_indicator(
//                         ctx,
//                         indicator_target,
//                         pill_is_focused,
//                     ))
//                     .with_content(
//                         Element::new()
//                             .with_tag("apps-header-title")
//                             .with_style(menu_button_shell_style())
//                             .with_content(HeaderAppsButton {
//                                 is_selected: apps_is_selected,
//                             }),
//                     )
//                     // .with_content(HeaderSettingsTrigger),
//             )
//     }
// }

fn header_style() -> Style {
    Style::new()
        .with_justify_content(JustifyContent::Center)
        .with_background_color(Color::from_rgba_premultiplied(12, 16, 18, 178))
        .with_overflow(daiko::style::Overflow::Visible)
        .with_direction(FlexDirection::Column)
        .with_size_constraint(SizeConstraint::exact_content_height())
        .with_padding(Indent::new(
            SCREEN_PADDING,
            SCREEN_PADDING,
            SCREEN_PADDING,
            0.0,
        ))
}

fn header_row_style() -> Style {
    Style::new()
        .with_overflow(daiko::style::Overflow::Visible)
        .with_fixed_height(ItemSize::Points(HEADER_MENU_HEIGHT))
        .with_direction(FlexDirection::Row)
        .with_justify_content(JustifyContent::SpaceBetween)
        .with_align_items(AlignItems::Center)
}

// fn header_menu_style() -> Style {
//     Style::new()
//         .with_overflow(daiko::style::Overflow::Visible)
//         .with_size_constraint(SizeConstraint::exact_content_width())
//         .with_fixed_height(ItemSize::Points(HEADER_MENU_HEIGHT))
//         .with_direction(FlexDirection::Row)
//         .with_align_items(AlignItems::Center)
//         .with_justify_content(JustifyContent::Center)
//         .with_spacing((HEADER_MENU_GAP, HEADER_MENU_GAP))
//         .with_padding(Indent::from((HEADER_MENU_PADDING, HEADER_MENU_PADDING)))
//         .with_border(Border::uniform(Stroke::new(
//             HEADER_MENU_BORDER_WIDTH,
//             Color::from_rgba_unmultiplied(255, 255, 255, 34),
//         )))
//         .with_border_radius(BorderRadius::all(HEADER_MENU_RADIUS))
// }

// fn header_menu_content_style() -> Style {
//     Style::new()
//         .with_overflow(daiko::style::Overflow::Visible)
//         .with_fixed_size(
//             HEADER_APPS_BUTTON_WIDTH + HEADER_MENU_GAP + HEADER_SETTINGS_BUTTON_WIDTH,
//             HEADER_BUTTON_HEIGHT,
//         )
//         .with_direction(FlexDirection::Row)
//         .with_align_items(AlignItems::Center)
//         .with_justify_content(JustifyContent::Center)
//         .with_spacing((HEADER_MENU_GAP, HEADER_MENU_GAP))
// }
//
// fn menu_button_shell_style() -> Style {
//     Style::new()
//         .with_overflow(daiko::style::Overflow::Visible)
//         .with_size_constraint(SizeConstraint::exact_content_size())
//         .with_order(1)
// }

// fn header_menu_indicator(
//     ctx: &mut ComponentContext,
//     target: Option<HeaderMenuTarget>,
//     is_focused: bool,
// ) -> Element {
//     let base_width = match target.unwrap_or(HeaderMenuTarget::Apps) {
//         HeaderMenuTarget::Apps => HEADER_APPS_BUTTON_WIDTH,
//         HeaderMenuTarget::Settings => HEADER_SETTINGS_BUTTON_WIDTH,
//     };
//     let base_x = match target.unwrap_or(HeaderMenuTarget::Apps) {
//         HeaderMenuTarget::Apps => 0.0,
//         HeaderMenuTarget::Settings => HEADER_APPS_BUTTON_WIDTH + HEADER_MENU_GAP,
//     };
//     let scale = if is_focused {
//         HEADER_FOCUS_SCALE
//     } else {
//         HEADER_ACTIVE_SCALE
//     };
//     let width = base_width * scale;
//     let height = HEADER_BUTTON_HEIGHT * scale;
//     let x = base_x - (width - base_width) / 2.0;
//     let y = -(height - HEADER_BUTTON_HEIGHT) / 2.0;
//     let (color, border_color) = header_indicator_surface(target, is_focused);
//
//     Element::new()
//         .with_tag("apps-header-menu-focus-pill")
//         .with_style(
//             Style::new()
//                 .with_absolute_position(transition(
//                     daiko::Vec2::new(x, y),
//                     AnimationParameters::default()
//                         .with_duration(Duration::from_millis(HEADER_INDICATOR_TRANSITION_MS))
//                         .with_easing(EasingFunction::EaseOut)
//                         .to_transition_options(),
//                     ctx,
//                 ))
//                 .with_fixed_size(
//                     transition(
//                         width,
//                         AnimationParameters::default()
//                             .with_duration(Duration::from_millis(HEADER_INDICATOR_TRANSITION_MS))
//                             .with_easing(EasingFunction::EaseOut)
//                             .to_transition_options(),
//                         ctx,
//                     ),
//                     transition(
//                         height,
//                         AnimationParameters::default()
//                             .with_duration(Duration::from_millis(HEADER_INDICATOR_TRANSITION_MS))
//                             .with_easing(EasingFunction::EaseOut)
//                             .to_transition_options(),
//                         ctx,
//                     ),
//                 )
//                 .with_background_color(transition(
//                     color,
//                     AnimationParameters::default()
//                         .with_duration(Duration::from_millis(HEADER_TRANSITION_MS))
//                         .with_easing(EasingFunction::EaseOut)
//                         .to_transition_options(),
//                     ctx,
//                 ))
//                 .with_border(Border::uniform(Stroke::new(
//                     1.0,
//                     transition(
//                         border_color,
//                         AnimationParameters::default()
//                             .with_duration(Duration::from_millis(80))
//                             .with_easing(EasingFunction::EaseOut)
//                             .to_transition_options(),
//                         ctx,
//                     ),
//                 )))
//                 .with_border_radius(BorderRadius::all(HEADER_BUTTON_RADIUS * scale))
//                 .with_order(0),
//         )
// }

// #[derive(Clone, Copy)]
// struct HeaderAppsButton {
//     is_selected: bool,
// }

// impl Component for HeaderAppsButton {
//     fn to_element(&self, ctx: &mut ComponentContext) -> Element {
//         let mut pointer = ctx.pointer();
//         let focusable = ctx.focusable();
//
//         focusable.set_focus_key(home_top_row_apps_focus_key());
//
//         if pointer.just_pressed() {
//             focusable.request_focus(FocusOrigin::Pointer);
//         }
//
//         let menu_state =
//             ctx.use_shared_state(Id::new(HEADER_MENU_STATE_ID), HeaderMenuState::default);
//         let should_mark_focused = {
//             let state = *menu_state.read();
//             focusable.is_focused() && state.focused_target != Some(HeaderMenuTarget::Apps)
//         };
//         if should_mark_focused {
//             *menu_state.write() = HeaderMenuState {
//                 focused_target: Some(HeaderMenuTarget::Apps),
//             };
//         }
//
//         let state = HeaderButtonState {
//             is_active: self.is_selected,
//             is_pressed: pointer.is_pressed(),
//             is_hovered: pointer.is_hovering(),
//             is_focused: focusable.is_focused(),
//         };
//         let text_color = header_apps_button_text_color(state);
//
//         Element::new()
//             .with_tag("apps-header-apps-button")
//             .with_style(header_button_style(
//                 ctx,
//                 HeaderButtonMetrics {
//                     width: HEADER_APPS_BUTTON_WIDTH,
//                     height: HEADER_BUTTON_HEIGHT,
//                     radius: HEADER_BUTTON_RADIUS,
//                 },
//                 state,
//                 false,
//             ))
//             .with_content(
//                 Text::new("Apps").with_style(
//                     TextStyle::default()
//                         .with_font_size(20.0)
//                         .with_weight(Weight::NORMAL)
//                         .with_font_color(text_color)
//                         .with_vertical_alignment(VerticalTextAlignment::Center)
//                         .with_wrap(TextWrap::NoWrap),
//                 ),
//             )
//     }
// }

// fn header_indicator_surface(target: Option<HeaderMenuTarget>, is_focused: bool) -> (Color, Color) {
//     match target {
//         Some(_) if is_focused => (focused_indicator_color(), focused_indicator_border_color()),
//         Some(_) => (
//             selected_indicator_color(),
//             selected_indicator_border_color(),
//         ),
//         None => (
//             Color::from_rgba_unmultiplied(236, 240, 243, 0),
//             Color::from_rgba_unmultiplied(255, 255, 255, 0),
//         ),
//     }
// }

// pub(super) fn header_button_foreground_color(state: HeaderButtonState) -> Color {
//     if state.is_pressed || state.is_focused {
//         interactive_text_color()
//     } else if state.is_hovered || state.is_active {
//         active_text_color()
//     } else {
//         inactive_text_color()
//     }
// }
//
// fn header_apps_button_text_color(state: HeaderButtonState) -> Color {
//     header_button_foreground_color(state)
// }
//
// fn selected_indicator_color() -> Color {
//     Color::from_rgba_unmultiplied(214, 220, 226, 112)
// }
//
// fn focused_indicator_color() -> Color {
//     Color::from_rgb(236, 240, 243)
// }
//
// fn selected_indicator_border_color() -> Color {
//     Color::from_rgba_unmultiplied(255, 255, 255, 56)
// }
//
// fn focused_indicator_border_color() -> Color {
//     Color::from_rgba_unmultiplied(255, 255, 255, 172)
// }
//
// fn active_text_color() -> Color {
//     Color::from_rgba_unmultiplied(232, 238, 250, 188)
// }
//
// fn interactive_text_color() -> Color {
//     Color::from_rgb(12, 16, 20)
// }
//
// fn inactive_text_color() -> Color {
//     Color::from_rgb(232, 238, 250)
// }

#[derive(Clone, Copy)]
pub(super) struct HeaderButtonMetrics {
    pub width: f32,
    pub height: f32,
    pub radius: f32,
}

#[derive(Clone, Copy)]
pub(super) struct HeaderButtonState {
    pub is_active: bool,
    pub is_pressed: bool,
    pub is_hovered: bool,
    pub is_focused: bool,
}

pub(super) fn header_button_style(
    ctx: &mut ComponentContext,
    metrics: HeaderButtonMetrics,
    state: HeaderButtonState,
    paint_surface: bool,
) -> Style {
    let is_lifted = state.is_hovered || state.is_focused;
    let background = if !paint_surface {
        Color::from_rgba_unmultiplied(255, 255, 255, 0)
    } else if state.is_pressed {
        Color::from_rgb(204, 210, 216)
    } else if state.is_active || is_lifted {
        Color::from_rgb(236, 240, 243)
    } else {
        Color::from_rgba_unmultiplied(255, 255, 255, 0)
    };
    let border_color = if paint_surface && (state.is_active || is_lifted) {
        Color::from_rgba_unmultiplied(255, 255, 255, 172)
    } else {
        Color::from_rgba_unmultiplied(255, 255, 255, 0)
    };

    Style::new()
        .with_overflow(daiko::style::Overflow::Visible)
        .with_fixed_size(metrics.width, metrics.height)
        .with_direction(FlexDirection::Row)
        .with_align_items(AlignItems::Center)
        .with_justify_content(JustifyContent::Center)
        .with_background_color(transition(
            background,
            AnimationParameters::default()
                .with_duration(Duration::from_millis(HEADER_TRANSITION_MS))
                .with_easing(EasingFunction::EaseOut)
                .to_transition_options(),
            ctx,
        ))
        .with_border(Border::uniform(Stroke::new(
            1.0,
            transition(
                border_color,
                AnimationParameters::default()
                    .with_duration(Duration::from_millis(80))
                    .with_easing(EasingFunction::EaseOut)
                    .to_transition_options(),
                ctx,
            ),
        )))
        .with_border_radius(BorderRadius::all(metrics.radius))
        .with_cursor(CursorIcon::PointingHand)
        .with_order(1)
}

fn central_container_style() -> Style {
    Style::new()
        .with_justify_content(JustifyContent::Center)
        .with_align_items(AlignItems::Center)
        .with_grow(1.0)
}
