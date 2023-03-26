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

    // TODO ability to fail gracefully just like me :D
    pub fn from_data(data: &[u8]) -> Self {
        let image = image::load_from_memory(data).expect("Failed to load image.");
        Self::from_dynamic_image(image)
    }

    pub fn from_uri(uri: &str) -> Image {
        let url = url::Url::parse(uri).expect("Failed to parse image URI.");
        if url.scheme() != "file" {
            panic!("Unsupported image URI scheme: {}", url.scheme());
        }

        let path = url
            .to_file_path()
            .expect("Failed to convert image URI to file path.");
        let image_result = image::open(path).expect("Failed to load image.");

        Self::from_dynamic_image(image_result)
    }

    pub fn from_dynamic_image(dynamic_image: DynamicImage) -> Self {
        // TODO support more than 3 channels
        const MAX_TEXTURE_SIZE: u32 = std::i32::MAX as u32;
        if dynamic_image.width() > MAX_TEXTURE_SIZE || dynamic_image.height() > MAX_TEXTURE_SIZE {
            panic!(
                "Texture size exceeds maximum allowed size of {} pixels",
                MAX_TEXTURE_SIZE
            );
        }

        let size = ImageSize::I2D {
            x: dynamic_image.width() as i32,
            y: dynamic_image.height() as i32,
        };

        // TODO support more than 3 channels
        let format = match dynamic_image.color() {
            image::ColorType::Rgb8 => ImageFormat::RGB,
            image::ColorType::Rgba8 => ImageFormat::RGBA,
            _ => panic!("Texture format not supported."),
        };

        let data = match dynamic_image {
            DynamicImage::ImageRgb8(texture_image) => texture_image.into_raw(),
            DynamicImage::ImageRgba8(texture_image) => texture_image.into_raw(),
            _ => panic!("Image format not supported"),
        };

        Self { data, format, size }
    }
}
