use super::image::Image;
use gl::types::{GLenum, GLint, GLuint, GLvoid};
use serde::{Deserialize, Serialize};

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

    pub fn gl_unbind(&self) {
        unsafe {
            // TODO add texture type (2D, 3D ... ) in Texture struct
            // TODO add texture unit in Texture struct
            // TODO move function inside Texture struct
            gl::BindTexture(self.image.gl_target, 0);
        }
    }

    // TODO deal with max amount of texture units
    pub fn gl_bind(&self, bind_position: GLuint) {
        unsafe {
            gl::BindTexture(self.image.gl_target, self.gl_id);
            gl::ActiveTexture(gl::TEXTURE0 + bind_position);
        }
    }

    pub fn gl_set_param_i(&self, param: u32, value: i32) {
        unsafe {
            gl::TexParameteri(self.image.gl_target, param, value);
        }
    }

    pub fn gl_register(&mut self) {
        let alignment = match self.image.gl_format {
            gl::RGB => 1,
            gl::RGBA => 4,
            _ => panic!("Texture format not supported yet."),
        };
        let gl_width: GLint = self
            .image
            .width
            .try_into()
            .expect("Texture Image width too big");
        let gl_height: GLint = self
            .image
            .height
            .try_into()
            .expect("Texture Image height too big");
        unsafe {
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, alignment);

            // register texture
            gl::GenTextures(1, &mut self.gl_id);
            gl::BindTexture(self.image.gl_target, self.gl_id);

            // data
            gl::TexImage2D(
                self.image.gl_target,
                0,
                self.image.gl_format as GLint,
                gl_width,
                gl_height,
                0,
                self.image.gl_format,
                gl::UNSIGNED_BYTE,
                self.image.data.as_ptr() as *const GLvoid,
            );

            // wrapping
            gl::TexParameteri(
                self.image.gl_target,
                gl::TEXTURE_WRAP_S,
                self.gl_s_wrapping as GLint,
            );
            gl::TexParameteri(
                self.image.gl_target,
                gl::TEXTURE_WRAP_T,
                self.gl_t_wrapping as GLint,
            );
            // filtering
            gl::TexParameteri(
                self.image.gl_target,
                gl::TEXTURE_MIN_FILTER,
                self.gl_min_filtering as GLint,
            );
            gl::TexParameteri(
                self.image.gl_target,
                gl::TEXTURE_MAG_FILTER,
                self.gl_mag_filtering as GLint,
            );
        }
        // mipmapping
        if self.mipmapping {
            unsafe {
                gl::GenerateMipmap(self.image.gl_target);
            }
        }

        self.gl_unbind();
    }
}
