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
    // TODO configurable tex options
    pub fn new(image: Image) -> Self {
        let mipmapping = true;
        let kind = TextureKind::Diffuse;
        let gl_s_wrapping = gl::REPEAT;
        let gl_t_wrapping = gl::REPEAT;
        let gl_min_filtering = gl::LINEAR_MIPMAP_LINEAR;
        let gl_mag_filtering = gl::LINEAR;
        // TODO 3D texture
        let target = gl::TEXTURE_2D;

        let alignment = match image.gl_format {
            gl::RGB => 1,
            gl::RGBA => 4,
            _ => panic!("Texture format not supported yet."),
        };
        unsafe {
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, alignment);
        }

        let mut gl_id = 0;
        unsafe {
            gl::GenTextures(1, &mut gl_id);
            gl::BindTexture(target, gl_id);
        }
        match image.size {
            ImageSize::I2D { x, y } => {
                unsafe {
                    // texture data
                    gl::TexImage2D(
                        target,
                        0,
                        image.gl_format as GLint,
                        x,
                        y,
                        0,
                        image.gl_format,
                        gl::UNSIGNED_BYTE,
                        image.data.as_ptr() as *const GLvoid,
                    );
                }
            }
            // TODO 3D texture
            _ => panic!("Texture size not supported yet."),
        }
        unsafe {
            // wrapping
            gl::TexParameteri(target, gl::TEXTURE_WRAP_S, gl_s_wrapping as GLint);
            gl::TexParameteri(target, gl::TEXTURE_WRAP_T, gl_t_wrapping as GLint);
            // filtering
            gl::TexParameteri(target, gl::TEXTURE_MIN_FILTER, gl_min_filtering as GLint);
            gl::TexParameteri(target, gl::TEXTURE_MAG_FILTER, gl_mag_filtering as GLint);
        }
        // mipmapping
        if mipmapping {
            unsafe {
                gl::GenerateMipmap(target);
            }
        }

        Texture::gl_unbind();

        Self {
            gl_id,
            image,
            kind,
            gl_s_wrapping,
            gl_t_wrapping,
            gl_min_filtering,
            gl_mag_filtering,
            mipmapping,
        }
    }

    pub fn gl_unbind() {
        unsafe {
            // TODO add texture type (2D, 3D ... ) in Texture struct
            // TODO add texture unit in Texture struct
            // TODO move function inside Texture struct
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    // TODO deal with max amount of texture units
    pub fn gl_bind(&self, bind_position: GLuint) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.gl_id);
            gl::ActiveTexture(gl::TEXTURE0 + bind_position);
        }
    }

    pub fn gl_set_param_i(&self, param: u32, value: i32) {
        unsafe {
            // TODO add texture type (2D, 3D ... ) in Texture struct
            gl::TexParameteri(gl::TEXTURE_2D, param, value);
        }
    }
}
