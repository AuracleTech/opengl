extern crate gl;
extern crate glfw;

mod camera;
mod program;
mod shader;
mod texture;

use camera::Camera;
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
    window.set_scroll_polling(true);
    window.set_cursor_pos_polling(true);
    glfw.set_swap_interval(glfw::SwapInterval::Sync(0));
    window.set_cursor_mode(glfw::CursorMode::Disabled);
    window.make_current();

    // CHANGE mouse input
    let mut first_mouse = true;
    let mut last_x = 0.0;
    let mut last_y = 0.0;
    let mut yaw = -90.0;
    let mut pitch = 0.0;

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

        camera.speed = camera.speed_factor * delta_time;

        // update local uniform values
        // translate, rotate and scale matrix manipulations - order matters

        // projection matrix manipulations
        let projection =
            glm::ext::perspective(glm::radians(camera.fov_y), WIN_ASPECT_RATIO, 0.1, 100.0);

        // view matrix manipulations
        let view = glm::ext::look_at(camera.pos, camera.pos + camera.front, camera.up);

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
                    if first_mouse {
                        last_x = xpos;
                        last_y = ypos;
                        first_mouse = false;
                    }

                    let xoffset = xpos - last_x;
                    let yoffset = last_y - ypos;
                    last_x = xpos;
                    last_y = ypos;

                    let sensitivity = 0.05;
                    let xoffset = xoffset as f32 * sensitivity;
                    let yoffset = yoffset as f32 * sensitivity;

                    yaw += xoffset;
                    pitch += yoffset;

                    if pitch > 89.0 {
                        pitch = 89.0;
                    }
                    if pitch < -89.0 {
                        pitch = -89.0;
                    }

                    let front = Vec3::new(
                        yaw.to_radians().cos() * pitch.to_radians().cos(),
                        pitch.to_radians().sin(),
                        yaw.to_radians().sin() * pitch.to_radians().cos(),
                    );
                    camera.front = glm::normalize(front);
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
