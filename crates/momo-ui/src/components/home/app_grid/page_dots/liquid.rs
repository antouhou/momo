use daiko::{BorderRadii, Path, Pos2, Rect, lyon::path::Winding};

const LIQUID_PATH_SAMPLES: usize = 14;
const CIRCLE_KAPPA: f32 = 0.552_284_8;

#[derive(Clone, Copy)]
pub(super) struct LiquidMorphSpec {
    pub(super) from_center_x: f32,
    pub(super) to_center_x: f32,
    pub(super) from_width: f32,
    pub(super) to_width: f32,
    pub(super) height: f32,
    pub(super) top_y: f32,
    pub(super) neck_ratio: f32,
}

pub(super) fn build_liquid_morph_path(spec: LiquidMorphSpec, progress: f32) -> Path {
    let mut path_builder = Path::builder();
    append_liquid_morph(&mut path_builder, spec, progress, Winding::Positive);
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
        add_capsule(path_builder, spec, winding);
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

    LiquidSpan {
        left_x: left_x.min(right_x),
        right_x: left_x.max(right_x),
        neck_inset,
        neck_center: 0.5 - direction * 0.12 * stretch_progress,
        neck_width: lerp(0.2, 0.34, 1.0 - stretch_progress * 0.45),
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
        let progress = sample_index as f32 / LIQUID_PATH_SAMPLES as f32;
        let x = lerp(left_arc_end_x, right_arc_start_x, progress);
        let inset = span.neck_inset * gaussian_profile(progress, span.neck_center, span.neck_width);
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
    }
    path_builder.close();
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
    spec: LiquidMorphSpec,
    winding: Winding,
) {
    add_capsule_from_edges(
        path_builder,
        spec.to_center_x - spec.to_width / 2.0,
        spec.to_center_x + spec.to_width / 2.0,
        spec.height,
        spec.top_y,
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
