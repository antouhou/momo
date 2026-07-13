use super::style::{
    BACKDROP_MAX_OPACITY, DESTINATION_ICON_RADIUS, DESTINATION_LABELS_GAP,
    DESTINATION_LABELS_TOP_GAP, LAUNCH_SURFACE_BACKGROUND, LAUNCH_TILE_META_GAP,
    SURFACE_BORDER_WIDTH, backdrop_style, destination_icon_glyph_style, destination_icon_style,
    destination_subtitle_style, destination_title_style, labels_offset_style, launch_surface_style,
    launch_tile_content_style, launch_tile_icon_placeholder_style, launch_tile_icon_style,
    shared_content_style, with_opacity,
};
use crate::components::home::{
    app_icon::{app_icon, app_icon_background_color},
    launch::{
        AnimatedRect, HOME_HERO_ICON_GLYPH_SIZE, HOME_HERO_ICON_SIZE, HOME_LAUNCH_ANIMATION_ID,
        HOME_LAUNCH_BACKDROP_TAG, HOME_LAUNCH_OVERLAY_EVENT_CHANNEL_ID, HOME_LAUNCH_SURFACE_RADIUS,
        HOME_LAUNCH_SURFACE_TAG, HOME_SHARED_CONTENT_WIDTH, LaunchOverlayEvent, LaunchPhase,
        LaunchTransitionState,
    },
    model::{LaunchRequest, TILE_BORDER_RADIUS, TILE_ICON_GLYPH_SIZE, TILE_ICON_SIZE},
};
use daiko::{
    Element, Id, Vec2,
    animation::{
        AnimationParameters, Interpolable as _,
        easing::{Easing as _, EasingFunction},
    },
    component::{Component, ComponentContext},
    widgets::{
        container::{Container, Fit},
        overlay::Overlay,
        text::Text,
    },
};
use std::{sync::Arc, time::Duration};

#[derive(Clone)]
pub(in crate::components::home) struct LaunchOverlay {
    pub launch: LaunchTransitionState,
}

#[derive(Clone, Default)]
struct LaunchOverlayAnimationState {
    current_app_id: Option<Arc<String>>,
    current_phase: Option<LaunchPhase>,
    expanded_event_sent: bool,
    contracted_event_sent: bool,
}

impl Component for LaunchOverlay {
    fn to_element(&self, ctx: &mut ComponentContext) -> Element {
        let animation_progress = update_launch_animation(ctx, self.launch.clone());
        render_launch_overlay(ctx, self.launch.clone(), animation_progress)
    }
}

fn update_launch_animation(ctx: &mut ComponentContext, launch: LaunchTransitionState) -> f32 {
    let overlay_event_channel = ctx.use_channel_with_id(HOME_LAUNCH_OVERLAY_EVENT_CHANNEL_ID);
    let animation_state_handle = ctx.use_local_state(LaunchOverlayAnimationState::default);
    let mut animation_state = animation_state_handle.read().clone();
    let launch_animation = ctx.animation_with_id(
        Id::new(HOME_LAUNCH_ANIMATION_ID),
        AnimationParameters::default()
            .with_duration(Duration::from_millis(360))
            .with_easing(EasingFunction::EaseInOut),
    );

    if animation_state
        .current_app_id
        .as_deref()
        .map(String::as_str)
        != Some(launch.request.app.id())
    {
        animation_state = LaunchOverlayAnimationState {
            current_app_id: Some(Arc::clone(&launch.request.app.id)),
            current_phase: Some(launch.phase),
            expanded_event_sent: false,
            contracted_event_sent: false,
        };
        launch_animation.set_progress(0.0);
        launch_animation.play_forward();
    } else if animation_state.current_phase != Some(launch.phase) {
        animation_state.current_phase = Some(launch.phase);
        match launch.phase {
            LaunchPhase::Expanding => {
                animation_state.expanded_event_sent = false;
                animation_state.contracted_event_sent = false;
                launch_animation.set_progress(0.0);
                launch_animation.play_forward();
            }
            LaunchPhase::WaitingForSurface => {
                launch_animation.set_progress(1.0);
            }
            LaunchPhase::Contracting => {
                animation_state.contracted_event_sent = false;
                launch_animation.play_backward();
            }
        }
    }

    let animation_progress = launch_animation.progress_linear();

    if launch.phase == LaunchPhase::Expanding
        && !launch_animation.is_running()
        && animation_progress >= 1.0
        && !animation_state.expanded_event_sent
    {
        let _ = overlay_event_channel.send(LaunchOverlayEvent::Expanded {
            app_id: Arc::clone(&launch.request.app.id),
        });
        animation_state.expanded_event_sent = true;
    }

    if launch.phase == LaunchPhase::Contracting
        && !launch_animation.is_running()
        && animation_progress <= 0.0
        && !animation_state.contracted_event_sent
    {
        let _ = overlay_event_channel.send(LaunchOverlayEvent::Contracted {
            app_id: Arc::clone(&launch.request.app.id),
        });
        animation_state.contracted_event_sent = true;
    }

    *animation_state_handle.write_silent() = animation_state;

    animation_progress
}

