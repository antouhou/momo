use crate::components::home::app_grid::{
    ACTIVE_PAGE_DOT_WIDTH, PAGE_DOT_FOCUS_BORDER_WIDTH, PAGE_DOT_FOCUS_PADDING, PAGE_DOT_SIZE,
    PAGE_DOTS_GAP, page_dot_focus_key,
};
use crate::components::home::model::HOME_APP_GRID_PAGE_STATE_ID;
use daiko::animation::{AnimationParameters, SmoothFollowConfig, transition};
use daiko::component::{Component, ComponentContext};
use daiko::layout::{AlignItems, FlexDirection, JustifyContent};
use daiko::navigation::{FocusBoundary, FocusKey, FocusOrigin};
use daiko::style::{Border, BorderRadius, Color, CursorIcon, Overflow, Stroke, Style};
use daiko::widgets::container::{Container, Fit};
use daiko::{Element, Id, Vec2};
use std::time::Duration;

const PAGE_DOT_HOVERED_STATE_ID: &str = "momo_home_app_grid_page_dot_hovered";
const PAGE_DOT_FOCUS_RING_TARGET_ID: &str = "momo_home_app_grid_page_dot_focus_ring_target";
const PAGE_DOT_FOCUS_RING_X_ID: &str = "momo_home_app_grid_page_dot_focus_ring_x";
const PAGE_DOT_FOCUS_RING_WIDTH_ID: &str = "momo_home_app_grid_page_dot_focus_ring_width";
const PAGE_DOT_FOCUS_RING_DURATION_MS: u64 = 140;

#[derive(Clone, Copy, Default)]
struct PageDotFocusRingState {
    last_target_page: Option<usize>,
}

#[derive(Clone, Copy)]
pub(in crate::components::home::app_grid) struct PageDots {
    pub(crate) page_count: usize,
    pub(crate) active_page: usize,
    pub(crate) interactions_disabled: bool,
}

impl Component for PageDots {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let focus_scope = ctx.focus_scope();
        focus_scope.set_boundary(FocusBoundary::Escape);
        focus_scope.set_default_focus(page_dot_focus_key(self.active_page));

        let hovered_page = *ctx
            .use_shared_state(Id::new(PAGE_DOT_HOVERED_STATE_ID), || None::<usize>)
            .read();
        let focused_page = focused_page_index(focus_scope.focused_child_key(), self.page_count);
        let focus_ring_page = if self.interactions_disabled {
            None
        } else {
            hovered_page.or(focused_page)
        };
        let track_width = page_dots_track_width(self.page_count, self.active_page);
        let track_height = page_dot_target_height();

        let mut dots = Container::horizontal()
            .with_fit(
                Fit::new()
                    .exact_width(track_width)
                    .exact_height(track_height),
            )
            .align_items_center()
            .justify_content_center()
            .with_spacing((PAGE_DOTS_GAP, PAGE_DOTS_GAP))
            .build();

        for page_index in 0..self.page_count {
            dots.add_content(PageDot {
                page_index,
                is_active: page_index == self.active_page,
                interactions_disabled: self.interactions_disabled,
            });
        }

        let mut root = Element::new().with_tag("apps-grid-page-dots").with_style(
            Style::new()
                .with_direction(FlexDirection::Row)
                .with_align_items(AlignItems::Center)
                .with_justify_content(JustifyContent::Center)
                .with_fixed_size(track_width, track_height)
                .with_overflow(Overflow::Visible),
        );

        root.add_content(page_dot_focus_ring(
            ctx,
            self.page_count,
            self.active_page,
            focus_ring_page,
        ));
        root.add_content(dots);
        root
    }
}

#[derive(Clone, Copy)]
struct PageDot {
    page_index: usize,
    is_active: bool,
    interactions_disabled: bool,
}

impl Component for PageDot {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let mut pointer = ctx.pointer();
        let focusable = ctx.focusable();
        let hovered_page =
            ctx.use_shared_state(Id::new(PAGE_DOT_HOVERED_STATE_ID), || None::<usize>);
        focusable.set_focus_key(page_dot_focus_key(self.page_index));
        focusable.set_navigation_enabled(!self.interactions_disabled);

        let is_hovering = !self.interactions_disabled && pointer.is_hovering();

        if !self.interactions_disabled && pointer.just_entered() {
            *hovered_page.write_silent() = Some(self.page_index);
            focusable.request_focus(FocusOrigin::Pointer);
        }

        if hovered_page.read().as_ref() == Some(&self.page_index) && !is_hovering {
            *hovered_page.write_silent() = None;
        }

        let just_selected =
            !self.interactions_disabled && (pointer.just_pressed() || focusable.just_activated());
        if just_selected {
            *ctx.use_shared_state(Id::new(HOME_APP_GRID_PAGE_STATE_ID), || 0)
                .write() = self.page_index;
        }

        page_dot(self.page_index, self.is_active, is_hovering, ctx)
    }
}

fn page_dot(
    page_index: usize,
    is_active: bool,
    is_hovered: bool,
    ctx: &mut ComponentContext,
) -> Element {
    let dot_width = page_dot_visual_width(is_active);
    Element::new()
        .with_tag(format!("apps-grid-page-dot-{page_index}"))
        .with_style(page_dot_target_style(dot_width, is_hovered))
        .with_content(page_dot_visual(page_index, is_active, ctx))
}

