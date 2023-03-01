extern crate gl;
extern crate glfw;

mod camera;
mod program;
mod shader;
mod texture;

use camera::Camera;
use gl::types::*;
use glfw::{Action, Context, Key};
use glm::{Vec3, Vec4};
use program::Program;
use shader::Shader;

const WIN_WIDTH: u32 = 1200;
const WIN_HEIGHT: u32 = 900;
const WIN_ASPECT_RATIO: f32 = WIN_WIDTH as f32 / WIN_HEIGHT as f32;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("Failed to initialize GLFW.");

    // set opengl version to 3.3 core profile
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    let (mut window, events) = glfw
        .create_window(
            WIN_WIDTH,
            WIN_HEIGHT,
            env!("CARGO_PKG_NAME"),
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window.");

    // verify opengl version is 3.3 or higher
    let version = window.get_context_version();
    if version.major < 3 || (version.major == 3 && version.minor < 3) {
        panic!("OpenGL version 3.3 or higher is required.");
    }

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    window.set_framebuffer_size_polling(true);
    window.set_key_polling(true);
    window.set_scroll_polling(true);
    window.set_cursor_pos_polling(true);
    glfw.set_swap_interval(glfw::SwapInterval::Sync(0));
    window.set_cursor_mode(glfw::CursorMode::Disabled);
    window.make_current();

    // Cursor position
    let mut last_x = WIN_WIDTH as f64 / 2.0;
    let mut last_y = WIN_HEIGHT as f64 / 2.0;
    let mut yaw = -90.0;
    let mut pitch = 0.0;

    // get max vertex attributes (min 16 on OpenGL 3.3+)
    // TODO check if current_vertex_attribs <= max_vertex_attribs before initializing each vertex attributes
    let mut max_vertex_attribs = 0;
    unsafe {
        gl::GetIntegerv(gl::MAX_VERTEX_ATTRIBS, &mut max_vertex_attribs);
    }

    // opengl settings
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

    let light_shader_fs = Shader::new(include_str!("shaders/light.fs"), gl::FRAGMENT_SHADER);
    let light_shader_vs = Shader::new(include_str!("shaders/light.vs"), gl::VERTEX_SHADER);
    let light_program = Program::new(light_shader_fs, light_shader_vs);

    let toy_coral_color = Vec3::new(1.0, 0.5, 0.31);
    let light_white_color = Vec3::new(1.0, 1.0, 1.0);
    let light_position = Vec3::new(1.2, 1.0, 2.0);
    let mut light_model = glm::Mat4::new(
        Vec4::new(1.0, 0.0, 0.0, 0.0),
        Vec4::new(0.0, 1.0, 0.0, 0.0),
        Vec4::new(0.0, 0.0, 1.0, 0.0),
        Vec4::new(0.0, 0.0, 0.0, 1.0),
    );
    light_model = glm::ext::translate(&light_model, light_position);
    light_model = glm::ext::scale(&light_model, Vec3::new(0.2, 0.2, 0.2));

    // vertex data
    const VERTEX_DATA: [GLfloat; 216] = [
        -0.5, -0.5, -0.5, 0.0, 0.0, -1.0, //
        0.5, -0.5, -0.5, 0.0, 0.0, -1.0, //
        0.5, 0.5, -0.5, 0.0, 0.0, -1.0, //
        0.5, 0.5, -0.5, 0.0, 0.0, -1.0, //
        -0.5, 0.5, -0.5, 0.0, 0.0, -1.0, //
        -0.5, -0.5, -0.5, 0.0, 0.0, -1.0, //
        -0.5, -0.5, 0.5, 0.0, 0.0, 1.0, //
        0.5, -0.5, 0.5, 0.0, 0.0, 1.0, //
        0.5, 0.5, 0.5, 0.0, 0.0, 1.0, //
        0.5, 0.5, 0.5, 0.0, 0.0, 1.0, //
        -0.5, 0.5, 0.5, 0.0, 0.0, 1.0, //
        -0.5, -0.5, 0.5, 0.0, 0.0, 1.0, //
        -0.5, 0.5, 0.5, -1.0, 0.0, 0.0, //
        -0.5, 0.5, -0.5, -1.0, 0.0, 0.0, //
        -0.5, -0.5, -0.5, -1.0, 0.0, 0.0, //
        -0.5, -0.5, -0.5, -1.0, 0.0, 0.0, //
        -0.5, -0.5, 0.5, -1.0, 0.0, 0.0, //
        -0.5, 0.5, 0.5, -1.0, 0.0, 0.0, //
        0.5, 0.5, 0.5, 1.0, 0.0, 0.0, //
        0.5, 0.5, -0.5, 1.0, 0.0, 0.0, //
        0.5, -0.5, -0.5, 1.0, 0.0, 0.0, //
        0.5, -0.5, -0.5, 1.0, 0.0, 0.0, //
        0.5, -0.5, 0.5, 1.0, 0.0, 0.0, //
        0.5, 0.5, 0.5, 1.0, 0.0, 0.0, //
        -0.5, -0.5, -0.5, 0.0, -1.0, 0.0, //
        0.5, -0.5, -0.5, 0.0, -1.0, 0.0, //
        0.5, -0.5, 0.5, 0.0, -1.0, 0.0, //
        0.5, -0.5, 0.5, 0.0, -1.0, 0.0, //
        -0.5, -0.5, 0.5, 0.0, -1.0, 0.0, //
        -0.5, -0.5, -0.5, 0.0, -1.0, 0.0, //
        -0.5, 0.5, -0.5, 0.0, 1.0, 0.0, //
        0.5, 0.5, -0.5, 0.0, 1.0, 0.0, //
        0.5, 0.5, 0.5, 0.0, 1.0, 0.0, //
        0.5, 0.5, 0.5, 0.0, 1.0, 0.0, //
        -0.5, 0.5, 0.5, 0.0, 1.0, 0.0, //
        -0.5, 0.5, -0.5, 0.0, 1.0, 0.0, //
    ];

    // vertex buffer object (VBO)
    let mut vbo = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (VERTEX_DATA.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
            VERTEX_DATA.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        );
    }

    // vertex array object (VAO) uses VBO
    let mut vao = 0;
    let main_stride = (6 * std::mem::size_of::<GLfloat>()) as GLsizei;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
        // position attribute
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, main_stride, std::ptr::null());
        gl::EnableVertexAttribArray(0);
        // normal attribute
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            main_stride,
            (3 * std::mem::size_of::<GLfloat>()) as *const GLvoid,
        );
        gl::EnableVertexAttribArray(1);
    }

    let mut camera = Camera {
        pos: glm::vec3(0.0, 0.0, 3.0),
        front: glm::vec3(0.0, 0.0, -1.0),
        up: glm::vec3(0.0, 1.0, 0.0),
        right: glm::vec3(0.0, 0.0, 0.0),
        speed_factor: 500.0,
        fov_y: 45.0,
        speed: 0.0,
    };

    // shaders
    let fragment_shader = Shader::new(include_str!("shaders/fragment.fs"), gl::FRAGMENT_SHADER);
    let vertex_shader = Shader::new(include_str!("shaders/vertex.vs"), gl::VERTEX_SHADER);
    let shader_program = Program::new(vertex_shader, fragment_shader);

    // copy vertex data to buffer
    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (VERTEX_DATA.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
            VERTEX_DATA.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        );
    }

    // light VAO
    let mut light_vao = 0;
    let light_stride = (6 * std::mem::size_of::<GLfloat>()) as GLsizei;
    unsafe {
        gl::GenVertexArrays(1, &mut light_vao);
        gl::BindVertexArray(light_vao);
        // we only need to bind to the VBO, the container's VBO's data already contains the data
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        // set the vertex attributes (only position data for our lamp)
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, light_stride, std::ptr::null());
        gl::EnableVertexAttribArray(0);
    }

    unsafe {
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    // calculate fps declarations
    let mut last_time = glfw.get_time();
    let mut frames_rendered = 0;

    // last frame time and delta time
    let mut last_frame = 0.0;

    // main loop
    while !window.should_close() {
        // SECTION main loop

        shader_program.use_program();

        let current_frame = glfw.get_time() as f32;
        let delta_time = current_frame - last_frame;
        last_frame = current_frame;

        camera.speed = camera.speed_factor * delta_time;

        // Translate - Rotate - Scale (TRS) matrix manipulations
        let projection =
            glm::ext::perspective(glm::radians(camera.fov_y), WIN_ASPECT_RATIO, 0.1, 100.0);
        let view = glm::ext::look_at(camera.pos, camera.pos + camera.front, camera.up);

        // update local uniform values
        shader_program.set_uniform_mat4("view", &view);
        shader_program.set_uniform_mat4("projection", &projection);
        shader_program.set_uniform_vec3("object_color", toy_coral_color);
        shader_program.set_uniform_vec3("light_color", light_white_color);
        shader_program.set_uniform_vec3("light_pos", light_position);
        shader_program.set_uniform_vec3("camera_pos", camera.pos);

        let mut model = glm::Mat4::new(
            Vec4::new(1.0, 0.0, 0.0, 0.0),
            Vec4::new(0.0, 1.0, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 1.0, 0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        );
        let angle = 20.0 * glfw.get_time() as f32;
        model = glm::ext::rotate(&model, glm::radians(angle), Vec3::new(1.0, 0.6, 0.0));
        shader_program.set_uniform_mat4("model", &model);
        unsafe {
            gl::BindVertexArray(vao);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }

        // SECTION cube light

        light_program.use_program();

        light_program.set_uniform_mat4("view", &view);
        light_program.set_uniform_mat4("projection", &projection);
        light_program.set_uniform_mat4("model", &light_model);

        unsafe {
            gl::BindVertexArray(light_vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }

        // SECTION swap buffers & poll events

        window.swap_buffers();
        glfw.poll_events();

        for (_, event) in glfw::flush_messages(&events) {
            match event {
                // ESC closes the window
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                }
                // P cycle through polygon modes
                glfw::WindowEvent::Key(Key::P, _, Action::Press, _) => {
                    let mut polygon_mode = [0];
                    unsafe {
                        gl::GetIntegerv(gl::POLYGON_MODE, polygon_mode.as_mut_ptr());
                    }
                    let polygon_mode = match polygon_mode[0] as GLenum {
                        gl::FILL => gl::LINE,
                        gl::LINE => gl::POINT,
                        gl::POINT => gl::FILL,
                        _ => panic!("Unknown polygon mode"),
                    };
                    unsafe {
                        gl::PolygonMode(gl::FRONT_AND_BACK, polygon_mode);
                    }
                    println!("Polygon mode: {}", polygon_mode);
                }
                // resize the viewport when the window is resized
                glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
                    gl::Viewport(0, 0, width, height);
                },
                // W
                glfw::WindowEvent::Key(Key::W, _, Action::Repeat | Action::Press, _) => {
                    camera.pos = camera.pos + (camera.front * camera.speed);
                }
                // S
                glfw::WindowEvent::Key(Key::S, _, Action::Repeat | Action::Press, _) => {
                    camera.pos = camera.pos - (camera.front * camera.speed);
                }
                // A
                glfw::WindowEvent::Key(Key::A, _, Action::Repeat | Action::Press, _) => {
                    camera.pos = camera.pos
                        - (glm::normalize(glm::cross(camera.front, camera.up)) * camera.speed);
                }
                // D
                glfw::WindowEvent::Key(Key::D, _, Action::Repeat | Action::Press, _) => {
                    camera.pos = camera.pos
                        + (glm::normalize(glm::cross(camera.front, camera.up)) * camera.speed);
                }
                // scroll
                glfw::WindowEvent::Scroll(_xoffset, yoffset) => {
                    camera.fov_y -= yoffset as f32;
                    camera.fov_y = camera.fov_y.max(1.0).min(45.0);
                }
                // mouse movement
                glfw::WindowEvent::CursorPos(xpos, ypos) => {
                    let xoffset = xpos - last_x;
                    let yoffset = last_y - ypos;
                    last_x = xpos;
                    last_y = ypos;

                    let sensitivity = 0.05;
                    let offset_x = xoffset as f32 * sensitivity;
                    let offset_y = yoffset as f32 * sensitivity;

                    yaw += offset_x;
                    pitch += offset_y;

                    pitch = pitch.clamp(-89.9, 89.9);

                    camera.front = glm::normalize(Vec3::new(
                        yaw.to_radians().cos() * pitch.to_radians().cos(),
                        pitch.to_radians().sin(),
                        yaw.to_radians().sin() * pitch.to_radians().cos(),
                    ));
                }
                _ => {}
            }
        }

        // calculate fps and print to console
        frames_rendered += 1;
        let current_time = glfw.get_time();
        if current_time - last_time >= 1.0 {
            println!(
                "{} fps {:0.4} ms/draw",
                frames_rendered,
                1000.0 / frames_rendered as f64
            );
            frames_rendered = 0;
            last_time = current_time;
        }
    }
}
