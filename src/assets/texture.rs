use crate::types::{Filtering, Image, ImageFormat, ImageSize, Texture, TextureKind, Wrapping};
use gl::types::{GLenum, GLint, GLvoid};

impl Texture {
    pub fn gl_register(&mut self) {
        let internal_format = match self.image.format {
            ImageFormat::RGB => gl::RGB,
            ImageFormat::RGBA => gl::RGBA,
            ImageFormat::RG => panic!("RG format not supported yet."),
            ImageFormat::R => panic!("R format not supported yet."),
            ImageFormat::Unicolor => gl::RED,
        };
        let alignment = match internal_format {
            gl::RGB => 1,
            gl::RGBA => 4,
            gl::RED => 1,
            _ => panic!("Texture format not supported yet."),
        };
        unsafe {
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, alignment);
        }

        let gl_s_wrapping = gl_wrapping_from(&self.s_wrapping);
        let gl_t_wrapping = gl_wrapping_from(&self.t_wrapping);
        let gl_min_filtering = gl_filtering_from(&self.min_filtering);
        let gl_mag_filtering = gl_filtering_from(&self.mag_filtering);

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
            gl::TexParameteri(target, gl::TEXTURE_WRAP_S, gl_s_wrapping as i32);
            gl::TexParameteri(target, gl::TEXTURE_WRAP_T, gl_t_wrapping as i32);
            // filtering
            gl::TexParameteri(target, gl::TEXTURE_MIN_FILTER, gl_min_filtering as i32);
            gl::TexParameteri(target, gl::TEXTURE_MAG_FILTER, gl_mag_filtering as i32);
        }
        // mipmapping
        if self.mipmapping {
            unsafe {
                gl::GenerateMipmap(target);
            }
        }

        self.gl_id = id;
    }

    pub fn from_image(image: Image) -> Self {
        // TODO set configurable default values
        Texture {
            gl_id: 0,
            image,
            kind: TextureKind::Diffuse,
            s_wrapping: Wrapping::Repeat,
            t_wrapping: Wrapping::Repeat,
            min_filtering: Filtering::LinearMipmapLinear,
            mag_filtering: Filtering::Linear,
            mipmapping: true,
        }
    }

    // TODO deal with max amount of texture units
    pub fn gl_bind(&self, texture_unit: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + texture_unit);
            gl::BindTexture(gl::TEXTURE_2D, self.gl_id);
        }
    }

    pub fn gl_unbind() {
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

fn gl_filtering_from(filtering: &Filtering) -> GLenum {
    match filtering {
        Filtering::Nearest => gl::NEAREST,
        Filtering::Linear => gl::LINEAR,
        Filtering::NearestMipmapNearest => gl::NEAREST_MIPMAP_NEAREST,
        Filtering::NearestMipmapLinear => gl::NEAREST_MIPMAP_LINEAR,
        Filtering::LinearMipmapNearest => gl::LINEAR_MIPMAP_NEAREST,
        Filtering::LinearMipmapLinear => gl::LINEAR_MIPMAP_LINEAR,
    }
}

fn gl_wrapping_from(wrapping: &Wrapping) -> GLenum {
    match wrapping {
        Wrapping::Repeat => gl::REPEAT,
        Wrapping::MirroredRepeat => gl::MIRRORED_REPEAT,
        Wrapping::ClampToEdge => gl::CLAMP_TO_EDGE,
        Wrapping::ClampToBorder => gl::CLAMP_TO_BORDER,
    }
}
