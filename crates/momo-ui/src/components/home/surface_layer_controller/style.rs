use daiko::{
    Vec2,
    layout::{FlexItemPosition, ItemSize},
    style::Style,
};

pub(super) fn no_view_style() -> Style {
    Style::new()
        .with_position(FlexItemPosition::Absolute(Vec2::new(0.0, 0.0)))
        .with_fixed_width(ItemSize::Points(0.0))
        .with_fixed_height(ItemSize::Points(0.0))
}
