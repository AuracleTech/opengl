use crate::types::{Ascii, Character};
use std::collections::HashMap;

impl Ascii {
    pub fn new(font_folder: String, font_name: String, font_size: u32) -> Self {
        let full_path = format!("{}{}", font_folder, font_name);

        let mut ascii: HashMap<char, Character> = HashMap::new();
        let library = freetype::Library::init().expect("Could not init freetype library");
        let face = library.new_face(full_path, 0).expect("Could not open font");
        face.set_pixel_sizes(0, font_size)
            .expect("Could not set pixel size");

        for c in 0..128 {
            ascii.insert(c as u8 as char, Character::from_face(&face, c));
        }

        Self {
            name: font_name,
            size: font_size,
            chars: ascii,
        }
    }
}
