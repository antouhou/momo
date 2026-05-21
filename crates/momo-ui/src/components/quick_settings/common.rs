use super::style::QuickSettingsControlState;
use daiko::Element;
use daiko::component::ComponentContext;
use daiko::navigation::FocusOrigin;
use daiko::style::{Color, Style};
use daiko::widgets::image::{Image, ImageParams, ImageSource, ImageType};

#[derive(Clone, Copy)]
pub(super) enum QuickSettingsGlyph {
    Asset(&'static [u8]),
}

pub(super) fn glyph_element(
    glyph: QuickSettingsGlyph,
    icon_size: usize,
    frame_size: f32,
    icon_color: Color,
) -> Element {
    match glyph {
        QuickSettingsGlyph::Asset(svg) => centered_glyph_frame(frame_size).with_content(
            Image::new(ImageParams {
                max_width: icon_size,
                max_height: icon_size,
                image_type: Some(ImageType::Svg),
                source: ImageSource::BytesSlice(svg),
            })
            .fill_color(Some(icon_color)),
        ),
    }
}

pub(super) fn control_state(ctx: &mut ComponentContext) -> QuickSettingsControlState {
    let mut pointer = ctx.pointer();
    let focusable = ctx.focusable();

    if pointer.just_entered() || pointer.just_pressed() {
        focusable.request_focus(FocusOrigin::Pointer);
    }

    QuickSettingsControlState {
        is_hovered: pointer.is_hovering(),
        is_focused: focusable.is_focused(),
    }
}

fn centered_glyph_frame(size: f32) -> Element {
    Element::new().with_style(
        Style::new()
            .with_fixed_size(size, size)
            .with_direction(daiko::layout::FlexDirection::Row)
            .with_align_items(daiko::layout::AlignItems::Center)
            .with_justify_content(daiko::layout::JustifyContent::Center),
    )
}
