use daiko::animation::easing::EasingFunction;
use daiko::animation::{AnimationParameters, transition};
use daiko::component::ComponentContext;
use daiko::layout::{AlignItems, FlexDirection, ItemSize, JustifyContent, SizeConstraint};
use daiko::style::{Border, BorderRadius, Color, CursorIcon, Indent, Stroke, Style};
use daiko::widgets::text::{TextStyle, TextWrap, Weight};
use std::time::Duration;

pub const SETTINGS_MENU_WIDTH: f32 = 392.0;
pub const SETTINGS_MENU_EDGE_MARGIN: f32 = 40.0;
pub const SETTINGS_MENU_TOP_OFFSET: f32 = 96.0;
pub const SETTINGS_MENU_GAP: f32 = 12.0;
pub const SETTINGS_MENU_HORIZONTAL_PADDING: f32 = 16.0;
pub const SETTINGS_MENU_VERTICAL_PADDING: f32 = 18.0;
pub const SETTINGS_MENU_INNER_WIDTH: f32 =
    SETTINGS_MENU_WIDTH - SETTINGS_MENU_HORIZONTAL_PADDING * 2.0;
pub const SETTINGS_MENU_SLIDE_DISTANCE: f32 =
    SETTINGS_MENU_WIDTH + SETTINGS_MENU_EDGE_MARGIN + 36.0;
pub const SETTINGS_ROUND_BUTTON_SIZE: f32 = 44.0;
pub const SETTINGS_STATUS_CHIP_WIDTH: f32 = 92.0;
pub const SETTINGS_STATUS_CHIP_HEIGHT: f32 = 44.0;
pub const SETTINGS_TILE_WIDTH: f32 = 174.0;
pub const SETTINGS_TILE_HEIGHT: f32 = 76.0;

const PANEL_RADIUS: f32 = 30.0;
const CONTROL_RADIUS: f32 = 22.0;
const TILE_RADIUS: f32 = 20.0;
const CONTROL_TRANSITION_MS: u64 = 120;

#[derive(Clone, Copy)]
pub struct QuickSettingsControlState {
    pub is_hovered: bool,
    pub is_focused: bool,
}

impl QuickSettingsControlState {
    fn is_highlighted(self) -> bool {
        self.is_hovered || self.is_focused
    }
}

pub fn settings_menu_style(max_height: f32) -> Style {
    Style::new()
        .with_size_constraint(SizeConstraint::exact_content_height().with_max_height(max_height))
        .with_fixed_width(ItemSize::Points(SETTINGS_MENU_WIDTH))
        .with_padding(Indent::new(
            SETTINGS_MENU_HORIZONTAL_PADDING,
            SETTINGS_MENU_VERTICAL_PADDING,
            SETTINGS_MENU_HORIZONTAL_PADDING,
            SETTINGS_MENU_VERTICAL_PADDING,
        ))
        .with_direction(FlexDirection::Column)
        .with_spacing((SETTINGS_MENU_GAP, SETTINGS_MENU_GAP))
        .with_background_color(Color::from_rgb(12, 16, 18))
        .with_border(Border::uniform(Stroke::new(
            1.0,
            Color::from_rgba_unmultiplied(255, 255, 255, 42),
        )))
        .with_border_radius(BorderRadius::all(PANEL_RADIUS))
}

pub fn settings_scrollable_style() -> Style {
    Style::new()
}

pub fn settings_top_row_style() -> Style {
    Style::new()
        .with_size_constraint(SizeConstraint::exact_content_height())
        .with_fixed_width(ItemSize::Percent(1.0))
        .with_direction(FlexDirection::Row)
        .with_align_items(AlignItems::Center)
        .with_justify_content(JustifyContent::SpaceBetween)
}

pub fn settings_top_actions_style() -> Style {
    Style::new()
        .with_direction(FlexDirection::Row)
        .with_align_items(AlignItems::Center)
        .with_justify_content(JustifyContent::FlexEnd)
        .with_spacing((10.0, 10.0))
}

