use crate::shader::Shader;
use gl::types::GLchar;

/**
 * Program object.
 * @field id The OpenGL ID of the program.
 */
pub struct Program {
    id: u32,
}

#[allow(dead_code)]
impl Program {
    /**
     * Create a new shader program.
     * @param vertex_shader The vertex shader to use.
     * @param fragment_shader The fragment shader to use.
     */
    pub fn new(vertex_shader: &Shader, fragment_shader: &Shader) -> Self {
        // shader program
        let shader_program = unsafe {
            let program = gl::CreateProgram();
            gl::AttachShader(program, vertex_shader.id);
            gl::AttachShader(program, fragment_shader.id);
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
            gl::DeleteShader(vertex_shader.id);
            gl::DeleteShader(fragment_shader.id);
        }

        Self { id: shader_program }
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    /**
     * Set a uniform boolean value.
     * @param uniform_name The name of the uniform to set.
     * @param value The value to set the uniform to.
     */
    pub fn set_uniform_bool(&self, uniform_name: &str, value: bool) {
        unsafe {
            gl::Uniform1i(get_uniform_location(self.id, uniform_name), value as i32);
        }
    }

    /**
     * Set a uniform integer value.
     * @param uniform_name The name of the uniform to set.
     * @param value The value to set the uniform to.
     */
    pub fn set_uniform_int(&self, uniform_name: &str, value: i32) {
        unsafe {
            gl::Uniform1i(get_uniform_location(self.id, uniform_name), value);
        }
    }

    /**
     * Set a uniform float value.
     * @param uniform_name The name of the uniform to set.
     * @param value The value to set the uniform to.
     */
    pub fn set_uniform_float(&self, uniform_name: &str, value: f32) {
        unsafe {
            gl::Uniform1f(get_uniform_location(self.id, uniform_name), value);
        }
    }
}

fn get_uniform_location(program_id: u32, uniform_name: &str) -> i32 {
    let name = uniform_name.as_ptr() as *const GLchar;
    let uniform_location = unsafe { gl::GetUniformLocation(program_id, name) };

    // Verify uniform location was found
    if uniform_location < 0 {
        panic!("Failed to find uniform {}", uniform_name);
    }
    uniform_location
}
