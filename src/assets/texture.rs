use gl::types::{GLenum, GLint, GLuint, GLvoid};
use serde::{Deserialize, Serialize};

use super::image::{Image, ImageSize}; // FIX TODO REPLACE IMAGE FORMAT BY GL_FORMAT OR SOMETHING

// TODO remove debug everywhere
#[non_exhaustive]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Texture {
    gl_id: GLuint,
    image: Image,
    pub(crate) kind: TextureKind,
    pub(crate) gl_s_wrapping: GLenum,
    pub(crate) gl_t_wrapping: GLenum,
    pub(crate) gl_min_filtering: GLenum,
    pub(crate) gl_mag_filtering: GLenum,
    mipmapping: bool,
}

// TODO remove debug everywhere
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TextureKind {
    Diffuse,
    Specular,
    Normal,
    Height,
    Emissive,
    Ambient,
}

impl Texture {
    pub fn from_image(image: Image) -> Self {
        let mut texture = Self {
            gl_id: 0,
            image,
            kind: TextureKind::Diffuse,
            gl_s_wrapping: gl::REPEAT,
            gl_t_wrapping: gl::REPEAT,
            gl_min_filtering: gl::LINEAR_MIPMAP_LINEAR,
            gl_mag_filtering: gl::LINEAR,
            mipmapping: true,
        };
        texture.gl_register();
        texture
    }

    pub fn gl_register(&mut self) {
        let internal_format = match self.image.gl_format {
            gl::RED => gl::RED,
            gl::RG => gl::RG,
            gl::RGB => gl::RGB,
            gl::RGBA => gl::RGBA,
            _ => panic!("Texture format not supported yet."),
        };
        let alignment = match internal_format {
            gl::RED => 1,
            gl::RG => 2,
            gl::RGB => 3,
            gl::RGBA => 4,
            _ => panic!("Texture format not supported yet."),
        };
        unsafe {
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, alignment);
        }

        // TODO 3D texture
        let target = gl::TEXTURE_2D;

        let mut id = 0;
        unsafe {
            // generate texture id
            gl::GenTextures(1, &mut id);
            gl::BindTexture(target, id);
        }
        match self.image.size {
            ImageSize::I2D { x, y } => {
                unsafe {
                    // texture data
                    gl::TexImage2D(
                        target,
                        0,
                        internal_format as GLint,
                        x,
                        y,
                        0,
                        internal_format,
                        gl::UNSIGNED_BYTE,
                        self.image.data.as_ptr() as *const GLvoid,
                    );
                }
            }
            // TODO 3D texture
            _ => panic!("Texture size not supported yet."),
        }
        unsafe {
            // wrapping
            gl::TexParameteri(target, gl::TEXTURE_WRAP_S, self.gl_s_wrapping as GLint);
            gl::TexParameteri(target, gl::TEXTURE_WRAP_T, self.gl_t_wrapping as GLint);
            // filtering
            gl::TexParameteri(
                target,
                gl::TEXTURE_MIN_FILTER,
                self.gl_min_filtering as GLint,
            );
            gl::TexParameteri(
                target,
                gl::TEXTURE_MAG_FILTER,
                self.gl_mag_filtering as GLint,
            );
        }
        // mipmapping
        if self.mipmapping {
            unsafe {
                gl::GenerateMipmap(target);
            }
        }

        self.gl_id = id;
    }

    // TODO deal with max amount of texture units
    pub fn gl_bind(&self, bind_position: GLuint) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.gl_id);
            gl::ActiveTexture(gl::TEXTURE0 + bind_position);
        }
    }

    pub fn gl_unbind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    pub fn gl_set_param_i(&self, param: u32, value: i32) {
        unsafe {
            // TODO add texture type (2D, 3D ... ) in Texture struct
            gl::TexParameteri(gl::TEXTURE_2D, param, value);
        }
    }
}
