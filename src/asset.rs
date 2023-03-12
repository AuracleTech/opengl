use crate::types::{Asset, AssetImage2D, AssetManager};

impl AssetManager {
    pub fn new() -> Self {
        Self {
            image_assets: Vec::new(),
            text_assets: Vec::new(),
        }
    }

    pub fn new_image_asset(&mut self, path: &str) -> AssetImage2D {
        let ext = path.split('.').last().expect("No file extension.");
        let name = path.split('.').next().expect("No file name.");

        let image = match ext {
            "jpg" | "png" => image::open(path).expect("Failed to load image.").flipv(),
            _ => panic!("Unsupported asset type"),
        };

        let asset = Asset {
            name: name.to_string(),
            path: path.to_string(),
        };

        AssetImage2D { asset, image }
    }
}
