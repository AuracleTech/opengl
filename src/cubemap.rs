use gl::types::{GLenum, GLint, GLuint, GLvoid};

use crate::assets::image::Image;

pub struct Cubemap {
    gl_id: GLuint,
    images_pos_x: Image,
    images_neg_x: Image,
    images_pos_y: Image,
    images_neg_y: Image,
    images_pos_z: Image,
    images_neg_z: Image,
    pub(crate) gl_s_wrapping: GLenum,
    pub(crate) gl_t_wrapping: GLenum,
    pub(crate) gl_r_wrapping: GLenum,
    pub(crate) gl_min_filtering: GLenum,
    pub(crate) gl_mag_filtering: GLenum,
    mipmapping: bool,
}

impl Cubemap {
    pub fn from_images(
        images_pos_x: Image,
        images_neg_x: Image,
        images_pos_y: Image,
        images_neg_y: Image,
        images_pos_z: Image,
        images_neg_z: Image,
    ) -> Self {
        let mut texture = Self {
            gl_id: 0,
            images_pos_x,
            images_neg_x,
            images_pos_y,
            images_neg_y,
            images_pos_z,
            images_neg_z,
            gl_s_wrapping: gl::CLAMP_TO_EDGE,
            gl_t_wrapping: gl::CLAMP_TO_EDGE,
            gl_r_wrapping: gl::CLAMP_TO_EDGE,
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
            gl::BindTexture(gl::TEXTURE_CUBE_MAP, 0);
        }
    }

    // TODO deal with max amount of texture units
    pub fn gl_bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_CUBE_MAP, self.gl_id);
        }
    }

    pub fn gl_set_param_i(&self, param: u32, value: i32) {
        unsafe {
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, param, value);
        }
    }

    pub fn gl_register(&mut self) {
        let alignment = match self.images_pos_x.gl_format {
            gl::RGB => 1,
            gl::RGBA => 4,
            _ => panic!("Texture format not supported yet."),
        };
        let gl_width: GLint = self
            .images_pos_x
            .width
            .try_into()
            .expect("Texture Image width too big");
        let gl_height: GLint = self
            .images_pos_x
            .height
            .try_into()
            .expect("Texture Image height too big");
        unsafe {
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, alignment);

            // register texture
            gl::GenTextures(1, &mut self.gl_id);
            gl::BindTexture(gl::TEXTURE_CUBE_MAP, self.gl_id);

            // data X
            gl::TexImage2D(
                gl::TEXTURE_CUBE_MAP_POSITIVE_X,
                0,
                self.images_pos_x.gl_format as GLint,
                gl_width,
                gl_height,
                0,
                self.images_pos_x.gl_format,
                gl::UNSIGNED_BYTE,
                self.images_pos_x.data.as_ptr() as *const GLvoid,
            );
            gl::TexImage2D(
                gl::TEXTURE_CUBE_MAP_NEGATIVE_X,
                0,
                self.images_neg_x.gl_format as GLint,
                gl_width,
                gl_height,
                0,
                self.images_neg_x.gl_format,
                gl::UNSIGNED_BYTE,
                self.images_neg_x.data.as_ptr() as *const GLvoid,
            );

            // data Y
            gl::TexImage2D(
                gl::TEXTURE_CUBE_MAP_POSITIVE_Y,
                0,
                self.images_pos_y.gl_format as GLint,
                gl_width,
                gl_height,
                0,
                self.images_pos_y.gl_format,
                gl::UNSIGNED_BYTE,
                self.images_pos_y.data.as_ptr() as *const GLvoid,
            );
            gl::TexImage2D(
                gl::TEXTURE_CUBE_MAP_NEGATIVE_Y,
                0,
                self.images_neg_y.gl_format as GLint,
                gl_width,
                gl_height,
                0,
                self.images_neg_y.gl_format,
                gl::UNSIGNED_BYTE,
                self.images_neg_y.data.as_ptr() as *const GLvoid,
            );

            // data Z
            gl::TexImage2D(
                gl::TEXTURE_CUBE_MAP_POSITIVE_Z,
                0,
                self.images_pos_z.gl_format as GLint,
                gl_width,
                gl_height,
                0,
                self.images_pos_z.gl_format,
                gl::UNSIGNED_BYTE,
                self.images_pos_z.data.as_ptr() as *const GLvoid,
            );
            gl::TexImage2D(
                gl::TEXTURE_CUBE_MAP_NEGATIVE_Z,
                0,
                self.images_neg_z.gl_format as GLint,
                gl_width,
                gl_height,
                0,
                self.images_neg_z.gl_format,
                gl::UNSIGNED_BYTE,
                self.images_neg_z.data.as_ptr() as *const GLvoid,
            );

            // filtering
            gl::TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_MIN_FILTER,
                self.gl_min_filtering as GLint,
            );
            gl::TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_MAG_FILTER,
                self.gl_mag_filtering as GLint,
            );

            // wrapping
            gl::TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_WRAP_S,
                self.gl_s_wrapping as GLint,
            );
            gl::TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_WRAP_T,
                self.gl_t_wrapping as GLint,
            );
            gl::TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_WRAP_R,
                self.gl_r_wrapping as GLint,
            );

            // mipmapping
            if self.mipmapping {
                gl::GenerateMipmap(gl::TEXTURE_CUBE_MAP);
            }

            gl::BindTexture(gl::TEXTURE_CUBE_MAP, 0);
        }
    }
}

impl Drop for Cubemap {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.gl_id);
        }
    }
}