fn render_launch_overlay(
    ctx: &mut ComponentContext,
    launch: LaunchTransitionState,
    animation_progress: f32,
) -> Element {
    let viewport_size = ctx.viewport().unwrap_or_default().size().to_vector();
    let target = AnimatedRect {
        position: Vec2::zero(),
        size: viewport_size,
    };
    let source = AnimatedRect {
        position: launch.request.position,
        size: launch.request.size,
    };
    let surface_rect = launch_surface_rect(source, target, animation_progress);
    let accent = launch.request.app.accent;
    let border_color = accent.gamma_multiply(0.9);
    let backdrop_opacity = if launch.phase == LaunchPhase::WaitingForSurface {
        BACKDROP_MAX_OPACITY
    } else {
        BACKDROP_MAX_OPACITY
            * EasingFunction::EaseOut.apply(interval(animation_progress, 0.08, 0.5))
    };
    let destination_icon_opacity = 1.0;
    // let destination_icon_opacity = if launch.phase == LaunchPhase::WaitingForSurface {
    //     1.0
    // } else {
    //     EasingFunction::EaseOut.apply(interval(animation_progress, 0.0, 0.32))
    // };
    let destination_labels_opacity = if launch.phase == LaunchPhase::WaitingForSurface {
        1.0
    } else {
        EasingFunction::EaseOut.apply(interval(animation_progress, 0.54, 0.86))
    };
    let final_icon_center = Vec2::new(viewport_size.x / 2.0, viewport_size.y / 2.0);
    let source_surface_center = launch.request.position + launch.request.size / 2.0;
    let current_surface_center = surface_rect.position + surface_rect.size / 2.0;
    let source_icon_center = launch.request.icon_position + launch.request.icon_size / 2.0;
    let icon_progress_x = if launch.phase == LaunchPhase::WaitingForSurface {
        1.0
    } else {
        EasingFunction::EaseInOut.apply(axis_progress(
            current_surface_center.x,
            source_surface_center.x,
            final_icon_center.x,
        ))
    };
    let icon_progress_y = if launch.phase == LaunchPhase::WaitingForSurface {
        1.0
    } else {
        EasingFunction::EaseInOut.apply(axis_progress(
            current_surface_center.y,
            source_surface_center.y,
            final_icon_center.y,
        ))
    };
    let width_progress = if launch.phase == LaunchPhase::WaitingForSurface {
        1.0
    } else {
        axis_progress(surface_rect.size.x, launch.request.size.x, viewport_size.x)
    };
    let height_progress = if launch.phase == LaunchPhase::WaitingForSurface {
        1.0
    } else {
        axis_progress(surface_rect.size.y, launch.request.size.y, viewport_size.y)
    };
    let icon_scale_progress =
        EasingFunction::EaseInOut.apply(((width_progress + height_progress) * 0.5).clamp(0.0, 1.0));
    let radius_progress = if launch.phase == LaunchPhase::WaitingForSurface {
        1.0
    } else {
        EasingFunction::EaseInOut.apply(interval(animation_progress, 0.22, 0.92))
    };
    let border_progress = if launch.phase == LaunchPhase::WaitingForSurface {
        1.0
    } else {
        EasingFunction::EaseOut.apply(interval(animation_progress, 0.4, 0.92))
    };
    let border_radius =
        TILE_BORDER_RADIUS + (HOME_LAUNCH_SURFACE_RADIUS - TILE_BORDER_RADIUS) * radius_progress;
    let border_width = SURFACE_BORDER_WIDTH * (1.0 - border_progress);
    let surface_content_size = Vec2::new(
        (surface_rect.size.x - border_width * 2.0).max(0.0),
        (surface_rect.size.y - border_width * 2.0).max(0.0),
    );

    let backdrop = Element::new()
        .with_tag(HOME_LAUNCH_BACKDROP_TAG)
        .with_style(backdrop_style(viewport_size, backdrop_opacity));

    let tile_content = build_launch_tile_content(launch.request.clone(), false);
    let shared_app_content =
        build_launch_destination_shared_content(LaunchDestinationSharedContentProps {
            request: launch.request.clone(),
            source_icon_center,
            source_icon_size: launch.request.icon_size.x,
            final_icon_center,
            icon_opacity: destination_icon_opacity,
            labels_opacity: destination_labels_opacity,
            motion_progress: Vec2::new(icon_progress_x, icon_progress_y),
            scale_progress: icon_scale_progress,
        });

    let surface = Element::new()
        .with_tag(HOME_LAUNCH_SURFACE_TAG)
        .with_style(launch_surface_style(
            surface_rect.position,
            surface_content_size,
            LAUNCH_SURFACE_BACKGROUND,
            border_color,
            border_width,
            border_radius,
        ))
        .with_content(tile_content);

    Overlay::new_fullscreen(
        ctx,
        Element::new()
            .with_content(backdrop)
            .with_content(surface)
            .with_content(shared_app_content),
    )
    .to_element(ctx)
}

