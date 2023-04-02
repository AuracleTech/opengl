use gl::types::{GLenum, GLint, GLsizei, GLuint};

pub struct TextureFramebuffer {
    pub gl_id: GLuint, // FIX set to pub(crate) max
    pub(crate) gl_target: GLenum,
    pub(crate) gl_format: GLenum,
    pub(crate) gl_type: GLenum,
    pub(crate) width: GLsizei,
    pub(crate) height: GLsizei,
}

impl TextureFramebuffer {
    pub fn new(width: GLsizei, height: GLsizei) -> Self {
        // TODO gl_target
        let gl_target = gl::TEXTURE_2D;
        // TODO gl_format
        let gl_format = gl::RGBA;
        // TODO gl_type
        let gl_type = gl::UNSIGNED_BYTE;

        let mut gl_id = 0;
        unsafe {
            gl::GenTextures(1, &mut gl_id);
            gl::BindTexture(gl_target, gl_id);
            gl::TexImage2D(
                gl_target,
                0, // TODO mipmap level
                gl_format as GLint,
                width,
                height,
                0,
                gl_format as GLenum,
                gl_type,
                std::ptr::null(),
            );
            gl::TexParameteri(gl_target, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
            gl::TexParameteri(gl_target, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
            gl::BindTexture(gl_target, 0);
        }

        Self {
            gl_id,
            gl_target,
            gl_format,
            gl_type,
            width,
            height,
        }
    }
}