fn page_dot_target_style(dot_width: f32, is_hovered: bool) -> Style {
    let mut style = Style::new()
        .with_direction(FlexDirection::Row)
        .with_justify_content(JustifyContent::Center)
        .with_align_items(AlignItems::Center)
        .with_fixed_size(page_dot_target_width(dot_width), page_dot_target_height())
        .with_padding(PAGE_DOT_FOCUS_PADDING + PAGE_DOT_FOCUS_BORDER_WIDTH)
        .with_border_radius(BorderRadius::all(page_dot_target_height() / 2.0));

    if is_hovered {
        style.set_cursor(CursorIcon::PointingHand);
    }

    style
}

fn page_dot_visual(page_index: usize, is_active: bool, ctx: &mut ComponentContext) -> Element {
    let width = page_dot_visual_width(is_active);
    let color = if is_active {
        Color::from_rgb(236, 246, 255)
    } else {
        Color::from_rgb(88, 105, 124)
    };

    Element::new()
        .with_tag(format!("apps-grid-page-dot-visual-{page_index}"))
        .with_style(
            Style::new()
                .with_fixed_size(width, PAGE_DOT_SIZE)
                .with_background_color(transition(
                    color,
                    AnimationParameters::default()
                        .with_duration(Duration::from_millis(PAGE_DOT_FOCUS_RING_DURATION_MS))
                        .to_transition_options(),
                    ctx,
                ))
                .with_border_radius(BorderRadius::all(PAGE_DOT_SIZE / 2.0)),
        )
}

fn page_dot_focus_ring(
    ctx: &mut ComponentContext,
    page_count: usize,
    active_page: usize,
    focus_ring_page: Option<usize>,
) -> Element {
    let ring_state = ctx.use_local_state_with_id(
        Id::new(PAGE_DOT_FOCUS_RING_TARGET_ID),
        PageDotFocusRingState::default,
    );
    let previous_target_page = ring_state.read().last_target_page;

    let layout_page = focus_ring_page
        .or(previous_target_page)
        .unwrap_or(active_page.min(page_count.saturating_sub(1)));
    let target_x = page_dot_target_left(layout_page, active_page);
    let target_width = page_dot_target_width(page_dot_visual_width(layout_page == active_page));
    let should_reset_ring = focus_ring_page.is_some() && previous_target_page.is_none();
    let rendered_x = {
        let mut x_follow = ctx.smooth_follow_with_id::<f32>(
            Id::new(PAGE_DOT_FOCUS_RING_X_ID),
            SmoothFollowConfig::new(
                Duration::from_millis(PAGE_DOT_FOCUS_RING_DURATION_MS),
                0.28,
                0.32,
            ),
        );
        if should_reset_ring {
            x_follow.reset_to(target_x);
            target_x
        } else {
            x_follow.follow(target_x)
        }
    };
    let rendered_width = {
        let mut width_follow = ctx.smooth_follow_with_id::<f32>(
            Id::new(PAGE_DOT_FOCUS_RING_WIDTH_ID),
            SmoothFollowConfig::new(
                Duration::from_millis(PAGE_DOT_FOCUS_RING_DURATION_MS),
                0.28,
                0.32,
            ),
        );
        if should_reset_ring {
            width_follow.reset_to(target_width);
            target_width
        } else {
            width_follow.follow(target_width)
        }
    };

    *ring_state.write_silent() = PageDotFocusRingState {
        last_target_page: focus_ring_page,
    };

    let border_color = transition(
        if focus_ring_page.is_some() {
            Color::from_rgb(236, 246, 255)
        } else {
            Color::TRANSPARENT
        },
        AnimationParameters::default()
            .with_duration(Duration::from_millis(PAGE_DOT_FOCUS_RING_DURATION_MS))
            .to_transition_options(),
        ctx,
    );

    Element::new()
        .with_tag("apps-grid-page-dot-focus-ring")
        .with_style(
            Style::new()
                .with_absolute_position(Vec2::new(rendered_x, 0.0))
                .with_fixed_size(rendered_width, page_dot_target_height())
                .with_border(Border::uniform(Stroke::new(
                    PAGE_DOT_FOCUS_BORDER_WIDTH,
                    border_color,
                )))
                .with_border_radius(BorderRadius::all(page_dot_target_height() / 2.0)),
        )
}

fn focused_page_index(focused_key: Option<FocusKey>, page_count: usize) -> Option<usize> {
    let focused_key = focused_key?;
    (0..page_count).find(|page_index| page_dot_focus_key(*page_index) == focused_key)
}

fn page_dots_track_width(page_count: usize, active_page: usize) -> f32 {
    (0..page_count)
        .map(|page_index| page_dot_target_width(page_dot_visual_width(page_index == active_page)))
        .sum::<f32>()
        + page_count.saturating_sub(1) as f32 * PAGE_DOTS_GAP
}

fn page_dot_target_left(page_index: usize, active_page: usize) -> f32 {
    (0..page_index)
        .map(|candidate| page_dot_target_width(page_dot_visual_width(candidate == active_page)))
        .sum::<f32>()
        + page_index as f32 * PAGE_DOTS_GAP
}

fn page_dot_target_width(dot_width: f32) -> f32 {
    dot_width + page_dot_target_outset()
}

fn page_dot_target_height() -> f32 {
    PAGE_DOT_SIZE + page_dot_target_outset()
}

fn page_dot_target_outset() -> f32 {
    (PAGE_DOT_FOCUS_PADDING + PAGE_DOT_FOCUS_BORDER_WIDTH) * 2.0
}

fn page_dot_visual_width(is_active: bool) -> f32 {
    if is_active {
        ACTIVE_PAGE_DOT_WIDTH
    } else {
        PAGE_DOT_SIZE
    }
}
