use freetype::Library;
use image::DynamicImage;

use crate::types::{
    Asset, AssetFont, AssetImage2D, AssetManager, AssetTexture, Character, Filtering, ImageFormat,
    ImageKind, Name, Path, TextureSize, Wrapping,
};
use std::collections::HashMap;

impl AssetManager {
    pub fn new() -> Self {
        Self {
            image_assets: HashMap::new(),
            font_assets: HashMap::new(),
            texture_assets: HashMap::new(),
        }
    }

    // Image

    pub fn new_image_asset(&mut self, path: &Path) -> AssetImage2D {
        let (name, ext) = parse_path(path);

        let image = match ext.as_str() {
            "jpg" | "png" => image::open(path).expect("Failed to load image."),
            _ => panic!("Unsupported asset type"),
        };

        let asset = Asset {
            name: name.to_string(),
            path: path.to_string(),
        };

        AssetImage2D { asset, image }
    }

    // Font

    pub fn new_font_asset(&mut self, path: &Path, size: u32) -> AssetFont {
        let (name, ext) = parse_path(path);

        let library = Library::init().expect("Could not init freetype library");
        let face = match ext.as_str() {
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

        let asset = Asset {
            name: name.to_string(),
            path: path.to_string(),
        };

        AssetFont { asset, size, chars }
    }

    // Texture

    pub fn new_texture_asset(
        &mut self,
        path: &Path,
        kind: ImageKind,
        s_wrapping: Wrapping,
        t_wrapping: Wrapping,
        min_filtering: Filtering,
        mag_filtering: Filtering,
        mipmapping: bool,
    ) -> AssetTexture {
        let (name, ext) = parse_path(path);

        let image = match ext.as_str() {
            "jpg" | "png" => image::open(path).expect("Failed to load image.").flipv(),
            _ => panic!("Unsupported asset type"),
        };

        if image.width() > i32::MAX as u32 {
            panic!("Texture '{}' width too large dataloss not tolerated.", path);
        }
        if image.height() > i32::MAX as u32 {
            panic!("Texture '{}' height too tall dataloss not tolerated.", path);
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

        let asset = Asset {
            name: name.to_string(),
            path: path.to_string(),
        };

        AssetTexture::create_texture(
            asset,
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

fn parse_path(path: &str) -> (Name, Path) {
    let ext = path.split('.').last().expect("No file extension.");
    let name = path.split('.').next().expect("No file name.");

    (name.to_owned(), ext.to_owned())
}
