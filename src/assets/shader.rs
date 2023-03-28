use gl::types::{GLchar, GLuint};
use std::{ffi::CString, fs::File, io::Read, path::PathBuf};

pub struct Shader {
    pub gl_id: GLuint,
}

impl Shader {
    pub fn from_foreign(path: PathBuf, extension: &str) -> Self {
        let shader_type = match extension.to_lowercase().as_str() {
            "vs" => gl::VERTEX_SHADER,
            "fs" => gl::FRAGMENT_SHADER,
            _ => panic!("Unsupported shader extension: {}", extension),
        };

        let mut source = String::new();
        let mut file = File::open(path).expect("Failed to open shader file.");
        file.read_to_string(&mut source)
            .expect("Failed to read shader file.");

        let gl_id = unsafe { gl::CreateShader(shader_type) };

        if gl_id <= 0 {
            panic!("The shader id is invalid.");
        }

        // Compile shader
        let source = CString::new(source).expect("Failed to convert source to CString.");
        unsafe {
            gl::ShaderSource(gl_id, 1, &source.as_ptr(), std::ptr::null());
            gl::CompileShader(gl_id);
        }

        verify_shader(gl_id, extension);

        Self { gl_id }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.gl_id);
        }
    }
}

fn verify_shader(gl_id: GLuint, extension: &str) {
    let mut success = 0;
    unsafe {
        gl::GetShaderiv(gl_id, gl::COMPILE_STATUS, &mut success);
    }
    if success == 0 {
        let mut log_length = 0;
        unsafe {
            gl::GetShaderiv(gl_id, gl::INFO_LOG_LENGTH, &mut log_length);
        }
        let mut log = Vec::with_capacity(log_length as usize);
        unsafe {
            gl::GetShaderInfoLog(
                gl_id,
                log_length,
                std::ptr::null_mut(),
                log.as_mut_ptr() as *mut GLchar,
            );
            log.set_len(log_length as usize);
        }
        let why = String::from_utf8(log).expect("Failed to convert log to String.");
        panic!("Failed to compile shader extension {}: {}", extension, why);
    }
}
