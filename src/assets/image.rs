use crate::types::{Image, ImageFormat, ImageSize};
use image::DynamicImage;
use std::path::PathBuf;

impl Image {
    pub fn from_file(path: PathBuf, extension: &str) -> Self {
        let image = match extension.to_lowercase().as_str() {
            "jpg" | "png" => image::open(path).expect("Failed to load image."),
            _ => panic!("Unsupported image extension: {}", extension),
        };

        // TODO support more than 3 channels
        const MAX_TEXTURE_SIZE: u32 = std::i32::MAX as u32;
        if image.width() > MAX_TEXTURE_SIZE || image.height() > MAX_TEXTURE_SIZE {
            panic!(
                "Texture size exceeds maximum allowed size of {} pixels",
                MAX_TEXTURE_SIZE
            );
        }

        let size = ImageSize::I2D {
            x: image.width() as i32,
            y: image.height() as i32,
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

        Self { data, format, size }
    }

    pub fn from_data(data: &[u8], mime_type: &str) -> Self {
        let image = match mime_type {
            "image/png" => image::load_from_memory_with_format(data, ::image::ImageFormat::Png),
            "image/jpeg" => image::load_from_memory_with_format(data, ::image::ImageFormat::Jpeg),
            _ => panic!("Unsupported image mime type: {}", mime_type),
        }
        .expect("Failed to load image.");

        // TODO support more than 3 channels
        const MAX_TEXTURE_SIZE: u32 = std::i32::MAX as u32;
        if image.width() > MAX_TEXTURE_SIZE || image.height() > MAX_TEXTURE_SIZE {
            panic!(
                "Texture size exceeds maximum allowed size of {} pixels",
                MAX_TEXTURE_SIZE
            );
        }

        let size = ImageSize::I2D {
            x: image.width() as i32,
            y: image.height() as i32,
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

        Self { data, format, size }
    }
}
