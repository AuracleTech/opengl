use gl::types::{GLchar, GLenum, GLuint};
use std::ffi::CString;

pub struct Shader {
    pub id: GLuint,
}

impl Shader {
    pub fn new(source: &str, shader_type: GLenum) -> Shader {
        let id = unsafe { gl::CreateShader(shader_type) };

        // Verify ID was created
        if id <= 0 {
            panic!("Failed to create shader");
        }

        // Compile shader
        let source = CString::new(source.as_bytes()).expect("Failed to convert source to CString");
        unsafe {
            gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
            gl::CompileShader(id);
        }

        // Verify shader compiled
        let mut success = 0;
        unsafe {
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        }
        if success == 0 {
            let mut log_length = 0;
            unsafe {
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut log_length);
            }
            let mut log = Vec::with_capacity(log_length as usize);
            unsafe {
                gl::GetShaderInfoLog(
                    id,
                    log_length,
                    std::ptr::null_mut(),
                    log.as_mut_ptr() as *mut GLchar,
                );
                log.set_len(log_length as usize);
            }
            panic!(
                "Failed to compile vertex shader: {}",
                String::from_utf8(log).expect("Vertex shader log is not valid UTF-8.")
            );
        }

        Self { id }
    }
}
