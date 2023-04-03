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
        let mut framebuffer = Self {
            gl_id: 0,
            // TODO configurable gl_target
            gl_target: gl::TEXTURE_2D,
            // TODO configurable gl_format
            gl_format: gl::RGBA,
            // TODO configurable gl_type
            gl_type: gl::UNSIGNED_BYTE,
            width,
            height,
        };

        unsafe {
            gl::GenTextures(1, &mut framebuffer.gl_id);
            gl::BindTexture(framebuffer.gl_target, framebuffer.gl_id);
            gl::TexImage2D(
                framebuffer.gl_target,
                0, // TODO mipmap level
                framebuffer.gl_format as GLint,
                width,
                height,
                0,
                framebuffer.gl_format as GLenum,
                framebuffer.gl_type,
                std::ptr::null(),
            );
            gl::TexParameteri(
                framebuffer.gl_target,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR as GLint,
            );
            gl::TexParameteri(
                framebuffer.gl_target,
                gl::TEXTURE_MAG_FILTER,
                gl::LINEAR as GLint,
            );
            gl::BindTexture(framebuffer.gl_target, 0);
        }

        framebuffer
    }
}
