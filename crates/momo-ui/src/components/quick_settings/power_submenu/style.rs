use super::super::style::SETTINGS_COMPACT_CONTENT_GAP;
use daiko::{layout::FlexDirection, style::Style};

pub(super) fn power_actions_style() -> Style {
    Style::new()
        .with_direction(FlexDirection::Column)
        .with_spacing((SETTINGS_COMPACT_CONTENT_GAP, SETTINGS_COMPACT_CONTENT_GAP))
}
