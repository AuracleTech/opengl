use cgmath::{Matrix4, Point3, Vector3, Vector4};
use gl::types::{GLchar, GLint, GLuint};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Uniform {
    pub(crate) gl_name: String,
    pub(crate) gl_location: GLint,
}

impl Uniform {
    pub fn new(program_gl_id: GLuint, gl_location: GLuint) -> Self {
        let mut name = [0 as GLchar; 256];
        let mut name_length = 0;
        let mut size = 0;
        let mut uniform_type = 0;
        unsafe {
            gl::GetActiveUniform(
                program_gl_id,
                gl_location,
                256,
                &mut name_length,
                &mut size,
                &mut uniform_type,
                name.as_mut_ptr(),
            );
        }

        Self {
            gl_name: unsafe { std::ffi::CStr::from_ptr(name.as_ptr()) }
                .to_str()
                .expect("Failed to convert uniform name to str.")
                .to_string(),
            gl_location: gl_location as GLint,
        }
    }

    pub fn get_all_uniforms(program_gl_id: GLuint) -> Vec<Self> {
        let mut uniform_count = 0;
        unsafe {
            gl::GetProgramiv(program_gl_id, gl::ACTIVE_UNIFORMS, &mut uniform_count);
        }

        let mut uniforms = Vec::new();
        for i in 0..uniform_count {
            uniforms.push(Self::new(program_gl_id, i as u32));
        }

        uniforms
    }

    pub fn set_bool(&self, value: bool) {
        unsafe {
            gl::Uniform1i(self.gl_location, value as i32);
        }
    }

    pub fn set_int(&self, value: i32) {
        unsafe {
            gl::Uniform1i(self.gl_location, value);
        }
    }

    pub fn set_float(&self, value: f32) {
        unsafe {
            gl::Uniform1f(self.gl_location, value);
        }
    }

    pub fn set_vec3f32(&self, value: Vector3<f32>) {
        unsafe {
            gl::Uniform3f(self.gl_location, value.x, value.y, value.z);
        }
    }

    pub fn set_uniform_vec4f32(&self, value: Vector4<f32>) {
        unsafe {
            gl::Uniform4f(self.gl_location, value.x, value.y, value.z, value.w);
        }
    }

    pub fn set_point3f32(&self, value: Point3<f32>) {
        unsafe {
            gl::Uniform3f(self.gl_location, value.x, value.y, value.z);
        }
    }

    pub fn set_mat4f32(&self, value: &Matrix4<f32>) {
        unsafe {
            gl::UniformMatrix4fv(
                self.gl_location,
                1,
                gl::FALSE,
                value as *const _ as *const f32,
            );
        }
    }
}
