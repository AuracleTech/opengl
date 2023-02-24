extern crate gl;
extern crate glfw;

mod program;
mod shader;
mod texture;

use gl::types::*;
use glfw::{Action, Context, Key};
use glm::Vec3;
use program::Program;
use shader::Shader;
use texture::Texture;

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
    window.make_current();

    // swap interval
    glfw.set_swap_interval(glfw::SwapInterval::Sync(0));

    // get max vertex attributes (min 16 on OpenGL 3.3+)
    // TODO check if current_vertex_attribs <= max_vertex_attribs before initializing each vertex attributes
    let mut max_vertex_attribs = 0;
    unsafe {
        gl::GetIntegerv(gl::MAX_VERTEX_ATTRIBS, &mut max_vertex_attribs);
    }

    // texture
    let texture_frame = Texture::new("../assets/textures/frame.jpg");
    let texture_flume = Texture::new("../assets/textures/flume.jpg");

    // opengl settings
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

    // vertex data
    // const VERTEX_DATA: [GLfloat; 32] = [
    //     // 3 positions, 3 colors, 2 texture coords
    //     0.5, 0.5, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, // top right
    //     0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, // bottom right
    //     -0.5, -0.5, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, // bottom left
    //     -0.5, 0.5, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, // top left
    // ];
    // load obj file and assign vertex data
    const VERTEX_DATA: [GLfloat; 180] = [
        -0.5, -0.5, -0.5, 0.0, 0.0, //
        0.5, -0.5, -0.5, 1.0, 0.0, //
        0.5, 0.5, -0.5, 1.0, 1.0, //
        0.5, 0.5, -0.5, 1.0, 1.0, //
        -0.5, 0.5, -0.5, 0.0, 1.0, //
        -0.5, -0.5, -0.5, 0.0, 0.0, //
        -0.5, -0.5, 0.5, 0.0, 0.0, //
        0.5, -0.5, 0.5, 1.0, 0.0, //
        0.5, 0.5, 0.5, 1.0, 1.0, //
        0.5, 0.5, 0.5, 1.0, 1.0, //
        -0.5, 0.5, 0.5, 0.0, 1.0, //
        -0.5, -0.5, 0.5, 0.0, 0.0, //
        -0.5, 0.5, 0.5, 1.0, 0.0, //
        -0.5, 0.5, -0.5, 1.0, 1.0, //
        -0.5, -0.5, -0.5, 0.0, 1.0, //
        -0.5, -0.5, -0.5, 0.0, 1.0, //
        -0.5, -0.5, 0.5, 0.0, 0.0, //
        -0.5, 0.5, 0.5, 1.0, 0.0, //
        0.5, 0.5, 0.5, 1.0, 0.0, //
        0.5, 0.5, -0.5, 1.0, 1.0, //
        0.5, -0.5, -0.5, 0.0, 1.0, //
        0.5, -0.5, -0.5, 0.0, 1.0, //
        0.5, -0.5, 0.5, 0.0, 0.0, //
        0.5, 0.5, 0.5, 1.0, 0.0, //
        -0.5, -0.5, -0.5, 0.0, 1.0, //
        0.5, -0.5, -0.5, 1.0, 1.0, //
        0.5, -0.5, 0.5, 1.0, 0.0, //
        0.5, -0.5, 0.5, 1.0, 0.0, //
        -0.5, -0.5, 0.5, 0.0, 0.0, //
        -0.5, -0.5, -0.5, 0.0, 1.0, //
        -0.5, 0.5, -0.5, 0.0, 1.0, //
        0.5, 0.5, -0.5, 1.0, 1.0, //
        0.5, 0.5, 0.5, 1.0, 0.0, //
        0.5, 0.5, 0.5, 1.0, 0.0, //
        -0.5, 0.5, 0.5, 0.0, 0.0, //
        -0.5, 0.5, -0.5, 0.0, 1.0, //
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

    // cube positions
    let cube_positions: [glm::Vec3; 10] = [
        glm::vec3(0.0, 0.0, 0.0),
        glm::vec3(2.0, 5.0, -15.0),
        glm::vec3(-1.5, -2.2, -2.5),
        glm::vec3(-3.8, -2.0, -12.3),
        glm::vec3(2.4, -0.4, -3.5),
        glm::vec3(-1.7, 3.0, -7.5),
        glm::vec3(1.3, -2.0, -2.5),
        glm::vec3(1.5, 2.0, -2.5),
        glm::vec3(1.5, 0.2, -1.5),
        glm::vec3(-1.3, 1.0, -1.5),
    ];

    // vertex array object (VAO) uses VBO
    let mut vao = 0;
    let stride = (5 * std::mem::size_of::<GLfloat>()) as GLsizei;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
        // position attribute
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
        gl::EnableVertexAttribArray(0);
        // texture coord attribute
        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            stride,
            (3 * std::mem::size_of::<GLfloat>()) as *const GLvoid,
        );
        gl::EnableVertexAttribArray(1);
    }

    // element buffer data (EBO) (Rectangle from 2 triangles)
    // const EBO_INDEX_DATA: [GLuint; 6] = [0, 1, 3, 1, 2, 3];

    // element buffer object (EBO) uses VBO
    // let mut ebo = 0;
    // unsafe {
    //     gl::GenBuffers(1, &mut ebo);
    //     gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
    //     gl::BufferData(
    //         gl::ELEMENT_ARRAY_BUFFER,
    //         (EBO_INDEX_DATA.len() * std::mem::size_of::<GLuint>()) as GLsizeiptr,
    //         EBO_INDEX_DATA.as_ptr() as *const GLvoid,
    //         gl::STATIC_DRAW,
    //     );
    // }

    // camera
    let mut camera_pos = glm::vec3(0.0, 0.0, 3.0);
    const CAMERA_SPEED_MULTIPLIER: f32 = 50.0;
    let camera_front = glm::vec3(0.0, 0.0, -1.0);
    let camera_up = glm::vec3(0.0, 1.0, 0.0);
    // let difference = camera_pos - camera_target;
    // let camera_direction = glm::normalize(difference);
    // let camera_right = glm::normalize(glm::cross(camera_up, camera_direction));
    // let camera_up = glm::cross(camera_direction, camera_right);

    // vertex shader
    let vertex_shader = Shader::new(include_str!("shaders/vertex.glsl"), gl::VERTEX_SHADER);

    // fragment shader
    let fragment_shader = Shader::new(include_str!("shaders/fragment.glsl"), gl::FRAGMENT_SHADER);

    // shader program
    let shader_program = Program::new(&vertex_shader, &fragment_shader);

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

    // TODO deal with max amount of texture units
    texture_frame.bind(0);
    texture_flume.bind(1);

    // projection matrix manipulations
    let projection = glm::ext::perspective(45.0, WIN_ASPECT_RATIO, 0.1, 100.0);

    // use shader program
    shader_program.use_program();
    // set uniform values
    shader_program.set_uniform_int("texture_frame", 0);
    shader_program.set_uniform_int("texture_flume", 1);

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
        let current_frame = glfw.get_time() as f32;
        let delta_time = current_frame - last_frame;
        last_frame = current_frame;

        // update local uniform values
        // translate, rotate and scale matrix manipulations - order matters

        // view matrix manipulations
        let view = glm::ext::look_at(camera_pos, camera_pos + camera_front, camera_up);

        // update shader uniform values
        shader_program.set_uniform_mat4("view", &view);
        shader_program.set_uniform_mat4("projection", &projection);

        // render
        unsafe {
            gl::BindVertexArray(vao);

            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        // draw the 10 cubes
        for i in 0..10 {
            let mut model = glm::mat4(
                1.0, 0.0, 0.0, 0.0, //
                0.0, 1.0, 0.0, 0.0, //
                0.0, 0.0, 1.0, 0.0, //
                0.0, 0.0, 0.0, 1.0, //
            );
            model = glm::ext::translate(&model, cube_positions[i]);
            let angle = 20.0 * i as f32;

            model = glm::ext::rotate(
                &model,
                glm::radians(angle) + glfw.get_time() as f32,
                Vec3::new(0.5, 1.0, 0.0),
            );
            shader_program.set_uniform_mat4("model", &model);
            unsafe {
                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }
        }

        // swap buffers
        window.swap_buffers();

        // poll events (keyboard, mouse, etc)
        glfw.poll_events();

        // window events (resize, close, etc)
        for (_, event) in glfw::flush_messages(&events) {
            let camera_speed = CAMERA_SPEED_MULTIPLIER * delta_time;
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
                    camera_pos = camera_pos + (camera_front * camera_speed);
                }
                // S
                glfw::WindowEvent::Key(Key::S, _, Action::Repeat | Action::Press, _) => {
                    camera_pos = camera_pos - (camera_front * camera_speed);
                }
                // A
                glfw::WindowEvent::Key(Key::A, _, Action::Repeat | Action::Press, _) => {
                    camera_pos = camera_pos
                        - (glm::normalize(glm::cross(camera_front, camera_up)) * camera_speed);
                }
                // D
                glfw::WindowEvent::Key(Key::D, _, Action::Repeat | Action::Press, _) => {
                    camera_pos = camera_pos
                        + (glm::normalize(glm::cross(camera_front, camera_up)) * camera_speed);
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
