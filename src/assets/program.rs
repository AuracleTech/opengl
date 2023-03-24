use crate::types::{Program, Shader};
use cgmath::{Matrix4, Point3, Vector3, Vector4};
use gl::types::GLchar;

impl Program {
    pub fn new(vertex_shader: Shader, fragment_shader: Shader) -> Self {
        // shader program
        let shader_program = unsafe {
            let program = gl::CreateProgram();
            gl::AttachShader(program, vertex_shader.gl_id);
            gl::AttachShader(program, fragment_shader.gl_id);
            gl::LinkProgram(program);
            program
        };

        if shader_program <= 0 {
            panic!("Failed to create shader program");
        }

        // shader program link verification
        let mut success = 0;
        unsafe {
            gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
        }
        if success == 0 {
            let mut log_length = 0;
            unsafe {
                gl::GetProgramiv(shader_program, gl::INFO_LOG_LENGTH, &mut log_length);
            }
            let mut log = Vec::with_capacity(log_length as usize);
            unsafe {
                gl::GetProgramInfoLog(
                    shader_program,
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

        // cleanup
        unsafe {
            gl::DeleteShader(vertex_shader.gl_id);
            gl::DeleteShader(fragment_shader.gl_id);
        }

        Self {
            gl_id: shader_program,
        }
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.gl_id);
        }
    }

    /**
     * Set a uniform boolean value.
     * @param name The name of the uniform to set.
     * @param value The value to set the uniform to.
     */
    pub fn set_uniform_bool(&self, name: &str, value: bool) {
        unsafe {
            gl::Uniform1i(get_uniform_location(self.gl_id, name), value as i32);
        }
    }

    /**
     * Set a uniform integer value.
     * @param name The name of the uniform to set.
     * @param value The value to set the uniform to.
     */
    pub fn set_uniform_int(&self, name: &str, value: i32) {
        unsafe {
            gl::Uniform1i(get_uniform_location(self.gl_id, name), value);
        }
    }

    /**
     * Set a uniform float value.
     * @param name The name of the uniform to set.
     * @param value The value to set the uniform to.
     */
    pub fn set_uniform_float(&self, name: &str, value: f32) {
        unsafe {
            gl::Uniform1f(get_uniform_location(self.gl_id, name), value);
        }
    }

    /**
     * Set a uniform Mat4 value.
     * @param name The name of the uniform to set.
     * @param value The value to set the uniform to.
     */
    // TODO rename to mat4fv
    pub fn set_uniform_mat4(&self, name: &str, value: &Matrix4<f32>) {
        unsafe {
            gl::UniformMatrix4fv(
                get_uniform_location(self.gl_id, name),
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
    pub fn set_uniform_vec3(&self, name: &str, value: Vector3<f32>) {
        unsafe {
            gl::Uniform3f(
                get_uniform_location(self.gl_id, name),
                value.x,
                value.y,
                value.z,
            );
        }
    }

    pub fn set_uniform_vec4(&self, name: &str, value: Vector4<f32>) {
        unsafe {
            gl::Uniform4f(
                get_uniform_location(self.gl_id, name),
                value.x,
                value.y,
                value.z,
                value.w,
            );
        }
    }

    // TODO rename to vec3f
    pub fn set_uniform_point3(&self, name: &str, value: Point3<f32>) {
        unsafe {
            gl::Uniform3f(
                get_uniform_location(self.gl_id, name),
                value.x,
                value.y,
                value.z,
            );
        }
    }
}

// TODO this should be a new file called uniform.rs
// TODO during runtime this should be cached in a hashmap
fn get_uniform_location(program_id: u32, name: &str) -> i32 {
    let formatted_name =
        std::ffi::CString::new(name).expect("Failed to convert uniform name to CString.");
    match unsafe { gl::GetUniformLocation(program_id, formatted_name.as_ptr()) } {
        -1 => panic!("Failed to find uniform location: {}", name),
        location => location,
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.gl_id);
        }
    }
}
