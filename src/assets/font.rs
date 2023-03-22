use crate::types::{
    Filtering, Font, Glyph, Image, ImageFormat, ImageSize, Texture, TextureKind, Wrapping,
};
use freetype::Library;
use image::{ImageBuffer, Rgba};
use std::{collections::HashMap, path::PathBuf};

impl Font {
    pub fn from_ttf(path: PathBuf) -> Font {
        let library = Library::init().expect("Could not init freetype library");
        let face = library.new_face(path, 0).expect("Could not create face.");
        let font_width: u32 = 30;
        let font_height: u32 = 30;
        let line_height = font_height / 2;
        let total_glyphs = face.num_glyphs() as u32;
        face.set_pixel_sizes(font_width, font_height)
            .expect("Could not set pixel sizes.");

        let mut omega_glyph_width = 0;
        let mut highest_glyph_height = 0;
        let mut glyphs: HashMap<char, Glyph> = HashMap::new();
        for glyph_index in 0..total_glyphs {
            face.load_glyph(glyph_index, freetype::face::LoadFlag::RENDER)
                .expect("Could not load glyph.");

            let character = std::char::from_u32(glyph_index)
                .expect("Could not convert glyph index to character.");

            let glyphslot = face.glyph();
            let bitmap = glyphslot.bitmap();

            let glyph = Glyph {
                width: bitmap.width(),
                height: bitmap.rows(),
                sprite_x: 0,
                sprite_y: 0,
                bearing_x: glyphslot.bitmap_left(),
                bearing_y: glyphslot.bitmap_top(),
                advance_x: glyphslot.advance().x as i64,
                advance_y: glyphslot.advance().y as i64,
            };

            omega_glyph_width += glyph.width as u32;
            highest_glyph_height = highest_glyph_height.max(glyph.height as u32);

            if glyphs.get(&character).is_none() {
                glyphs.insert(character, glyph);
            } else {
                panic!("Duplicate character in font.");
            }
        }

        let mut sprite_x = 0;
        // TODO might be optimizable using Y as well
        let sprite_y = 0;
        let mut sprite_sheet = ImageBuffer::from_pixel(
            omega_glyph_width,
            highest_glyph_height,
            Rgba([255, 255, 255, 0]),
        );
        let sprite_sheet_width = sprite_sheet.width();
        let sprite_sheet_height = sprite_sheet.height();

        for glyph_index in 0..total_glyphs {
            face.load_glyph(glyph_index, freetype::face::LoadFlag::RENDER)
                .expect("Could not load glyph.");

            let character = std::char::from_u32(glyph_index)
                .expect("Could not convert glyph index to character.");

            let glyphslot = face.glyph();
            let bitmap = glyphslot.bitmap();
            let bitmap_width = bitmap.width() as u32;
            let bitmap_height = bitmap.rows() as u32;

            let glyph = glyphs.get_mut(&character).unwrap();
            glyph.sprite_x = sprite_x;
            glyph.sprite_y = sprite_y;

            for x in 0..bitmap_width {
                for y in 0..bitmap_height {
                    let pixel = bitmap.buffer()[(y * bitmap_width + x) as usize];
                    sprite_sheet.put_pixel(
                        sprite_x + x,
                        sprite_y + y,
                        Rgba([255, 255, 255, pixel]),
                    );
                }
            }
            sprite_x += bitmap_width;
        }

        let image = Image {
            data: sprite_sheet.into_raw(),
            format: ImageFormat::RGBA,
            size: ImageSize::I2D {
                x: sprite_sheet_width as i32,
                y: sprite_sheet_height as i32,
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
            width: font_width,
            height: font_height,
            line_height,
        }
    }
}
