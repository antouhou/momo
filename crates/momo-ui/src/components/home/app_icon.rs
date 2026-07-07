use daiko::{
    style::Color,
    widgets::image::{Image, ImageParams, ImageSource},
};
use std::path::PathBuf;

pub(crate) fn app_icon_background_color(accent: Color) -> Color {
    accent.gamma_multiply(0.2)
}

pub(crate) fn app_icon(icon_path: &Option<PathBuf>, size: usize) -> Image {
    // match icon {
    // AppIcon::BuiltIn(icon) => Image::new(ImageParams {
    //     max_width: size,
    //     max_height: size,
    //     image_type: Some(ImageType::Svg),
    //     source: ImageSource::BytesSlice(built_in_app_icon_svg(*icon)),
    // })
    // .fill_color(Some(fallback_color)),
    // AppIcon::File(icon) => Image::new(ImageParams {
    //     max_width: size,
    //     max_height: size,
    //     image_type: None,
    //     source: ImageSource::File(path.clone()),
    // })
    // }
    match icon_path {
        Some(icon_path) => Image::new(ImageParams {
            max_width: size,
            max_height: size,
            image_type: None,
            source: ImageSource::File(icon_path.clone()),
        }),
        None => {
            Image::new(ImageParams {
                max_width: size,
                max_height: size,
                image_type: Some(daiko::widgets::image::ImageType::Svg),
                // TODO: make a default executable icon
                source: ImageSource::BytesSlice(SETTINGS_ICON),
            })
            .fill_color(Some(Color::WHITE))
        }
    }
}

const SETTINGS_ICON: &[u8] = include_bytes!("../../../assets/gear-solid-full.svg");
