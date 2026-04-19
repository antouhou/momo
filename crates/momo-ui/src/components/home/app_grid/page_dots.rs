use crate::components::home::app_grid::{
    ACTIVE_PAGE_DOT_WIDTH, PAGE_DOT_FOCUS_BORDER_WIDTH, PAGE_DOT_FOCUS_PADDING, PAGE_DOT_SIZE,
    PAGE_DOTS_GAP, page_dot_focus_key,
};
use crate::components::home::model::HOME_APP_GRID_PAGE_STATE_ID;
use daiko::animation::easing::EasingFunction;
use daiko::animation::{AnimationParameters, transition};
use daiko::component::{Component, ComponentContext};
use daiko::layout::{AlignItems, FlexDirection, JustifyContent, Layout};
use daiko::lyon::path::Winding;
use daiko::navigation::{FocusBoundary, FocusKey, FocusOrigin};
use daiko::style::{BorderRadius, Color, CursorIcon, Overflow, Style};
use daiko::widgets::container::{Container, Fit};
use daiko::{BorderRadii, Element, Id, Path, Pos2, Rect, Vec2};
use std::time::Duration;

const PAGE_DOT_HOVERED_STATE_ID: &str = "momo_home_app_grid_page_dot_hovered";
const PAGE_DOT_ACTIVE_VISUAL_TARGET_ID: &str = "momo_home_app_grid_page_dot_active_visual_target";
const PAGE_DOT_ACTIVE_VISUAL_MORPH_ID: &str = "momo_home_app_grid_page_dot_active_visual_morph";
const PAGE_DOT_FOCUS_RING_TARGET_ID: &str = "momo_home_app_grid_page_dot_focus_ring_target";
const PAGE_DOT_FOCUS_RING_MORPH_ID: &str = "momo_home_app_grid_page_dot_focus_ring_morph";
const PAGE_DOT_FOCUS_RING_DURATION_MS: u64 = 140;
const PAGE_DOT_ACTIVE_VISUAL_DURATION_MS: u64 = 180;
const PAGE_DOT_ACTIVE_NECK_RATIO: f32 = 0.34;
const PAGE_DOT_ACTIVE_TRAILING_MIN_SCALE: f32 = 0.42;
const PAGE_DOT_FOCUS_RING_NECK_RATIO: f32 = 0.46;
const PAGE_DOT_FOCUS_RING_TRAILING_MIN_SCALE: f32 = 0.58;
const LIQUID_PATH_SAMPLES: usize = 14;
const CIRCLE_KAPPA: f32 = 0.552_284_8;

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
struct LiquidMorphSpec {
    from_center_x: f32,
    to_center_x: f32,
    from_width: f32,
    to_width: f32,
    height: f32,
    top_y: f32,
    neck_ratio: f32,
    trailing_min_scale: f32,
}

#[derive(Clone, Copy)]
pub(in crate::components::home::app_grid) struct PageDots {
    pub(crate) page_count: usize,
    pub(crate) active_page: usize,
    pub(crate) interactions_disabled: bool,
}

impl Component for PageDots {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let mut pointer = ctx.pointer();
        let focus_scope = ctx.focus_scope();
        focus_scope.set_boundary(FocusBoundary::Escape);
        focus_scope.set_default_focus(page_dot_focus_key(self.active_page));

        if !self.interactions_disabled
            && pointer.just_pressed_anywhere()
            && let Some(clicked_page) = clicked_page_index(
                ctx.app_context.input_state().pointer.interact_position(),
                ctx.layout(),
                self.page_count,
            )
        {
            *ctx.use_shared_state(Id::new(HOME_APP_GRID_PAGE_STATE_ID), || 0)
                .write() = clicked_page;
        }

        let hovered_page = *ctx
            .use_shared_state(Id::new(PAGE_DOT_HOVERED_STATE_ID), || None::<usize>)
            .read();
        let focused_page = focused_page_index(focus_scope.focused_child_key(), self.page_count);
        let focus_ring_page = if self.interactions_disabled {
            None
        } else {
            hovered_page.or(focused_page)
        };
        let compact_track_width = page_dots_compact_track_width(self.page_count);
        let track_width = page_dots_track_width(self.page_count, self.active_page);
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

