use daiko::Element;
use daiko::style::{Border, BorderRadius, Color, Indent, Stroke, Style};
use daiko::widgets::text::{Text, TextStyle, TextWrap};

pub(super) fn clock_chip(time: String) -> Element {
    Element::new()
        .with_tag("clock-chip")
        .with_style(
            Style::new()
                .with_padding(Indent::from((18.0, 12.0)))
                .with_size_constraint(daiko::layout::SizeConstraint::exact_content_size()),
        )
        .with_content(
            Text::new(time).with_style(
                TextStyle::default()
                    .with_font_size(24.0)
                    .with_font_color(Color::from_rgb(232, 238, 250))
                    .with_wrap(TextWrap::NoWrap),
            ),
        )
}