pub fn settings_tile_grid_style() -> Style {
    Style::new()
        .with_size_constraint(SizeConstraint::exact_content_height())
        .with_direction(FlexDirection::Column)
        .with_spacing((SETTINGS_MENU_GAP, SETTINGS_MENU_GAP))
}

pub fn settings_tile_row_style() -> Style {
    Style::new()
        .with_size_constraint(SizeConstraint::exact_content_height())
        .with_direction(FlexDirection::Row)
        .with_justify_content(JustifyContent::SpaceBetween)
        .with_align_items(AlignItems::Center)
}

pub fn settings_tile_content_style() -> Style {
    Style::new()
        .with_direction(FlexDirection::Row)
        .with_align_items(AlignItems::Center)
        .with_justify_content(JustifyContent::FlexStart)
        .with_spacing((12.0, 12.0))
}

pub fn settings_tile_text_column_style() -> Style {
    Style::new()
        .with_fixed_height(ItemSize::Points(38.0))
        .with_direction(FlexDirection::Column)
        .with_justify_content(JustifyContent::Center)
        .with_align_items(AlignItems::FlexStart)
}

pub fn settings_status_chip_style(
    state: QuickSettingsControlState,
    ctx: &mut ComponentContext,
) -> Style {
    let background = if state.is_highlighted() {
        Color::from_rgb(236, 240, 243)
    } else {
        Color::from_rgb(214, 220, 226)
    };
    let border_color = if state.is_highlighted() {
        Color::from_rgb(236, 240, 243)
    } else {
        Color::from_rgba_unmultiplied(255, 255, 255, 48)
    };

    Style::new()
        .with_fixed_size(SETTINGS_STATUS_CHIP_WIDTH, SETTINGS_STATUS_CHIP_HEIGHT)
        .with_direction(FlexDirection::Row)
        .with_align_items(AlignItems::Center)
        .with_justify_content(JustifyContent::Center)
        .with_padding(Indent::uniform(10.0))
        .with_background_color(transition(
            background,
            AnimationParameters::default()
                .with_duration(Duration::from_millis(CONTROL_TRANSITION_MS))
                .with_easing(EasingFunction::EaseOut)
                .to_transition_options(),
            ctx,
        ))
        .with_border(Border::uniform(Stroke::new(
            1.0,
            transition(
                border_color,
                AnimationParameters::default()
                    .with_duration(Duration::from_millis(CONTROL_TRANSITION_MS))
                    .with_easing(EasingFunction::EaseOut)
                    .to_transition_options(),
                ctx,
            ),
        )))
        .with_border_radius(BorderRadius::all(CONTROL_RADIUS))
        .with_cursor(CursorIcon::PointingHand)
}

pub fn settings_round_button_style(
    state: QuickSettingsControlState,
    ctx: &mut ComponentContext,
    is_active: bool,
    is_danger: bool,
) -> Style {
    let background = if is_danger && state.is_highlighted() {
        Color::from_rgb(92, 32, 43)
    } else if is_danger {
        Color::from_rgb(74, 28, 36)
    } else if is_active || state.is_highlighted() {
        Color::from_rgb(236, 240, 243)
    } else {
        Color::from_rgb(24, 28, 31)
    };
    let border_color = if is_danger && state.is_highlighted() {
        Color::from_rgba_unmultiplied(255, 189, 198, 184)
    } else if is_danger {
        Color::from_rgba_unmultiplied(255, 160, 174, 72)
    } else if is_active || state.is_highlighted() {
        Color::from_rgba_unmultiplied(255, 255, 255, 138)
    } else {
        Color::from_rgba_unmultiplied(255, 255, 255, 30)
    };

    Style::new()
        .with_fixed_size(SETTINGS_ROUND_BUTTON_SIZE, SETTINGS_ROUND_BUTTON_SIZE)
        .with_direction(FlexDirection::Row)
        .with_align_items(AlignItems::Center)
        .with_justify_content(JustifyContent::Center)
        .with_background_color(transition(
            background,
            AnimationParameters::default()
                .with_duration(Duration::from_millis(CONTROL_TRANSITION_MS))
                .with_easing(EasingFunction::EaseOut)
                .to_transition_options(),
            ctx,
        ))
        .with_border(Border::uniform(Stroke::new(
            1.0,
            transition(
                border_color,
                AnimationParameters::default()
                    .with_duration(Duration::from_millis(CONTROL_TRANSITION_MS))
                    .with_easing(EasingFunction::EaseOut)
                    .to_transition_options(),
                ctx,
            ),
        )))
        .with_border_radius(BorderRadius::all(CONTROL_RADIUS))
        .with_cursor(CursorIcon::PointingHand)
}

