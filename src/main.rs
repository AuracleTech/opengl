extern crate gl;
extern crate glfw;

use gl::types::*;
use glfw::{Action, Context, Key};

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw
        .create_window(
            1280,
            720,
            env!("CARGO_PKG_NAME"),
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window.");

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    window.set_key_polling(true);
    window.make_current();

    // get max vertex attributes (min 16 on OpenGL 3.3)
    let mut max_vertex_attribs = 0;
    unsafe {
        gl::GetIntegerv(gl::MAX_VERTEX_ATTRIBS, &mut max_vertex_attribs);
    }
    dbg!(max_vertex_attribs);

    // vertex data
    static VERTEX_DATA: [GLfloat; 12] = [
        0.5, 0.5, 0.0, // top right
        0.5, -0.5, 0.0, // bottom right
        -0.5, -0.5, 0.0, // bottom left
        -0.5, 0.5, 0.0, // top left
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
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (3 * std::mem::size_of::<GLfloat>()) as GLsizei,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);
    }

    // element buffer data
    static EBO_INDEX_DATA: [GLuint; 6] = [0, 1, 3, 1, 2, 3];

    // element buffer object (EBO) uses VBO
    let mut ebo = 0;
    unsafe {
        gl::GenBuffers(1, &mut ebo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (EBO_INDEX_DATA.len() * std::mem::size_of::<GLuint>()) as GLsizeiptr,
            EBO_INDEX_DATA.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        );
    }

    // vertex shader source
    const VERTEX_SHADER_SRC: &'static str = include_str!("shaders/vertex.glsl");

    // vertex shader
    let vertex_shader = unsafe {
        let shader = gl::CreateShader(gl::VERTEX_SHADER);
        gl::ShaderSource(
            shader,
            1,
            [VERTEX_SHADER_SRC.as_ptr() as *const GLchar].as_ptr(),
            std::ptr::null(),
        );
        gl::CompileShader(shader);
        shader
    };

    if unsafe { gl::GetError() } != gl::NO_ERROR {
        panic!("Failed to compile vertex shader. Error: {}", unsafe {
            gl::GetError()
        });
    }

    // fragment shader source
    const FRAGMENT_SHADER_SRC: &'static str = include_str!("shaders/fragment.glsl");

    // fragment shader
    let fragment_shader = unsafe {
        let shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        gl::ShaderSource(
            shader,
            1,
            [FRAGMENT_SHADER_SRC.as_ptr() as *const GLchar].as_ptr(),
            std::ptr::null(),
        );
        gl::CompileShader(shader);
        shader
    };

    if unsafe { gl::GetError() } != gl::NO_ERROR {
        panic!("Failed to compile fragment shader. Error: {}", unsafe {
            gl::GetError()
        });
    }

    // shader program
    let shader_program = unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, fragment_shader);
        gl::LinkProgram(program);
        program
    };

    if unsafe { gl::GetError() } != gl::NO_ERROR {
        panic!("Failed to link shader program. Error: {}", unsafe {
            gl::GetError()
        });
    }

    // cleanup compiled shaders
    unsafe {
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);
    }

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
    // vertex attribute pointers
    unsafe {
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (3 * std::mem::size_of::<GLfloat>()) as GLsizei,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);
    }

    // define clear color
    unsafe {
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
        gl::UseProgram(shader_program);
    }

    // swap interval
    glfw.set_swap_interval(glfw::SwapInterval::Sync(0));

    // calculate fps declarations
    let mut last_time = glfw.get_time();
    let mut nb_frames = 0;

    // main loop
    while !window.should_close() {
        // render
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::BindVertexArray(vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }

        // call events and swap the buffers
        window.swap_buffers();
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

        // calculate fps and print to console
        let current_time = glfw.get_time();
        nb_frames += 1;
        if current_time - last_time >= 1.0 {
            println!(
                "{} fps {:0.3} ms/draw",
                nb_frames,
                1000.0 / nb_frames as f64
            );
            nb_frames = 0;
            last_time = current_time;
        }
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        // ESC closes the window
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        // W toggles wireframe mode
        glfw::WindowEvent::Key(Key::W, _, Action::Press, _) => unsafe {
            let mut polygon_mode = [0];
            gl::GetIntegerv(gl::POLYGON_MODE, polygon_mode.as_mut_ptr());
            match polygon_mode[0] as u32 {
                gl::FILL => {
                    gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
                    println!("PolygonMode: LINE");
                }
                gl::LINE => {
                    gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
                    println!("PolygonMode: FILL");
                }
                _ => panic!("PolygonMode: Unknown"),
            }
        },
        // event for resizing the window
        glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
            gl::Viewport(0, 0, width, height);
        },
        _ => {}
    }
}