        for page_index in 0..self.page_count {
            dots.add_content(PageDot {
                page_index,
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

        root.add_content(dots);
        root.add_content(page_dot_active_visual(
            ctx,
            self.page_count,
            self.active_page,
            self.interactions_disabled,
        ));
        root.add_content(page_dot_focus_ring(
            ctx,
            self.page_count,
            self.active_page,
            focus_ring_page,
            self.interactions_disabled,
        ));
        root
    }
}

#[derive(Clone, Copy)]
struct PageDot {
    page_index: usize,
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

        page_dot(self.page_index, is_hovering, ctx)
    }
}

fn page_dot(page_index: usize, is_hovered: bool, ctx: &mut ComponentContext) -> Element {
    Element::new()
        .with_tag(format!("apps-grid-page-dot-{page_index}"))
        .with_style(page_dot_target_style(is_hovered))
        .with_content(page_dot_visual(page_index, ctx))
}

fn page_dot_target_style(is_hovered: bool) -> Style {
    let mut style = Style::new()
        .with_direction(FlexDirection::Row)
        .with_justify_content(JustifyContent::Center)
        .with_align_items(AlignItems::Center)
        .with_fixed_size(
            page_dot_target_width(PAGE_DOT_SIZE),
            page_dot_target_height(),
        )
        .with_padding(PAGE_DOT_FOCUS_PADDING + PAGE_DOT_FOCUS_BORDER_WIDTH)
        .with_border_radius(BorderRadius::all(page_dot_target_height() / 2.0));

    if is_hovered {
        style.set_cursor(CursorIcon::PointingHand);
    }

    style
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
    interactions_disabled: bool,
) -> Element {
    let mut pointer = ctx.pointer();
    let morph_frame = page_dot_morph_frame(
        ctx,
        Id::new(PAGE_DOT_ACTIVE_VISUAL_TARGET_ID),
        Id::new(PAGE_DOT_ACTIVE_VISUAL_MORPH_ID),
        Some(active_page),
        Duration::from_millis(PAGE_DOT_ACTIVE_VISUAL_DURATION_MS),
    );
    if !interactions_disabled && pointer.just_pressed() {
        *ctx.use_shared_state(Id::new(HOME_APP_GRID_PAGE_STATE_ID), || 0)
            .write() = active_page;
    }
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
            trailing_min_scale: PAGE_DOT_ACTIVE_TRAILING_MIN_SCALE,
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

fn page_dot_focus_ring(
    ctx: &mut ComponentContext,
    page_count: usize,
    active_page: usize,
    focus_ring_page: Option<usize>,
    interactions_disabled: bool,
) -> Element {
    let mut pointer = ctx.pointer();
    let morph_frame = page_dot_morph_frame(
        ctx,
        Id::new(PAGE_DOT_FOCUS_RING_TARGET_ID),
        Id::new(PAGE_DOT_FOCUS_RING_MORPH_ID),
        focus_ring_page,
        Duration::from_millis(PAGE_DOT_FOCUS_RING_DURATION_MS),
    );
    let layout_page = focus_ring_page
        .or(morph_frame.from_page)
        .unwrap_or(active_page.min(page_count.saturating_sub(1)));
    if !interactions_disabled && pointer.just_pressed() {
        *ctx.use_shared_state(Id::new(HOME_APP_GRID_PAGE_STATE_ID), || 0)
            .write() = layout_page;
    }
    let source_page = morph_frame.from_page.unwrap_or(layout_page);
    let ring_path = build_liquid_ring_path(
        LiquidMorphSpec {
            from_center_x: page_dot_center_x(source_page),
            to_center_x: page_dot_center_x(morph_frame.target_page.unwrap_or(layout_page)),
            from_width: page_dot_focus_ring_width(source_page, active_page),
            to_width: page_dot_focus_ring_width(
                morph_frame.target_page.unwrap_or(layout_page),
                active_page,
            ),
            height: page_dot_target_height(),
            top_y: 0.0,
            neck_ratio: PAGE_DOT_FOCUS_RING_NECK_RATIO,
            trailing_min_scale: PAGE_DOT_FOCUS_RING_TRAILING_MIN_SCALE,
        },
        PAGE_DOT_FOCUS_BORDER_WIDTH,
        morph_frame.progress,
    );

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
        .with_clip_path(ring_path)
        .with_style(
            Style::new()
                .with_absolute_position(Vec2::new(0.0, 0.0))
                .with_fixed_size(
                    page_dots_track_width(page_count, active_page),
                    page_dot_target_height(),
                )
                .with_background_color(border_color)
                .with_order(2),
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

fn build_liquid_morph_path(spec: LiquidMorphSpec, progress: f32) -> Path {
    let mut path_builder = Path::builder();
    append_liquid_morph(&mut path_builder, spec, progress, Winding::Positive);
    path_builder.build()
}

fn build_liquid_ring_path(spec: LiquidMorphSpec, border_width: f32, progress: f32) -> Path {
    let mut path_builder = Path::builder();
    append_liquid_morph(&mut path_builder, spec, progress, Winding::Positive);

    let inner_height = (spec.height - border_width * 2.0).max(0.0);
    if inner_height > 0.0 {
        let inner_neck_height =
            (spec.height * spec.neck_ratio - border_width * 2.0).max(inner_height * 0.2);
        append_liquid_morph(
            &mut path_builder,
            LiquidMorphSpec {
                from_center_x: spec.from_center_x,
                to_center_x: spec.to_center_x,
                from_width: (spec.from_width - border_width * 2.0).max(inner_height),
                to_width: (spec.to_width - border_width * 2.0).max(inner_height),
                height: inner_height,
                top_y: spec.top_y + border_width,
                neck_ratio: (inner_neck_height / inner_height).clamp(0.2, 1.0),
                trailing_min_scale: spec.trailing_min_scale,
            },
            progress,
            Winding::Negative,
        );
    }

    path_builder.build()
}

fn append_liquid_morph(
    path_builder: &mut daiko::lyon::path::Builder,
    spec: LiquidMorphSpec,
    progress: f32,
    winding: Winding,
) {
    let progress = progress.clamp(0.0, 1.0);
    if (spec.from_center_x - spec.to_center_x).abs() < 0.01
        && (spec.from_width - spec.to_width).abs() < 0.01
    {
        add_capsule(
            path_builder,
            spec.to_center_x,
            spec.to_width,
            spec.height,
            spec.top_y,
            winding,
        );
        return;
    }

    let shape = liquid_span(spec, progress);
    if shape.right_x - shape.left_x <= spec.height {
        add_capsule_from_edges(
            path_builder,
            shape.left_x,
            shape.right_x,
            spec.height,
            spec.top_y,
            winding,
        );
        return;
    }

    add_single_drop_contour(path_builder, spec, shape, winding);
}

#[derive(Clone, Copy)]
struct LiquidSpan {
    left_x: f32,
    right_x: f32,
    neck_inset: f32,
    neck_center: f32,
    neck_width: f32,
}

fn liquid_span(spec: LiquidMorphSpec, progress: f32) -> LiquidSpan {
    let from_left = spec.from_center_x - spec.from_width / 2.0;
    let from_right = spec.from_center_x + spec.from_width / 2.0;
    let to_left = spec.to_center_x - spec.to_width / 2.0;
    let to_right = spec.to_center_x + spec.to_width / 2.0;
    let direction = (spec.to_center_x - spec.from_center_x).signum();
    let leading_progress = ease_out_cubic(progress);
    let trailing_progress = ease_in_cubic(progress);
    let (left_x, right_x) = if direction >= 0.0 {
        (
            lerp(from_left, to_left, trailing_progress),
            lerp(from_right, to_right, leading_progress),
        )
    } else {
        (
            lerp(from_left, to_left, leading_progress),
            lerp(from_right, to_right, trailing_progress),
        )
    };
    let stretch_progress = (std::f32::consts::PI * progress).sin().max(0.0);
    let neck_inset =
        (spec.height - spec.height * lerp(1.0, spec.neck_ratio, stretch_progress)) / 2.0;
    let neck_center = 0.5 - direction * 0.12 * stretch_progress;
    let neck_width = lerp(0.2, 0.34, 1.0 - stretch_progress * 0.45);

    LiquidSpan {
        left_x: left_x.min(right_x),
        right_x: left_x.max(right_x),
        neck_inset,
        neck_center,
        neck_width,
    }
}

fn add_single_drop_contour(
    path_builder: &mut daiko::lyon::path::Builder,
    spec: LiquidMorphSpec,
    span: LiquidSpan,
    winding: Winding,
) {
    let top_y = spec.top_y;
    let bottom_y = spec.top_y + spec.height;
    let middle_y = spec.top_y + spec.height / 2.0;
    let radius = spec.height / 2.0;
    let left_arc_end_x = span.left_x + radius;
    let right_arc_start_x = span.right_x - radius;

    if right_arc_start_x <= left_arc_end_x {
        add_capsule_from_edges(
            path_builder,
            span.left_x,
            span.right_x,
            spec.height,
            spec.top_y,
            winding,
        );
        return;
    }

    let mut top_points = Vec::with_capacity(LIQUID_PATH_SAMPLES + 1);
    let mut bottom_points = Vec::with_capacity(LIQUID_PATH_SAMPLES + 1);
    for sample_index in 0..=LIQUID_PATH_SAMPLES {
        let t = sample_index as f32 / LIQUID_PATH_SAMPLES as f32;
        let x = lerp(left_arc_end_x, right_arc_start_x, t);
        let inset = span.neck_inset * gaussian_profile(t, span.neck_center, span.neck_width);
        top_points.push(Pos2::new(x, top_y + inset));
        bottom_points.push(Pos2::new(x, bottom_y - inset));
    }

    let kappa = radius * CIRCLE_KAPPA;
    let left_middle = Pos2::new(span.left_x, middle_y);
    let right_middle = Pos2::new(span.right_x, middle_y);
    let left_top = Pos2::new(left_arc_end_x, top_points[0].y);
    let right_top = Pos2::new(right_arc_start_x, top_points[top_points.len() - 1].y);
    let right_bottom = Pos2::new(right_arc_start_x, bottom_points[bottom_points.len() - 1].y);
    let left_bottom = Pos2::new(left_arc_end_x, bottom_points[0].y);

    if matches!(winding, Winding::Positive) {
        path_builder.begin(left_middle);
        path_builder.cubic_bezier_to(
            Pos2::new(span.left_x, middle_y - kappa),
            Pos2::new(left_arc_end_x - kappa, top_y),
            left_top,
        );
        append_points(path_builder, &top_points[1..]);
        path_builder.cubic_bezier_to(
            Pos2::new(right_arc_start_x + kappa, top_y),
            Pos2::new(span.right_x, middle_y - kappa),
            right_middle,
        );
        path_builder.cubic_bezier_to(
            Pos2::new(span.right_x, middle_y + kappa),
            Pos2::new(right_arc_start_x + kappa, bottom_y),
            right_bottom,
        );
        append_points(
            path_builder,
            bottom_points[..bottom_points.len() - 1].iter().rev(),
        );
        path_builder.cubic_bezier_to(
            Pos2::new(left_arc_end_x - kappa, bottom_y),
            Pos2::new(span.left_x, middle_y + kappa),
            left_middle,
        );
        path_builder.close();
    } else {
        path_builder.begin(left_middle);
        path_builder.cubic_bezier_to(
            Pos2::new(span.left_x, middle_y + kappa),
            Pos2::new(left_arc_end_x - kappa, bottom_y),
            left_bottom,
        );
        append_points(path_builder, bottom_points[1..].iter());
        path_builder.cubic_bezier_to(
            Pos2::new(right_arc_start_x + kappa, bottom_y),
            Pos2::new(span.right_x, middle_y + kappa),
            right_middle,
        );
        path_builder.cubic_bezier_to(
            Pos2::new(span.right_x, middle_y - kappa),
            Pos2::new(right_arc_start_x + kappa, top_y),
            right_top,
        );
        append_points(
            path_builder,
            top_points[..top_points.len() - 1].iter().rev(),
        );
        path_builder.cubic_bezier_to(
            Pos2::new(left_arc_end_x - kappa, top_y),
            Pos2::new(span.left_x, middle_y - kappa),
            left_middle,
        );
        path_builder.close();
    }
}

fn append_points<'a>(
    path_builder: &mut daiko::lyon::path::Builder,
    points: impl IntoIterator<Item = &'a Pos2>,
) {
    for point in points {
        path_builder.line_to(*point);
    }
}

fn gaussian_profile(position: f32, center: f32, width: f32) -> f32 {
    let normalized = (position - center) / width.max(0.05);
    (-normalized * normalized).exp()
}

fn ease_in_cubic(progress: f32) -> f32 {
    progress * progress * progress
}

fn ease_out_cubic(progress: f32) -> f32 {
    1.0 - (1.0 - progress).powi(3)
}

fn add_capsule(
    path_builder: &mut daiko::lyon::path::Builder,
    center_x: f32,
    width: f32,
    height: f32,
    top_y: f32,
    winding: Winding,
) {
    add_capsule_from_edges(
        path_builder,
        center_x - width / 2.0,
        center_x + width / 2.0,
        height,
        top_y,
        winding,
    );
}

fn add_capsule_from_edges(
    path_builder: &mut daiko::lyon::path::Builder,
    left_x: f32,
    right_x: f32,
    height: f32,
    top_y: f32,
    winding: Winding,
) {
    let width = (right_x - left_x).max(0.0);
    if width <= 0.0 || height <= 0.0 {
        return;
    }

    let area = Rect {
        min: Pos2::new(left_x, top_y),
        max: Pos2::new(right_x, top_y + height),
    };
    let radius = (height / 2.0).min(width / 2.0);
    path_builder.add_rounded_rectangle(&area, &BorderRadii::new(radius), winding);
}

fn lerp(from: f32, to: f32, progress: f32) -> f32 {
    from + (to - from) * progress
}

fn focused_page_index(focused_key: Option<FocusKey>, page_count: usize) -> Option<usize> {
    let focused_key = focused_key?;
    (0..page_count).find(|page_index| page_dot_focus_key(*page_index) == focused_key)
}

fn page_dots_track_width(page_count: usize, _active_page: usize) -> f32 {
    page_dots_compact_track_width(page_count) + page_dot_track_side_inset() * 2.0
}

fn page_dot_target_left(page_index: usize) -> f32 {
    page_dot_track_side_inset()
        + page_index as f32 * (page_dot_target_width(PAGE_DOT_SIZE) + PAGE_DOTS_GAP)
}

fn page_dot_focus_ring_width(page_index: usize, active_page: usize) -> f32 {
    if page_index == active_page {
        page_dot_target_width(ACTIVE_PAGE_DOT_WIDTH)
    } else {
        page_dot_target_width(PAGE_DOT_SIZE)
    }
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

fn clicked_page_index(
    pointer_position: Option<Pos2>,
    layout: Option<Layout>,
    page_count: usize,
) -> Option<usize> {
    let pointer_position = pointer_position?;
    let layout = layout?;
    let visible_area = layout.visible_area;
    if pointer_position.x < visible_area.min.x
        || pointer_position.x > visible_area.max.x
        || pointer_position.y < visible_area.min.y
        || pointer_position.y > visible_area.max.y
    {
        return None;
    }

    (0..page_count).min_by(|left, right| {
        let left_distance =
            (page_dot_center_x(*left) - (pointer_position.x - visible_area.min.x)).abs();
        let right_distance =
            (page_dot_center_x(*right) - (pointer_position.x - visible_area.min.x)).abs();
        left_distance.total_cmp(&right_distance)
    })
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
mod tests {
    use super::*;
    use daiko::lyon::algorithms::hit_test::hit_test_path;
    use daiko::lyon::lyon_tessellation::FillRule;

    #[test]
    fn liquid_morph_path_builds_a_bridge_between_pages() {
        let path = build_liquid_morph_path(
            LiquidMorphSpec {
                from_center_x: 10.0,
                to_center_x: 38.0,
                from_width: ACTIVE_PAGE_DOT_WIDTH,
                to_width: ACTIVE_PAGE_DOT_WIDTH,
                height: PAGE_DOT_SIZE,
                top_y: 0.0,
                neck_ratio: PAGE_DOT_ACTIVE_NECK_RATIO,
                trailing_min_scale: PAGE_DOT_ACTIVE_TRAILING_MIN_SCALE,
            },
            0.5,
        );

        assert!(hit_test_path(
            &Pos2::new(24.0, PAGE_DOT_SIZE / 2.0),
            &path,
            FillRule::EvenOdd,
            0.1,
        ));
    }

    #[test]
    fn liquid_ring_path_keeps_the_center_hollow() {
        let path = build_liquid_ring_path(
            LiquidMorphSpec {
                from_center_x: 16.0,
                to_center_x: 16.0,
                from_width: page_dot_target_width(PAGE_DOT_SIZE),
                to_width: page_dot_target_width(PAGE_DOT_SIZE),
                height: page_dot_target_height(),
                top_y: 0.0,
                neck_ratio: PAGE_DOT_FOCUS_RING_NECK_RATIO,
                trailing_min_scale: PAGE_DOT_FOCUS_RING_TRAILING_MIN_SCALE,
            },
            PAGE_DOT_FOCUS_BORDER_WIDTH,
            1.0,
        );

        assert!(hit_test_path(
            &Pos2::new(16.0, 1.0),
            &path,
            FillRule::EvenOdd,
            0.1,
        ));
        assert!(!hit_test_path(
            &Pos2::new(16.0, page_dot_target_height() / 2.0),
            &path,
            FillRule::EvenOdd,
            0.1,
        ));
    }
}
