use crate::types::{Character, Font};
use std::collections::HashMap;

impl Font {
    pub fn new(font_folder: String, file_name: String, size: u32) -> Self {
        let full_path = format!("{}{}", font_folder, file_name);

        let mut chars: HashMap<char, Character> = HashMap::new();
        let library = freetype::Library::init().expect("Could not init freetype library");
        let face = library.new_face(full_path, 0).expect("Could not open font");
        face.set_pixel_sizes(0, size)
            .expect("Could not set pixel size");

        for c in 0..128 {
            chars.insert(c as u8 as char, Character::from_face(&face, c));
        }

        Self { size, chars }
    }
}
