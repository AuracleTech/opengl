use gl::types::{GLenum, GLsizei};
use image::DynamicImage;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// TODO remove debug everywhere
#[non_exhaustive]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Image {
    pub gl_format: GLenum,
    pub gl_target: GLenum,
    pub size: ImageSize,
    pub data: Vec<u8>,
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

    pub fn new(dynamic_image: DynamicImage) -> Self {
        let mut image = Self::default();

        // TODO support more than 3 channels
        const MAX_TEXTURE_SIZE: u32 = std::i32::MAX as u32;
        if dynamic_image.width() > MAX_TEXTURE_SIZE || dynamic_image.height() > MAX_TEXTURE_SIZE {
            panic!(
                "Texture size exceeds maximum allowed size of {} pixels",
                MAX_TEXTURE_SIZE
            );
        }

        // TODO support more than 3 channels
        image.gl_format = match dynamic_image.color() {
            image::ColorType::Rgb8 => gl::RGB,
            image::ColorType::Rgba8 => gl::RGBA,
            _ => panic!("Texture format not supported."),
        };

        image.size = ImageSize::I2D {
            x: dynamic_image.width() as i32,
            y: dynamic_image.height() as i32,
        };

        image.data = match dynamic_image {
            DynamicImage::ImageRgb8(texture_image) => texture_image.into_raw(),
            DynamicImage::ImageRgba8(texture_image) => texture_image.into_raw(),
            _ => panic!("Image format not supported"),
        };

        image
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

impl Default for Image {
    fn default() -> Self {
        Self {
            gl_format: gl::RGB,
            // TODO support more than 2D textures
            gl_target: gl::TEXTURE_2D,
            size: ImageSize::I2D { x: 0, y: 0 },
            data: vec![],
        }
    }
}
