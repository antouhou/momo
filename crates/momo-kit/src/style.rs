use daiko::style::{Color, LinearGradient, LinearSideOrCorner};

/// The default text size used throughout interface
pub const SYSTEM_TEXT_SIZE: f32 = 18.0;

/// The text size for the header
pub const SYSTEM_TRAY_TEXT_SIZE: f32 = 22.0;

/// The deep-space-to-copper background
pub fn shell_background_gradient() -> LinearGradient {
    LinearGradient::to(LinearSideOrCorner::BottomRight)
        .stop_at_percent(0.00, Color::from_rgb(2, 3, 12))
        .stop_at_percent(0.28, Color::from_rgb(6, 10, 27))
        .stop_at_percent(0.52, Color::from_rgb(18, 19, 49))
        .stop_at_percent(0.72, Color::from_rgb(43, 31, 65))
        .stop_at_percent(0.90, Color::from_rgb(82, 51, 66))
        .stop_at_percent(1.00, Color::from_rgb(126, 75, 58))
}