fn build_launch_tile_content(request: LaunchRequest, show_icon: bool) -> Element {
    let accent = request.app.accent;
    let icon_background = app_icon_background_color(accent);

    let icon = Element::new()
        .with_style(launch_tile_icon_style(icon_background))
        .with_content(app_icon(&request.app.icon, TILE_ICON_GLYPH_SIZE));

    let icon_slot = if show_icon {
        icon
    } else {
        Element::new().with_style(launch_tile_icon_placeholder_style())
    };

    let meta = Container::vertical()
        .with_fit(Fit::new().at_least_parent_width().at_least_content_height())
        .with_spacing((LAUNCH_TILE_META_GAP, LAUNCH_TILE_META_GAP))
        .build();
    // let meta = Container::vertical()
    //     .with_fit(Fit::new().at_least_parent_width().at_least_content_height())
    //     .with_spacing((4.0, 4.0))
    //     .build()
    //     .with_content(
    //         Text::new(request.app.name).with_style(
    //             TextStyle::default()
    //                 .with_font_size(18.0)
    //                 .with_font_color(Color::from_rgb(240, 245, 255)),
    //         ),
    //     )
    //     .with_content(
    //         Text::new(request.app.subtitle).with_style(
    //             TextStyle::default()
    //                 .with_font_size(13.0)
    //                 .with_font_color(Color::from_rgb(132, 149, 179)),
    //         ),
    //     );

    Element::new()
        .with_style(launch_tile_content_style(request.size))
        .with_content(icon_slot)
        .with_content(meta)
}

#[derive(Clone)]
struct LaunchDestinationSharedContentProps {
    request: LaunchRequest,
    source_icon_center: Vec2,
    source_icon_size: f32,
    final_icon_center: Vec2,
    icon_opacity: f32,
    labels_opacity: f32,
    motion_progress: Vec2,
    scale_progress: f32,
}

