use crate::types::{
    Filtering, Font, Glyph, Image, ImageFormat, ImageSize, Texture, TextureKind, Wrapping,
};
use freetype::Library;
use image::{ImageBuffer, Rgba};
use std::{collections::HashMap, path::PathBuf};

impl Font {
    pub fn from_foreign(path: PathBuf, extension: &str) -> Font {
        let library = match extension {
            "ttf" => Library::init().expect("Could not init freetype library"),
            _ => panic!("Unsupported font format."),
        };

        let face = library.new_face(path, 0).expect("Could not create face.");
        let width: u32 = 30;
        let height: u32 = 30;
        let line_height = height / 2;

        let total_glyphs = face.num_glyphs() as u32;
        face.set_pixel_sizes(width, height)
            .expect("Could not set pixel sizes.");

        let mut sprite_sheet =
            ImageBuffer::from_pixel(width * total_glyphs, height, Rgba([0, 0, 0, 0]));

        let mut sprite_x = 0;
        // TODO might be optimizable using Y as well
        let sprite_y = 0;

        let mut glyphs: HashMap<char, Glyph> = HashMap::new();
        for glyph_index in 0..total_glyphs {
            face.load_glyph(glyph_index, freetype::face::LoadFlag::RENDER)
                .expect("Could not load glyph.");

            let character = std::char::from_u32(glyph_index)
                .expect("Could not convert glyph index to character.");

            let glyphslot = face.glyph();
            let bitmap = glyphslot.bitmap();

            for y in 0..bitmap.rows() {
                for x in 0..bitmap.width() {
                    let pixel = bitmap.buffer()[(y * bitmap.width() + x) as usize];
                    sprite_sheet.put_pixel(
                        (sprite_x + x) as u32,
                        (sprite_y + y) as u32,
                        Rgba([255, 255, 255, pixel]),
                    );
                }
            }

            sprite_x += width as i32;

            let glyph = Glyph {
                width: bitmap.width(),
                height: bitmap.rows(),
                sprite_x,
                sprite_y,
                bearing_x: glyphslot.bitmap_left(),
                bearing_y: glyphslot.bitmap_top(),
                advance_x: glyphslot.advance().x,
                advance_y: glyphslot.advance().y,
            };

            if glyphs.get(&character).is_none() {
                glyphs.insert(character, glyph);
            } else {
                panic!("Duplicate character in font.");
            }
        }

        let image = Image {
            data: sprite_sheet.into_raw(),
            format: ImageFormat::RGBA,
            size: ImageSize::I2D {
                x: width as i32,
                y: height as i32,
            },
        };

        let kind = TextureKind::Diffuse;
        let s_wrapping = Wrapping::Repeat;
        let t_wrapping = Wrapping::Repeat;
        let min_filtering = Filtering::Linear;
        let mag_filtering = Filtering::Linear;
        let mipmapping = false;

        let mut sprite = Texture {
            gl_id: 0,
            image,
            kind,
            s_wrapping,
            t_wrapping,
            min_filtering,
            mag_filtering,
            mipmapping,
        };
        sprite.gl_register();

        Font {
            sprite,
            glyphs,
            width,
            height,
            line_height,
        }
    }
}
