pub(crate) mod renderbuffer;
pub(crate) mod texturebuffer;
use self::{renderbuffer::Renderbuffer, texturebuffer::TextureFramebuffer};
use gl::types::GLuint;

pub struct Framebuffer {
    pub(crate) gl_id: GLuint,
    pub gl_texturebuffer_id: GLuint,
    pub gl_renderbuffer_id: GLuint,
}

impl Framebuffer {
    pub fn new() -> Self {
        let mut framebuffer = Self {
            gl_id: 0,
            gl_texturebuffer_id: 0,
            gl_renderbuffer_id: 0,
        };

        unsafe {
            gl::GenFramebuffers(1, &mut framebuffer.gl_id);
            gl::BindFramebuffer(gl::FRAMEBUFFER, framebuffer.gl_id);
        }

        framebuffer
    }

    pub fn gl_attach_texture(&mut self, texture: TextureFramebuffer) {
        // TODO attachement
        let gl_attachement = gl::COLOR_ATTACHMENT0;
        unsafe {
            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl_attachement,
                texture.gl_target,
                texture.gl_id,
                0, // TODO mipmap level
            );
        }

        #[cfg(feature = "pillow")]
        self.integrity_check();

        self.gl_texturebuffer_id = texture.gl_id;
    }

    pub fn gl_attach_renderbuffer(&mut self, renderbuffer: Renderbuffer) {
        // TODO attachement
        let gl_attachement = gl::DEPTH_STENCIL_ATTACHMENT;
        unsafe {
            gl::FramebufferRenderbuffer(
                gl::FRAMEBUFFER,
                gl_attachement,
                renderbuffer.gl_target,
                renderbuffer.gl_id,
            );
        }

        #[cfg(feature = "pillow")]
        self.integrity_check();

        self.gl_renderbuffer_id = renderbuffer.gl_id;
    }

    pub fn gl_bind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.gl_id);
        }
    }

    pub fn gl_unbind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }

    #[inline]
    #[cfg(feature = "pillow")]
    pub fn integrity_check(&self) {
        let status = unsafe { gl::CheckFramebufferStatus(gl::FRAMEBUFFER) };
        match status {
            gl::FRAMEBUFFER_COMPLETE => (),
            gl::FRAMEBUFFER_INCOMPLETE_ATTACHMENT => {
                panic!("Incomplete framebuffer: Attachment is NOT complete.")
            }
            gl::FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT => {
                panic!("Incomplete framebuffer: No image is attached to FBO.")
            }
            gl::FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER => {
                panic!("Incomplete framebuffer: Draw buffer.")
            }
            gl::FRAMEBUFFER_INCOMPLETE_READ_BUFFER => {
                panic!("Incomplete framebuffer: Read buffer.")
            }
            gl::FRAMEBUFFER_UNSUPPORTED => {
                panic!("Incomplete framebuffer: Unsupported by FBO implementation.")
            }
            gl::FRAMEBUFFER_INCOMPLETE_MULTISAMPLE => {
                panic!("Incomplete framebuffer: Multisample.")
            }
            gl::FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS => {
                panic!("Incomplete framebuffer: Layer targets.")
            }
            _ => panic!("Incomplete framebuffer: Unknown error."),
        }
    }
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteFramebuffers(1, &self.gl_id);
        }
    }
}
