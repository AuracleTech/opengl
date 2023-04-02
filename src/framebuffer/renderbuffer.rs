use gl::types::{GLenum, GLsizei, GLuint};

pub struct Renderbuffer {
    pub(crate) gl_id: GLuint,
    pub(crate) gl_target: GLenum,
    pub(crate) gl_format: GLenum,
    pub(crate) width: GLsizei,
    pub(crate) height: GLsizei,
}

impl Renderbuffer {
    pub fn new(width: GLsizei, height: GLsizei) -> Self {
        // TODO gl_target
        let gl_target = gl::RENDERBUFFER;
        // TODO gl_format
        let gl_format = gl::DEPTH24_STENCIL8;

        let mut gl_id = 0;
        unsafe {
            gl::GenRenderbuffers(1, &mut gl_id);
            gl::BindRenderbuffer(gl_target, gl_id);
            gl::RenderbufferStorage(gl_target, gl_format, width, height);
            gl::BindRenderbuffer(gl_target, 0);
        }

        Self {
            gl_id,
            gl_target,
            gl_format,
            width,
            height,
        }
    }

    pub fn gl_bind(&self) {
        unsafe {
            gl::BindRenderbuffer(self.gl_target, self.gl_id);
        }
    }

    pub fn gl_unbind() {
        unsafe {
            gl::BindRenderbuffer(gl::RENDERBUFFER, 0);
        }
    }
}
