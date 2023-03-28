use gl::types::{GLenum, GLsizei};
use image::DynamicImage;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// TODO remove debug everywhere
#[non_exhaustive]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Image {
    pub data: Vec<u8>,
    pub gl_format: GLenum,
    pub size: ImageSize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ImageSize {
    I2D { x: GLsizei, y: GLsizei },
    I3D { x: GLsizei, y: GLsizei, z: GLsizei },
}

impl Image {
    pub fn from_file(path: PathBuf, extension: &str) -> Self {
        let image = match extension.to_lowercase().as_str() {
            "jpg" | "png" => image::open(path).expect("Failed to load image."),
            _ => panic!("Unsupported image extension: {}", extension),
        };

        Self::from_dynamic_image(image)
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
        let gl_format = match dynamic_image.color() {
            image::ColorType::Rgb8 => gl::RGB,
            image::ColorType::Rgba8 => gl::RGBA,
            _ => panic!("Texture format not supported."),
        };

        let data = match dynamic_image {
            DynamicImage::ImageRgb8(texture_image) => texture_image.into_raw(),
            DynamicImage::ImageRgba8(texture_image) => texture_image.into_raw(),
            _ => panic!("Image format not supported"),
        };

        Self {
            data,
            gl_format,
            size,
        }
    }

    pub fn to_glfw_pixelimage(&self) -> glfw::PixelImage {
        let (width, height) = match self.size {
            ImageSize::I2D { x, y } => (x, y),
            _ => panic!("Icon size is not 2D."),
        };

        let mut icon_pixels: Vec<u32> = vec![];
        for chunk in self.data.chunks_exact(4) {
            let u32_value = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
            icon_pixels.push(u32_value);
        }

        glfw::PixelImage {
            width: width as u32,
            height: height as u32,
            pixels: icon_pixels,
        }
    }
}