fn build_launch_destination_shared_content(props: LaunchDestinationSharedContentProps) -> Element {
    let accent = props.request.app.accent;
    let current_icon_center = Vec2::new(
        props.source_icon_center.x
            + (props.final_icon_center.x - props.source_icon_center.x) * props.motion_progress.x,
        props.source_icon_center.y
            + (props.final_icon_center.y - props.source_icon_center.y) * props.motion_progress.y,
    );
    let current_icon_size = props.source_icon_size
        + (HOME_HERO_ICON_SIZE - props.source_icon_size) * props.scale_progress;
    let source_icon_scale = props.source_icon_size / TILE_ICON_SIZE;
    let icon_radius = DESTINATION_ICON_RADIUS.source * source_icon_scale
        + (DESTINATION_ICON_RADIUS.target - DESTINATION_ICON_RADIUS.source * source_icon_scale)
            * props.scale_progress;
    let current_icon_top_left =
        current_icon_center - Vec2::new(current_icon_size / 2.0, current_icon_size / 2.0);
    let local_icon_x = (HOME_SHARED_CONTENT_WIDTH - current_icon_size) / 2.0;
    let local_icon_y = 0.0;
    let current_shared_content_top_left = Vec2::new(
        current_icon_top_left.x - local_icon_x,
        current_icon_top_left.y - local_icon_y,
    );
    let source_icon_glyph_size = TILE_ICON_GLYPH_SIZE as f32 * source_icon_scale;
    let icon_glyph_size = (source_icon_glyph_size
        + (HOME_HERO_ICON_GLYPH_SIZE as f32 - source_icon_glyph_size) * props.scale_progress)
        .round()
        .max(1.0);
    let icon = Element::new()
        .with_style(destination_icon_style(
            Vec2::new(local_icon_x, local_icon_y),
            current_icon_size,
            with_opacity(app_icon_background_color(accent), props.icon_opacity),
            icon_radius,
        ))
        .with_content(
            app_icon(&props.request.app.icon, HOME_HERO_ICON_GLYPH_SIZE)
                .with_style(destination_icon_glyph_style(icon_glyph_size))
                .fit_to_container(true),
        );

    let labels = Container::vertical()
        .with_fit(
            Fit::new()
                .exact_width(HOME_SHARED_CONTENT_WIDTH)
                .exact_content_height(),
        )
        .align_items_center()
        .with_spacing((DESTINATION_LABELS_GAP, DESTINATION_LABELS_GAP))
        .build()
        .with_content(
            Text::new(Arc::clone(&props.request.app.name))
                .with_style(destination_title_style(props.labels_opacity)),
        )
        .with_content(
            Text::new("Opening app").with_style(destination_subtitle_style(props.labels_opacity)),
        );

    Element::new()
        .with_tag(format!("launch-overlay-app-{}", props.request.app.id()))
        .with_style(shared_content_style(current_shared_content_top_left))
        .with_content(icon)
        .with_content(
            Element::new()
                .with_style(labels_offset_style(
                    current_icon_size + DESTINATION_LABELS_TOP_GAP,
                ))
                .with_content(labels),
        )
}

fn launch_surface_rect(source: AnimatedRect, target: AnimatedRect, t: f32) -> AnimatedRect {
    // if t <= 0.12 {
    //     return interpolate_rect(
    //         source,
    //         scale_rect(source, HOME_LAUNCH_PRESS_SCALE),
    //         EasingFunction::EaseOut.apply(interval(t, 0.0, 0.12)),
    //     );
    // }
    //
    // let rebound = scale_rect(source, HOME_LAUNCH_REBOUND_SCALE);
    // if t <= 0.24 {
    //     return interpolate_rect(
    //         scale_rect(source, HOME_LAUNCH_PRESS_SCALE),
    //         rebound,
    //         EasingFunction::EaseOut.apply(interval(t, 0.12, 0.24)),
    //     );
    // }
    //
    // interpolate_rect(
    //     rebound,
    //     target,
    //     EasingFunction::EaseInOut.apply(interval(t, 0.2, 1.0)),
    // )

    interpolate_rect(
        source,
        target,
        EasingFunction::EaseInOut.apply(interval(t, 0.0, 1.0)),
    )
}

fn interpolate_rect(from: AnimatedRect, to: AnimatedRect, t: f32) -> AnimatedRect {
    AnimatedRect {
        position: from.position.interpolate(&to.position, t),
        size: from.size.interpolate(&to.size, t),
    }
}

// fn scale_rect(rect: AnimatedRect, scale: f32) -> AnimatedRect {
//     let scaled_size = rect.size * scale;
//     let offset = (rect.size - scaled_size) / 2.0;
//     AnimatedRect {
//         position: rect.position + offset,
//         size: scaled_size,
//     }
// }

fn interval(value: f32, start: f32, end: f32) -> f32 {
    if end <= start {
        return 1.0;
    }
    ((value - start) / (end - start)).clamp(0.0, 1.0)
}

fn axis_progress(current: f32, start: f32, end: f32) -> f32 {
    let delta = end - start;
    if delta.abs() <= f32::EPSILON {
        return 1.0;
    }
    ((current - start) / delta).clamp(0.0, 1.0)
}
