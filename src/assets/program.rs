mod uniform;
mod vertex_attribute;

use self::{uniform::Uniform, vertex_attribute::VertexAttribute};
use cgmath::Matrix4;
use gl::types::{GLchar, GLuint};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Program {
    pub gl_id: GLuint,
    pub uniforms: Vec<Uniform>, // OPTIMIZE use hashmap
    pub vertex_attributes: Vec<VertexAttribute>,
}

impl Program {
    pub fn new(shaders_gl_ids: Vec<GLuint>) -> Self {
        let gl_id = unsafe { gl::CreateProgram() };

        for shader_gl_id in shaders_gl_ids {
            // TODO verifications (shader already attached, shader not compiled, etc.)
            unsafe {
                gl::AttachShader(gl_id, shader_gl_id);
            }
        }
        unsafe {
            gl::LinkProgram(gl_id);
        }

        if gl_id <= 0 {
            panic!("Failed to link shader program");
        }

        let program = Self::verify_link(gl_id);
        dbg!(program.uniforms.clone());
        program
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.gl_id);
        }
    }

    fn verify_link(gl_id: GLuint) -> Self {
        let mut success = 0;
        unsafe {
            gl::GetProgramiv(gl_id, gl::LINK_STATUS, &mut success);
        }
        if success == 0 {
            let mut log_length = 0;
            unsafe {
                gl::GetProgramiv(gl_id, gl::INFO_LOG_LENGTH, &mut log_length);
            }
            let mut log = Vec::with_capacity(log_length as usize);
            unsafe {
                gl::GetProgramInfoLog(
                    gl_id,
                    log_length,
                    std::ptr::null_mut(),
                    log.as_mut_ptr() as *mut GLchar,
                );
                log.set_len(log_length as usize);
            }
            panic!(
                "Failed to link shader program: {}",
                String::from_utf8(log).expect("Shader program log is not valid UTF-8.")
            );
        }

        Self {
            gl_id,
            uniforms: Uniform::get_all_uniforms(gl_id),
            vertex_attributes: vec![], // TODO VertexAttribute::get_all_vertex_attributes(gl_id),
        }
    }

    pub fn set_uniform_int(&self, name: &str, value: i32) {
        // OPTIMIZE replace uniform ved by hashmap
        if let Some(uniform) = self.uniforms.iter().find(|uniform| uniform.gl_name == name) {
            uniform.set_uniform_int(value);
        }
    }

    pub fn set_uniform_mat4(&self, name: &str, value: &Matrix4<f32>) {
        // OPTIMIZE replace uniform ved by hashmap
        if let Some(uniform) = self.uniforms.iter().find(|uniform| uniform.gl_name == name) {
            uniform.set_uniform_mat4(value);
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.gl_id);
        }
    }
}
