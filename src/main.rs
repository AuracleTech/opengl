extern crate gl;
extern crate glfw;

mod program;
mod shader;

use gl::types::*;
use glfw::{Action, Context, Key};
use image::DynamicImage;
use program::Program;
use shader::Shader;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("Failed to initialize GLFW.");

    // set opengl version to 3.3 core profile
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    let (mut window, events) = glfw
        .create_window(
            1280,
            720,
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

    // get max vertex attributes (min 16 on OpenGL 3.3+)
    // TODO check if current_vertex_attribs <= max_vertex_attribs before initializing each vertex attributes
    let mut max_vertex_attribs = 0;
    unsafe {
        gl::GetIntegerv(gl::MAX_VERTEX_ATTRIBS, &mut max_vertex_attribs);
    }

    // texture
    let texture_image = image::open("../assets/textures/frame.jpg")
        .expect("Failed to load texture.")
        .flipv();
    let width = texture_image.width();
    let height = texture_image.height();
    let _nr_channels = texture_image.color().channel_count();
    dbg!(width, height, _nr_channels);
    let data = match texture_image {
        DynamicImage::ImageRgb8(texture_image) => texture_image.into_raw(),
        _ => panic!("Image format not supported"),
    };

    let mut texture = 0;
    unsafe {
        // generate texture id
        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);
        // set texture wrapping
        gl::TexParameterfv(
            gl::TEXTURE_2D,
            gl::TEXTURE_BORDER_COLOR,
            [1.0, 1.0, 0.0, 1.0].as_ptr(),
        );
        // set texture filtering
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MIN_FILTER,
            gl::LINEAR_MIPMAP_LINEAR as i32,
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        // set texture data
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as i32,
            width as i32,
            height as i32,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            data.as_ptr() as *const _,
        );
        // generates mipmaps
        gl::GenerateMipmap(gl::TEXTURE_2D);
    }

    // vertex data
    const VERTEX_DATA: [GLfloat; 32] = [
        // 3 positions, 3 colors, 2 texture coords
        0.5, 0.5, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, // top right
        0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, // bottom right
        -0.5, -0.5, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, // bottom left
        -0.5, 0.5, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, // top left
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
        // position attribute
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (8 * std::mem::size_of::<GLfloat>()) as GLsizei,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);
        // color attribute
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            (8 * std::mem::size_of::<GLfloat>()) as GLsizei,
            (3 * std::mem::size_of::<GLfloat>()) as *const GLvoid,
        );
        gl::EnableVertexAttribArray(1);
        // texture coord attribute
        gl::VertexAttribPointer(
            2,
            2,
            gl::FLOAT,
            gl::FALSE,
            (8 * std::mem::size_of::<GLfloat>()) as GLsizei,
            (6 * std::mem::size_of::<GLfloat>()) as *const GLvoid,
        );
        gl::EnableVertexAttribArray(2);
    }

    // element buffer data (EBO) (Rectangle from 2 triangles)
    const EBO_INDEX_DATA: [GLuint; 6] = [0, 1, 3, 1, 2, 3];

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

    // vertex shader
    static VERTEX_SHADER_SRC: &str = include_str!("shaders/vertex.glsl");
    let vertex_shader = Shader::new(VERTEX_SHADER_SRC, gl::VERTEX_SHADER);

    // fragment shader
    const FRAGMENT_SHADER_SRC: &'static str = include_str!("shaders/fragment.glsl");
    let fragment_shader = Shader::new(FRAGMENT_SHADER_SRC, gl::FRAGMENT_SHADER);

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

    // opengl setup
    shader_program.use_program();
    unsafe {
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
        gl::BindTexture(gl::TEXTURE_2D, texture);
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
            // clear the screen
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::BindVertexArray(vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }

        // swap buffers
        window.swap_buffers();

        // poll events (keyboard, mouse, etc)
        glfw.poll_events();

        // window events (resize, close, etc)
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
        // resize the viewport when the window is resized
        glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
            gl::Viewport(0, 0, width, height);
        },
        _ => {}
    }
}
