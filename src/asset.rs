use freetype::Library;

use crate::types::{Asset, AssetImage2D, AssetManager, Character, Font, Name, Path};
use std::collections::HashMap;

impl AssetManager {
    pub fn new() -> Self {
        Self {
            image_assets: HashMap::new(),
            font_assets: HashMap::new(),
        }
    }

    // Image

    pub fn new_image_asset(&mut self, path: &Path) -> AssetImage2D {
        let (name, ext) = parse_path(path);

        let image = match ext.as_str() {
            "jpg" | "png" => image::open(path).expect("Failed to load image.").flipv(),
            _ => panic!("Unsupported asset type"),
        };

        let asset = Asset {
            name: name.to_string(),
            path: path.to_string(),
        };

        AssetImage2D { asset, image }
    }

    pub fn stock_image_asset(&mut self, path: &Path, image_asset: AssetImage2D) {
        self.image_assets.insert(path.to_owned(), image_asset);
    }

    // Font

    pub fn new_font_asset(&mut self, path: &Path, size: u32) -> Font {
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

        Font { asset, size, chars }
    }

    pub fn stock_font_asset(&mut self, path: &Path, font_asset: Font) {
        self.font_assets.insert(path.to_owned(), font_asset);
    }
}

fn parse_path(path: &str) -> (Name, Path) {
    let ext = path.split('.').last().expect("No file extension.");
    let name = path.split('.').next().expect("No file name.");

    (name.to_owned(), ext.to_owned())
}
