use daiko::{
    style::Color,
    widgets::image::{Image, ImageParams, ImageSource, ImageType},
};

pub fn svg_icon(svg: &'static [u8], icon_size: usize, color: Color) -> Image {
    Image::new(ImageParams {
        max_width: icon_size,
        max_height: icon_size,
        image_type: Some(ImageType::Svg),
        source: ImageSource::BytesSlice(svg),
    })
    .fill_color(Some(color))
}
