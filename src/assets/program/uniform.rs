use cgmath::{Matrix4, Point3, Vector3, Vector4};
use gl::types::{GLchar, GLint, GLuint};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Uniform {
    pub(crate) gl_name: String,
    pub(crate) gl_location: GLint,
}

impl Uniform {
    pub fn get_all_uniforms(program_gl_id: GLuint) -> Vec<Self> {
        let mut uniform_count = 0;
        unsafe {
            gl::GetProgramiv(program_gl_id, gl::ACTIVE_UNIFORMS, &mut uniform_count);
        }

        let mut uniforms = Vec::new();
        for i in 0..uniform_count {
            uniforms.push(Uniform::new(program_gl_id, i as u32));
        }

        uniforms
    }

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

    // fn get_uniform_location(program_id: u32, name: &str) -> i32 {
    //     let formatted_name =
    //         std::ffi::CString::new(name).expect("Failed to convert uniform name to CString.");
    //     match unsafe { gl::GetUniformLocation(program_id, formatted_name.as_ptr()) } {
    //         -1 => panic!("Failed to find uniform location: {}", name),
    //         location => location,
    //     }
    // }

    /**
     * Set a uniform boolean value.
     * @param name The name of the uniform to set.
     * @param value The value to set the uniform to.
     */
    pub fn set_uniform_bool(&self, value: bool) {
        unsafe {
            gl::Uniform1i(self.gl_location, value as i32);
        }
    }

    /**
     * Set a uniform integer value.
     * @param name The name of the uniform to set.
     * @param value The value to set the uniform to.
     */
    pub fn set_uniform_int(&self, value: i32) {
        unsafe {
            gl::Uniform1i(self.gl_location, value);
        }
    }

    /**
     * Set a uniform float value.
     * @param name The name of the uniform to set.
     * @param value The value to set the uniform to.
     */
    pub fn set_uniform_float(&self, value: f32) {
        unsafe {
            gl::Uniform1f(self.gl_location, value);
        }
    }

    /**
     * Set a uniform Mat4 value.
     * @param name The name of the uniform to set.
     * @param value The value to set the uniform to.
     */
    // TODO rename to mat4fv
    pub fn set_uniform_mat4(&self, value: &Matrix4<f32>) {
        unsafe {
            gl::UniformMatrix4fv(
                self.gl_location,
                1,
                gl::FALSE,
                value as *const _ as *const f32,
            );
        }
    }

    /**
     * Set a uniform Vec3 value.
     * @param name The name of the uniform to set.
     * @param value The value to set the uniform to.
     */
    // TODO rename to vec3f
    pub fn set_uniform_vec3(&self, value: Vector3<f32>) {
        unsafe {
            gl::Uniform3f(self.gl_location, value.x, value.y, value.z);
        }
    }

    pub fn set_uniform_vec4(&self, value: Vector4<f32>) {
        unsafe {
            gl::Uniform4f(self.gl_location, value.x, value.y, value.z, value.w);
        }
    }

    // TODO rename to vec3f
    pub fn set_uniform_point3(&self, value: Point3<f32>) {
        unsafe {
            gl::Uniform3f(self.gl_location, value.x, value.y, value.z);
        }
    }
}
