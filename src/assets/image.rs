use crate::{
    assets_path,
    types::{AssetImage, ImageFormat, TextureSize},
};
use image::DynamicImage;

const PATH: &str = "images";

impl AssetImage {
    pub fn load(filename: &str) -> Self {
        let path = assets_path().join(PATH).join(&filename);
        let extension = path
            .extension()
            .expect("Failed to get file extension.")
            .to_str()
            .expect("Failed to convert file extension to str.");

        // TODO support more than 3 channels
        let image = match extension {
            "jpg" | "png" => image::open(path).expect("Failed to load image."),
            _ => panic!("Unsupported image extension: {}", extension),
        };

        if image.width() > i32::MAX as u32 {
            panic!("Texture '{}' width too large dataloss imminent.", filename);
        }
        if image.height() > i32::MAX as u32 {
            panic!("Texture '{}' height too tall dataloss imminent.", filename);
        }

        let size = TextureSize::TwoD {
            width: image.width() as i32,
            height: image.height() as i32,
        };

        // TODO support more than 3 channels
        let format = match image.color() {
            image::ColorType::Rgb8 => ImageFormat::RGB,
            image::ColorType::Rgba8 => ImageFormat::RGBA,
            _ => panic!("Texture format not supported."),
        };

        let data = match image {
            DynamicImage::ImageRgb8(texture_image) => texture_image.into_raw(),
            DynamicImage::ImageRgba8(texture_image) => texture_image.into_raw(),
            _ => panic!("Image format not supported"),
        };

        Self {
            filename: filename.to_string(),
            data,
            format,
            size,
        }
    }
}
