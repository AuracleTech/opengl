use crate::{
    assets_path,
    types::{AssetFont, Character},
};
use freetype::Library;
use std::collections::HashMap;

const PATH: &str = "fonts";
const EXT: &str = "ttf";

impl AssetFont {
    pub fn load(name: String, size: u32) -> Self {
        let path = assets_path().join(PATH).join(&name).with_extension(EXT);

        let library = Library::init().expect("Could not init freetype library");
        let face = library.new_face(path, 0).expect("Could not open font");

        let mut chars: HashMap<char, Character> = HashMap::new();
        // TODO make size configurable by width and height
        face.set_pixel_sizes(0, size)
            .expect("Could not set pixel size");

        // TODO make this configurable
        for c in 0..128 {
            chars.insert(c as u8 as char, Character::new(&face, c, &name));
        }

        Self { name, size, chars }
    }
}
