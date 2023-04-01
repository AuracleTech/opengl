use gl::types::GLenum;
use image::DynamicImage;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// TODO remove debug everywhere
#[non_exhaustive]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Image {
    pub(crate) gl_format: GLenum,
    // TODO support more than 2D textures
    pub(crate) gl_target: GLenum,
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) data: Vec<u8>,
}

impl Image {
    pub fn new(dynamic_image: DynamicImage) -> Self {
        let image = Self {
            gl_format: match dynamic_image.color() {
                image::ColorType::Rgb8 => gl::RGB,
                image::ColorType::Rgba8 => gl::RGBA,
                _ => panic!("Texture format not supported."),
            },
            gl_target: gl::TEXTURE_2D,
            width: dynamic_image.width(),
            height: dynamic_image.height(),
            data: match dynamic_image {
                DynamicImage::ImageRgb8(texture_image) => texture_image.into_raw(),
                DynamicImage::ImageRgba8(texture_image) => texture_image.into_raw(),
                _ => panic!("Image format not supported"),
            },
        };
        #[cfg(debug_assertions)]
        integrity_check(&image);
        image
    }

    pub fn from_file(path: PathBuf, extension: &str) -> Self {
        let image = match extension.to_lowercase().as_str() {
            "jpg" | "png" => image::open(path).expect("Failed to load image."),
            _ => panic!("Unsupported image extension: {}", extension),
        };

        Self::new(image)
    }

    // TODO ability to fail gracefully just like me :D
    pub fn from_data(data: &[u8]) -> Self {
        let image = image::load_from_memory(data).expect("Failed to load image.");
        Self::new(image)
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

        Self::new(image_result)
    }

    pub fn to_glfw_pixelimage(&self) -> glfw::PixelImage {
        let mut icon_pixels: Vec<u32> = vec![];
        for chunk in self.data.chunks_exact(4) {
            let u32_value = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
            icon_pixels.push(u32_value);
        }

        glfw::PixelImage {
            width: self.width,
            height: self.height,
            pixels: icon_pixels,
        }
    }
}

fn integrity_check(image: &Image) {
    let expected_size = image.width
        * image.height
        * match image.gl_format {
            gl::RGB => 3,
            gl::RGBA => 4,
            _ => panic!("Texture format not supported yet."),
        };

    if image.data.len() != expected_size as usize {
        panic!(
            "Image data size does not match expected size. Expected: {}, Actual: {}",
            expected_size,
            image.data.len()
        );
    }

    if image.width <= 0 || image.height <= 0 {
        panic!(
            "Image dimensions are invalid. Width: {}, Height: {}",
            image.width, image.height
        );
    }
}
