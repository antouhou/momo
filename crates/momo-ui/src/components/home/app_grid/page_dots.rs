mod liquid;

use std::time::Duration;
use daiko::{
    Element, Id, Vec2,
    animation::{AnimationParameters, easing::EasingFunction, transition},
    component::{Component, ComponentContext},
    layout::{AlignItems, FlexDirection, JustifyContent},
    navigation::FocusBoundary,
    style::{BorderRadius, Color, Overflow, Style},
    widgets::container::{Container, Fit},
};
use self::liquid::{LiquidMorphSpec, build_liquid_morph_path};
use crate::components::home::app_grid::{
    ACTIVE_PAGE_DOT_WIDTH, PAGE_DOT_FOCUS_BORDER_WIDTH, PAGE_DOT_FOCUS_PADDING, PAGE_DOT_SIZE,
    PAGE_DOTS_GAP, page_dot_focus_key, state::app_grid_state_handle,
};

const PAGE_DOT_ACTIVE_VISUAL_TARGET_ID: &str = "momo_home_app_grid_page_dot_active_visual_target";
const PAGE_DOT_ACTIVE_VISUAL_MORPH_ID: &str = "momo_home_app_grid_page_dot_active_visual_morph";
const PAGE_DOT_FOCUS_RING_DURATION_MS: u64 = 140;
const PAGE_DOT_ACTIVE_VISUAL_DURATION_MS: u64 = 180;
const PAGE_DOT_ACTIVE_NECK_RATIO: f32 = 0.34;

#[derive(Clone, Copy, Default)]
struct PageDotMorphState {
    from_page: Option<usize>,
    last_target_page: Option<usize>,
}

#[derive(Clone, Copy, Default)]
struct PageDotMorphFrame {
    from_page: Option<usize>,
    target_page: Option<usize>,
    progress: f32,
}

#[derive(Clone, Copy)]
pub struct PageDots;

impl Component for PageDots {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let app_grid_state = app_grid_state_handle(ctx);
        let (active_page, page_count) = {
            let state = app_grid_state.read();
            (state.active_page, state.page_count)
        };

        let focus_scope = ctx.focus_scope();
        focus_scope.set_boundary(FocusBoundary::Escape);
        focus_scope.set_default_focus(page_dot_focus_key(active_page));

        let compact_track_width = page_dots_compact_track_width(page_count);
        let track_width = page_dots_track_width(page_count, active_page);
        let track_height = page_dot_target_height();

        let mut dots = Container::horizontal()
            .with_fit(
                Fit::new()
                    .exact_width(compact_track_width)
                    .exact_height(track_height),
            )
            .align_items_center()
            .justify_content_center()
            .with_spacing((PAGE_DOTS_GAP, PAGE_DOTS_GAP))
            .build();

        for page_index in 0..page_count {
            dots.add_content(page_dot(page_index, ctx));
        }

        let mut root = Element::new().with_tag("apps-grid-page-dots").with_style(
            Style::new()
                .with_direction(FlexDirection::Row)
                .with_align_items(AlignItems::Center)
                .with_justify_content(JustifyContent::Center)
                .with_fixed_size(track_width, track_height)
                .with_overflow(Overflow::Visible),
        );

        root.add_content(dots);
        root.add_content(page_dot_active_visual(ctx, page_count, active_page));
        // root.add_content(page_dot_focus_ring(
        //     ctx,
        //     page_count,
        //     active_page,
        //     focus_ring_page,
        // ));
        root
    }
}

fn page_dot(page_index: usize, ctx: &mut ComponentContext) -> Element {
    Element::new()
        .with_tag(format!("apps-grid-page-dot-{page_index}"))
        .with_style(page_dot_target_style())
        .with_content(page_dot_visual(page_index, ctx))
}

fn page_dot_target_style() -> Style {
    Style::new()
        .with_direction(FlexDirection::Row)
        .with_justify_content(JustifyContent::Center)
        .with_align_items(AlignItems::Center)
        .with_fixed_size(
            page_dot_target_width(PAGE_DOT_SIZE),
            page_dot_target_height(),
        )
        .with_padding(PAGE_DOT_FOCUS_PADDING + PAGE_DOT_FOCUS_BORDER_WIDTH)
        .with_border_radius(BorderRadius::all(page_dot_target_height() / 2.0))
}

fn page_dot_visual(page_index: usize, ctx: &mut ComponentContext) -> Element {
    Element::new()
        .with_tag(format!("apps-grid-page-dot-visual-{page_index}"))
        .with_style(
            Style::new()
                .with_fixed_size(PAGE_DOT_SIZE, PAGE_DOT_SIZE)
                .with_background_color(transition(
                    Color::from_rgb(88, 105, 124),
                    AnimationParameters::default()
                        .with_duration(Duration::from_millis(PAGE_DOT_FOCUS_RING_DURATION_MS))
                        .to_transition_options(),
                    ctx,
                ))
                .with_border_radius(BorderRadius::all(PAGE_DOT_SIZE / 2.0)),
        )
}

