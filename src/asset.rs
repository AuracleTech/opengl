use freetype::Library;
use image::DynamicImage;

use crate::types::{
    AssetFont, AssetImage2D, AssetManager, AssetTexture, Character, Filtering, ImageFormat,
    ImageKind, TextureSize, Wrapping,
};
use std::{collections::HashMap, path::PathBuf};

impl AssetManager {
    pub fn new(assets_path: PathBuf) -> Self {
        Self {
            image_assets: HashMap::new(),
            image_assets_path: assets_path.join("images"),
            font_assets: HashMap::new(),
            font_assets_path: assets_path.join("fonts"),
            texture_assets: HashMap::new(),
            texture_assets_path: assets_path.join("textures"),
            assets_path,
        }
    }

    // Image

    pub fn new_image_asset(&mut self, filename: &str) -> AssetImage2D {
        let path = self.image_assets_path.join(filename);
        let ext = path
            .extension()
            .expect("Failed to get file extension.")
            .to_str()
            .expect("Failed to convert file extension to str.");

        let image = match ext {
            "jpg" | "png" => image::open(path).expect("Failed to load image."),
            _ => panic!("Unsupported asset type"),
        };

        AssetImage2D {
            filename: filename.to_string(),
            image,
        }
    }

    // Font

    pub fn new_font_asset(&mut self, filename: &str, size: u32) -> AssetFont {
        let path = self.font_assets_path.join(filename);
        let ext = path
            .extension()
            .expect("Failed to get file extension.")
            .to_str()
            .expect("Failed to convert file extension to str.");

        let library = Library::init().expect("Could not init freetype library");
        let face = match ext {
            "ttf" => library.new_face(path, 0).expect("Could not open font"),
            _ => panic!("Unsupported asset type"),
        };
        let mut chars: HashMap<char, Character> = HashMap::new();

        // TODO make size configurable by width and height
        face.set_pixel_sizes(0, size)
            .expect("Could not set pixel size");

        // TODO make this configurable
        for c in 0..128 {
            chars.insert(c as u8 as char, Character::from_face(&face, c));
        }

        AssetFont {
            filename: filename.to_string(),
            size,
            chars,
        }
    }

    // Texture

    pub fn new_texture_asset(
        &mut self,
        filename: &str,
        kind: ImageKind,
        s_wrapping: Wrapping,
        t_wrapping: Wrapping,
        min_filtering: Filtering,
        mag_filtering: Filtering,
        mipmapping: bool,
    ) -> AssetTexture {
        let path = self.texture_assets_path.join(filename);
        let ext = path
            .extension()
            .expect("Failed to get file extension.")
            .to_str()
            .expect("Failed to convert file extension to str.");

        let image = match ext {
            "jpg" | "png" => image::open(path).expect("Failed to load image.").flipv(),
            _ => panic!("Unsupported asset type"),
        };

        if image.width() > i32::MAX as u32 {
            panic!(
                "Texture '{}' width too large dataloss not tolerated.",
                filename
            );
        }
        if image.height() > i32::MAX as u32 {
            panic!(
                "Texture '{}' height too tall dataloss not tolerated.",
                filename
            );
        }

        let size = TextureSize::TwoD {
            width: image.width() as i32,
            height: image.height() as i32,
        };

        // TODO support more than 3 channels
        let format = match image.color() {
            image::ColorType::Rgb8 => ImageFormat::RGB,
            _ => panic!("Texture format not supported."),
        };

        let data = match image {
            DynamicImage::ImageRgb8(texture_image) => texture_image.into_raw(),
            _ => panic!("Image format not supported"),
        };

        AssetTexture::create_texture(
            filename,
            data,
            kind,
            size,
            format,
            s_wrapping,
            t_wrapping,
            min_filtering,
            mag_filtering,
            mipmapping,
        )
    }
}
