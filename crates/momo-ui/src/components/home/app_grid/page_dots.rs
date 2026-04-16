use daiko::component::{Component, ComponentContext};
use daiko::{Element, Id};
use daiko::layout::{AlignItems, FlexDirection, JustifyContent};
use daiko::navigation::{FocusBoundary, FocusOrigin};
use daiko::style::{Border, BorderRadius, Color, Stroke, Style};
use daiko::widgets::container::{Container, Fit};
use crate::components::home::app_grid::{page_dot_focus_key, ACTIVE_PAGE_DOT_WIDTH, PAGE_DOTS_GAP, PAGE_DOT_FOCUS_BORDER_WIDTH, PAGE_DOT_FOCUS_PADDING, PAGE_DOT_SIZE};
use crate::components::home::model::HOME_APP_GRID_PAGE_STATE_ID;

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

        let mut dots = Container::horizontal()
            .with_style(Style::new().with_background(Color::RED))
            .with_fit(Fit::new().exact_content_height())
            .align_items_center()
            .justify_content_center()
            .with_spacing((PAGE_DOTS_GAP, PAGE_DOTS_GAP))
            .build()
            .with_tag("apps-grid-page-dots");

        for page_index in 0..self.page_count {
            dots.add_content(PageDot {
                page_index,
                is_active: page_index == self.active_page,
                interactions_disabled: self.interactions_disabled,
            });
        }

        dots
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
        focusable.set_focus_key(page_dot_focus_key(self.page_index));
        focusable.set_navigation_enabled(!self.interactions_disabled);

        if !self.interactions_disabled && pointer.just_entered() {
            focusable.request_focus(FocusOrigin::Pointer);
        }

        let just_selected =
            !self.interactions_disabled && (pointer.just_pressed() || focusable.just_activated());
        if just_selected {
            *ctx.use_shared_state(Id::new(HOME_APP_GRID_PAGE_STATE_ID), || 0)
                .write() = self.page_index;
        }

        page_dot(
            self.page_index,
            self.is_active,
            focusable.is_focus_visible() || pointer.is_hovering(),
        )
    }
}

fn page_dot(page_index: usize, is_active: bool, show_border: bool) -> Element {
    let dot_width = page_dot_visual_width(is_active);
    Element::new()
        .with_tag(format!("apps-grid-page-dot-{page_index}"))
        .with_style(page_dot_target_style(show_border, dot_width))
        .with_content(page_dot_visual(page_index, is_active))
}

fn page_dot_target_style(show_border: bool, dot_width: f32) -> Style {
    let border_color = if show_border {
        Color::from_rgb(236, 246, 255)
    } else {
        Color::TRANSPARENT
    };
    let target_outset = (PAGE_DOT_FOCUS_PADDING + PAGE_DOT_FOCUS_BORDER_WIDTH) * 2.0;

    Style::new()
        .with_direction(FlexDirection::Row)
        .with_justify_content(JustifyContent::Center)
        .with_align_items(AlignItems::Center)
        .with_fixed_size(dot_width + target_outset, PAGE_DOT_SIZE + target_outset)
        .with_padding(PAGE_DOT_FOCUS_PADDING)
        .with_border(Border::uniform(Stroke::new(
            PAGE_DOT_FOCUS_BORDER_WIDTH,
            border_color,
        )))
        .with_border_radius(BorderRadius::all((PAGE_DOT_SIZE + target_outset) / 2.0))
}

fn page_dot_visual(page_index: usize, is_active: bool) -> Element {
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
                .with_background_color(color)
                .with_border_radius(BorderRadius::all(PAGE_DOT_SIZE / 2.0)),
        )
}

fn page_dot_visual_width(is_active: bool) -> f32 {
    if is_active {
        ACTIVE_PAGE_DOT_WIDTH
    } else {
        PAGE_DOT_SIZE
    }
}