fn page_dot_active_visual(
    ctx: &mut ComponentContext,
    page_count: usize,
    active_page: usize,
) -> Element {
    let morph_frame = page_dot_morph_frame(
        ctx,
        Id::new(PAGE_DOT_ACTIVE_VISUAL_TARGET_ID),
        Id::new(PAGE_DOT_ACTIVE_VISUAL_MORPH_ID),
        Some(active_page),
        Duration::from_millis(PAGE_DOT_ACTIVE_VISUAL_DURATION_MS),
    );

    let source_page = morph_frame.from_page.unwrap_or(active_page);
    let liquid_path = build_liquid_morph_path(
        LiquidMorphSpec {
            from_center_x: page_dot_center_x(source_page),
            to_center_x: page_dot_center_x(morph_frame.target_page.unwrap_or(active_page)),
            from_width: ACTIVE_PAGE_DOT_WIDTH,
            to_width: ACTIVE_PAGE_DOT_WIDTH,
            height: PAGE_DOT_SIZE,
            top_y: 0.0,
            neck_ratio: PAGE_DOT_ACTIVE_NECK_RATIO,
        },
        morph_frame.progress,
    );

    let color = transition(
        Color::from_rgb(236, 246, 255),
        AnimationParameters::default()
            .with_duration(Duration::from_millis(PAGE_DOT_FOCUS_RING_DURATION_MS))
            .to_transition_options(),
        ctx,
    );

    Element::new()
        .with_tag("apps-grid-page-dot-active-visual")
        .with_clip_path(liquid_path)
        .with_style(
            Style::new()
                .with_absolute_position(Vec2::new(0.0, page_dot_visual_top()))
                .with_fixed_size(
                    page_dots_track_width(page_count, active_page),
                    PAGE_DOT_SIZE,
                )
                .with_background_color(color)
                .with_order(1),
        )
}

fn page_dot_morph_frame(
    ctx: &mut ComponentContext,
    state_id: Id,
    animation_id: Id,
    target_page: Option<usize>,
    duration: Duration,
) -> PageDotMorphFrame {
    let morph_state = ctx.use_local_state_with_id(state_id, PageDotMorphState::default);
    let mut state = *morph_state.read();
    let animation = ctx.animation_with_id(
        animation_id,
        AnimationParameters::default()
            .with_duration(duration)
            .with_easing(EasingFunction::EaseInOut),
    );

    if state.last_target_page != target_page {
        let from_page = state.last_target_page.or(target_page);
        state = PageDotMorphState {
            from_page,
            last_target_page: target_page,
        };

        if let (Some(from_page), Some(target_page)) = (from_page, target_page) {
            if from_page != target_page {
                animation.set_progress(0.0);
                animation.play_forward();
            } else {
                animation.set_progress(1.0);
            }
        } else {
            animation.set_progress(1.0);
        }
    }

    let progress = if state.from_page == state.last_target_page || target_page.is_none() {
        1.0
    } else {
        animation.progress()
    };

    if progress >= 1.0 {
        state.from_page = state.last_target_page;
    }

    *morph_state.write_silent() = state;

    PageDotMorphFrame {
        from_page: state.from_page,
        target_page: state.last_target_page,
        progress,
    }
}

fn page_dots_track_width(page_count: usize, _active_page: usize) -> f32 {
    page_dots_compact_track_width(page_count) + page_dot_track_side_inset() * 2.0
}

fn page_dot_target_left(page_index: usize) -> f32 {
    page_dot_track_side_inset()
        + page_index as f32 * (page_dot_target_width(PAGE_DOT_SIZE) + PAGE_DOTS_GAP)
}

fn page_dot_visual_top() -> f32 {
    page_dot_target_height() / 2.0 - PAGE_DOT_SIZE / 2.0
}

fn page_dots_compact_track_width(page_count: usize) -> f32 {
    page_count as f32 * page_dot_target_width(PAGE_DOT_SIZE)
        + page_count.saturating_sub(1) as f32 * PAGE_DOTS_GAP
}

fn page_dot_track_side_inset() -> f32 {
    (page_dot_target_width(ACTIVE_PAGE_DOT_WIDTH) - page_dot_target_width(PAGE_DOT_SIZE)) / 2.0
}

fn page_dot_center_x(page_index: usize) -> f32 {
    page_dot_target_left(page_index) + page_dot_target_width(PAGE_DOT_SIZE) / 2.0
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

#[cfg(test)]
mod tests;