pub fn settings_tile_button_style(
    state: QuickSettingsControlState,
    ctx: &mut ComponentContext,
    is_active: bool,
) -> Style {
    let background = if is_active {
        Color::from_rgb(104, 79, 140)
    } else if state.is_highlighted() {
        Color::from_rgb(38, 42, 46)
    } else {
        Color::from_rgb(24, 28, 31)
    };
    let border_color = if is_active {
        Color::from_rgba_unmultiplied(211, 191, 255, 112)
    } else if state.is_highlighted() {
        Color::from_rgba_unmultiplied(255, 255, 255, 92)
    } else {
        Color::from_rgba_unmultiplied(255, 255, 255, 28)
    };

    Style::new()
        .with_fixed_size(SETTINGS_TILE_WIDTH, SETTINGS_TILE_HEIGHT)
        .with_direction(FlexDirection::Row)
        .with_align_items(AlignItems::Center)
        .with_padding(Indent::new(14.0, 14.0, 14.0, 14.0))
        .with_background_color(transition(
            background,
            AnimationParameters::default()
                .with_duration(Duration::from_millis(CONTROL_TRANSITION_MS))
                .with_easing(EasingFunction::EaseOut)
                .to_transition_options(),
            ctx,
        ))
        .with_border(Border::uniform(Stroke::new(
            1.0,
            transition(
                border_color,
                AnimationParameters::default()
                    .with_duration(Duration::from_millis(CONTROL_TRANSITION_MS))
                    .with_easing(EasingFunction::EaseOut)
                    .to_transition_options(),
                ctx,
            ),
        )))
        .with_border_radius(BorderRadius::all(TILE_RADIUS))
        .with_cursor(CursorIcon::PointingHand)
}

pub fn settings_tile_icon_style(is_active: bool) -> Style {
    let background = if is_active {
        Color::from_rgba_unmultiplied(255, 255, 255, 26)
    } else {
        Color::from_rgba_unmultiplied(255, 255, 255, 12)
    };

    Style::new()
        .with_fixed_size(38.0, 38.0)
        .with_direction(FlexDirection::Row)
        .with_align_items(AlignItems::Center)
        .with_justify_content(JustifyContent::Center)
        .with_background_color(background)
        .with_border_radius(BorderRadius::all(19.0))
}

pub fn status_value_style() -> TextStyle {
    TextStyle::default()
        .with_font_size(18.0)
        .with_weight(Weight::NORMAL)
        .with_font_color(Color::from_rgb(12, 16, 20))
        .with_wrap(TextWrap::NoWrap)
}

pub fn tile_title_style(is_active: bool) -> TextStyle {
    let color = if is_active {
        Color::from_rgb(248, 241, 255)
    } else {
        Color::from_rgb(235, 240, 247)
    };

    TextStyle::default()
        .with_font_size(18.0)
        .with_weight(Weight::NORMAL)
        .with_font_color(color)
        .with_wrap(TextWrap::NoWrap)
}
