use crate::types::{AssetTexture, Character};
use cgmath::Vector2;
use freetype::face::LoadFlag;
use freetype::Face;

impl Character {
    pub fn new(face: &Face, character: usize, font: &str) -> Self {
        face.load_char(character, LoadFlag::RENDER)
            .expect("Could not load character");
        let glyphslot = face.glyph();
        let bitmap = glyphslot.bitmap();
        let mut texture = AssetTexture::from_bitmap(&bitmap, character, &font);
        texture.gl_register();
        let size = Vector2::new(bitmap.width(), bitmap.rows());
        let bearing = Vector2::new(glyphslot.bitmap_left(), glyphslot.bitmap_top());
        let advance = glyphslot.advance().x as i64;
        texture.set_param_i(gl::TEXTURE_WRAP_S, gl::CLAMP_TO_BORDER as i32);
        texture.set_param_i(gl::TEXTURE_WRAP_T, gl::CLAMP_TO_BORDER as i32);
        texture.set_param_i(gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        texture.set_param_i(gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        Self {
            texture,
            size,
            bearing,
            advance,
        }
    }
}
