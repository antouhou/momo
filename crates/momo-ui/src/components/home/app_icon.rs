use crate::components::home::model::MockAppIcon;
use daiko::style::Color;
use daiko::widgets::image::{Image, ImageParams, ImageSource, ImageType};

pub(super) fn mock_app_icon_background_color(accent: Color) -> Color {
    accent.gamma_multiply(0.2)
}

pub(super) fn mock_app_icon_foreground_color(accent: Color) -> Color {
    accent.gamma_multiply(1.1)
}

pub(super) fn mock_app_icon(icon: MockAppIcon, size: usize, color: Color) -> Image {
    Image::new(ImageParams {
        max_width: size,
        max_height: size,
        image_type: Some(ImageType::Svg),
        source: ImageSource::BytesSlice(mock_app_icon_svg(icon)),
    })
    .fill_color(Some(color))
}

fn mock_app_icon_svg(icon: MockAppIcon) -> &'static [u8] {
    match icon {
        MockAppIcon::LiveTv => LIVE_TV_ICON,
        MockAppIcon::Movies => MOVIES_ICON,
        MockAppIcon::Music => MUSIC_ICON,
        MockAppIcon::Photos => PHOTOS_ICON,
        MockAppIcon::Browser => BROWSER_ICON,
        MockAppIcon::Settings => SETTINGS_ICON,
        MockAppIcon::Store => STORE_ICON,
        MockAppIcon::Files => FILES_ICON,
        MockAppIcon::Games => GAMES_ICON,
        MockAppIcon::News => NEWS_ICON,
        MockAppIcon::Weather => WEATHER_ICON,
        MockAppIcon::Sports => SPORTS_ICON,
        MockAppIcon::Podcasts => PODCASTS_ICON,
        MockAppIcon::Calendar => CALENDAR_ICON,
        MockAppIcon::Mail => MAIL_ICON,
        MockAppIcon::Camera => CAMERA_ICON,
        MockAppIcon::Terminal => TERMINAL_ICON,
        MockAppIcon::Sleep => SLEEP_ICON,
        MockAppIcon::Search => SEARCH_ICON,
        MockAppIcon::Kids => KIDS_ICON,
        MockAppIcon::Stream => STREAM_ICON,
        MockAppIcon::Radio => RADIO_ICON,
        MockAppIcon::Books => BOOKS_ICON,
        MockAppIcon::Fitness => FITNESS_ICON,
        MockAppIcon::Calls => CALLS_ICON,
        MockAppIcon::Travel => TRAVEL_ICON,
        MockAppIcon::Recipes => RECIPES_ICON,
        MockAppIcon::Security => SECURITY_ICON,
    }
}

const LIVE_TV_ICON: &[u8] = include_bytes!("../../../assets/window-maximize.svg");
const MOVIES_ICON: &[u8] = include_bytes!("../../../assets/file-video.svg");
const MUSIC_ICON: &[u8] = include_bytes!("../../../assets/file-audio.svg");
const PHOTOS_ICON: &[u8] = include_bytes!("../../../assets/images.svg");
const BROWSER_ICON: &[u8] = include_bytes!("../../../assets/compass.svg");
const SETTINGS_ICON: &[u8] = include_bytes!("../../../assets/gear-solid-full.svg");
const STORE_ICON: &[u8] = include_bytes!("../../../assets/credit-card.svg");
const FILES_ICON: &[u8] = include_bytes!("../../../assets/folder-open.svg");
const GAMES_ICON: &[u8] = include_bytes!("../../../assets/chess-knight.svg");
const NEWS_ICON: &[u8] = include_bytes!("../../../assets/newspaper.svg");
const WEATHER_ICON: &[u8] = include_bytes!("../../../assets/cloud.svg");
const SPORTS_ICON: &[u8] = include_bytes!("../../../assets/futbol.svg");
const PODCASTS_ICON: &[u8] = include_bytes!("../../../assets/comment-dots.svg");
const CALENDAR_ICON: &[u8] = include_bytes!("../../../assets/calendar-days.svg");
const MAIL_ICON: &[u8] = include_bytes!("../../../assets/envelope.svg");
const CAMERA_ICON: &[u8] = include_bytes!("../../../assets/camera.svg");
const TERMINAL_ICON: &[u8] = include_bytes!("../../../assets/keyboard.svg");
const SLEEP_ICON: &[u8] = include_bytes!("../../../assets/moon.svg");
const SEARCH_ICON: &[u8] = include_bytes!("../../../assets/eye.svg");
const KIDS_ICON: &[u8] = include_bytes!("../../../assets/face-smile.svg");
const STREAM_ICON: &[u8] = include_bytes!("../../../assets/play-circle.svg");
const RADIO_ICON: &[u8] = include_bytes!("../../../assets/circle-dot.svg");
const BOOKS_ICON: &[u8] = include_bytes!("../../../assets/address-book.svg");
const FITNESS_ICON: &[u8] = include_bytes!("../../../assets/heart.svg");
const CALLS_ICON: &[u8] = include_bytes!("../../../assets/message.svg");
const TRAVEL_ICON: &[u8] = include_bytes!("../../../assets/map.svg");
const RECIPES_ICON: &[u8] = include_bytes!("../../../assets/clipboard.svg");
const SECURITY_ICON: &[u8] = include_bytes!("../../../assets/circle-check.svg");
