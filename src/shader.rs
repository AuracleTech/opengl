use crate::{assets_path, types::Shader};
use gl::types::{GLchar, GLenum};
use std::{ffi::CString, fs::File, io::Read};

const PATH: &str = "shaders";
const EXT_VERTEX_SHADER: &str = "vs";
const EXT_FRAGMENT_SHADER: &str = "fs";
const EXT_GEOMETRY_SHADER: &str = "gs";

impl Shader {
    pub fn new(name: &str, shader_type: GLenum) -> Self {
        let mut path = assets_path().join(PATH).join(&name);
        path = match shader_type {
            gl::VERTEX_SHADER => path.with_extension(EXT_VERTEX_SHADER),
            gl::FRAGMENT_SHADER => path.with_extension(EXT_FRAGMENT_SHADER),
            gl::GEOMETRY_SHADER => path.with_extension(EXT_GEOMETRY_SHADER),
            _ => panic!("Invalid shader type."),
        };

        let mut source = String::new();
        let mut file = File::open(path).expect("Failed to open shader file.");
        file.read_to_string(&mut source)
            .expect("Failed to read shader file.");

        let id = unsafe { gl::CreateShader(shader_type) };

        if id <= 0 {
            panic!("The shader id is invalid.");
        }

        // Compile shader
        let source = CString::new(source).expect("Failed to convert source to CString.");
        unsafe {
            gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
            gl::CompileShader(id);
        }

        // Verify shader compiled
        let mut success = 0;
        unsafe {
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        }

        if success != 0 {
            return Self { id };
        }

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
        let why = String::from_utf8(log).expect("Failed to convert log to String.");
        panic!(
            "Failed to compile '{}' shader: {}",
            match shader_type {
                gl::VERTEX_SHADER => "Vertex",
                gl::FRAGMENT_SHADER => "Fragment",
                gl::GEOMETRY_SHADER => "Geometry",
                _ => "Other",
            },
            why
        );
    }
}
