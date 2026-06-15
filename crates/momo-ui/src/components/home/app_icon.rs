use crate::components::home::model::{AppIcon, BuiltInAppIcon};
use daiko::style::Color;
use daiko::widgets::image::{Image, ImageParams, ImageSource, ImageType};

pub(super) fn app_icon_background_color(accent: Color) -> Color {
    accent.gamma_multiply(0.2)
}

pub(super) fn app_icon_foreground_color(accent: Color) -> Color {
    accent.gamma_multiply(1.1)
}

pub(super) fn app_icon(icon: &AppIcon, size: usize, fallback_color: Color) -> Image {
    match icon {
        AppIcon::BuiltIn(icon) => Image::new(ImageParams {
            max_width: size,
            max_height: size,
            image_type: Some(ImageType::Svg),
            source: ImageSource::BytesSlice(built_in_app_icon_svg(*icon)),
        })
        .fill_color(Some(fallback_color)),
        AppIcon::File(path) => Image::new(ImageParams {
            max_width: size,
            max_height: size,
            image_type: None,
            source: ImageSource::File(path.clone()),
        }),
    }
}

fn built_in_app_icon_svg(icon: BuiltInAppIcon) -> &'static [u8] {
    match icon {
        BuiltInAppIcon::LiveTv => LIVE_TV_ICON,
        BuiltInAppIcon::Movies => MOVIES_ICON,
        BuiltInAppIcon::Music => MUSIC_ICON,
        BuiltInAppIcon::Photos => PHOTOS_ICON,
        BuiltInAppIcon::Browser => BROWSER_ICON,
        BuiltInAppIcon::Settings => SETTINGS_ICON,
        BuiltInAppIcon::Store => STORE_ICON,
        BuiltInAppIcon::Files => FILES_ICON,
        BuiltInAppIcon::Games => GAMES_ICON,
        BuiltInAppIcon::News => NEWS_ICON,
        BuiltInAppIcon::Weather => WEATHER_ICON,
        BuiltInAppIcon::Sports => SPORTS_ICON,
        BuiltInAppIcon::Podcasts => PODCASTS_ICON,
        BuiltInAppIcon::Calendar => CALENDAR_ICON,
        BuiltInAppIcon::Mail => MAIL_ICON,
        BuiltInAppIcon::Camera => CAMERA_ICON,
        BuiltInAppIcon::Terminal => TERMINAL_ICON,
        BuiltInAppIcon::Sleep => SLEEP_ICON,
        BuiltInAppIcon::Search => SEARCH_ICON,
        BuiltInAppIcon::Kids => KIDS_ICON,
        BuiltInAppIcon::Stream => STREAM_ICON,
        BuiltInAppIcon::Radio => RADIO_ICON,
        BuiltInAppIcon::Books => BOOKS_ICON,
        BuiltInAppIcon::Fitness => FITNESS_ICON,
        BuiltInAppIcon::Calls => CALLS_ICON,
        BuiltInAppIcon::Travel => TRAVEL_ICON,
        BuiltInAppIcon::Recipes => RECIPES_ICON,
        BuiltInAppIcon::Security => SECURITY_ICON,
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
