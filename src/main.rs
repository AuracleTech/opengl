extern crate gl;
extern crate glfw;

mod camera;
mod light;
mod material;
mod program;
mod shader;
mod texture;

use gl::types::*;
use glfw::{Context, Key};
use glm::{Mat4, Vec3, Vec4};

use camera::Camera;
use light::{DirLight, PointLight, SpotLight};
use material::Material;
use program::Program;
use shader::Shader;
use texture::Texture;

const WIN_WIDTH: u32 = 1200;
const WIN_HEIGHT: u32 = 900;
const WIN_ASPECT_RATIO: f32 = WIN_WIDTH as f32 / WIN_HEIGHT as f32;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("Failed to initialize GLFW.");

    // set opengl version to 4.6 core profile
    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 6));
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

    let version = window.get_context_version();
    println!("OpenGL version: {}.{}", version.major, version.minor);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    window.set_framebuffer_size_polling(true);
    window.set_key_polling(true);
    window.set_scroll_polling(true);
    window.set_cursor_pos_polling(true);
    glfw.set_swap_interval(glfw::SwapInterval::Sync(0));
    window.set_cursor_mode(glfw::CursorMode::Disabled);
    window.make_current();

    // TODO check if current_vertex_attribs <= max_vertex_attribs before initializing each vertex attributes
    let mut max_vertex_attribs = 0;
    unsafe {
        gl::GetIntegerv(gl::MAX_VERTEX_ATTRIBS, &mut max_vertex_attribs);
    }

    // opengl settings
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

    // vertex data (pos 3, normal 3, texcoord 2)
    const VERTEX_DATA: [GLfloat; 288] = [
        -0.5, -0.5, -0.5, 0.0, 0.0, -1.0, 0.0, 0.0, //
        0.5, -0.5, -0.5, 0.0, 0.0, -1.0, 1.0, 0.0, //
        0.5, 0.5, -0.5, 0.0, 0.0, -1.0, 1.0, 1.0, //
        0.5, 0.5, -0.5, 0.0, 0.0, -1.0, 1.0, 1.0, //
        -0.5, 0.5, -0.5, 0.0, 0.0, -1.0, 0.0, 1.0, //
        -0.5, -0.5, -0.5, 0.0, 0.0, -1.0, 0.0, 0.0, //
        -0.5, -0.5, 0.5, 0.0, 0.0, 1.0, 0.0, 0.0, //
        0.5, -0.5, 0.5, 0.0, 0.0, 1.0, 1.0, 0.0, //
        0.5, 0.5, 0.5, 0.0, 0.0, 1.0, 1.0, 1.0, //
        0.5, 0.5, 0.5, 0.0, 0.0, 1.0, 1.0, 1.0, //
        -0.5, 0.5, 0.5, 0.0, 0.0, 1.0, 0.0, 1.0, //
        -0.5, -0.5, 0.5, 0.0, 0.0, 1.0, 0.0, 0.0, //
        -0.5, 0.5, 0.5, -1.0, 0.0, 0.0, 1.0, 0.0, //
        -0.5, 0.5, -0.5, -1.0, 0.0, 0.0, 1.0, 1.0, //
        -0.5, -0.5, -0.5, -1.0, 0.0, 0.0, 0.0, 1.0, //
        -0.5, -0.5, -0.5, -1.0, 0.0, 0.0, 0.0, 1.0, //
        -0.5, -0.5, 0.5, -1.0, 0.0, 0.0, 0.0, 0.0, //
        -0.5, 0.5, 0.5, -1.0, 0.0, 0.0, 1.0, 0.0, //
        0.5, 0.5, 0.5, 1.0, 0.0, 0.0, 1.0, 0.0, //
        0.5, 0.5, -0.5, 1.0, 0.0, 0.0, 1.0, 1.0, //
        0.5, -0.5, -0.5, 1.0, 0.0, 0.0, 0.0, 1.0, //
        0.5, -0.5, -0.5, 1.0, 0.0, 0.0, 0.0, 1.0, //
        0.5, -0.5, 0.5, 1.0, 0.0, 0.0, 0.0, 0.0, //
        0.5, 0.5, 0.5, 1.0, 0.0, 0.0, 1.0, 0.0, //
        -0.5, -0.5, -0.5, 0.0, -1.0, 0.0, 0.0, 1.0, //
        0.5, -0.5, -0.5, 0.0, -1.0, 0.0, 1.0, 1.0, //
        0.5, -0.5, 0.5, 0.0, -1.0, 0.0, 1.0, 0.0, //
        0.5, -0.5, 0.5, 0.0, -1.0, 0.0, 1.0, 0.0, //
        -0.5, -0.5, 0.5, 0.0, -1.0, 0.0, 0.0, 0.0, //
        -0.5, -0.5, -0.5, 0.0, -1.0, 0.0, 0.0, 1.0, //
        -0.5, 0.5, -0.5, 0.0, 1.0, 0.0, 0.0, 1.0, //
        0.5, 0.5, -0.5, 0.0, 1.0, 0.0, 1.0, 1.0, //
        0.5, 0.5, 0.5, 0.0, 1.0, 0.0, 1.0, 0.0, //
        0.5, 0.5, 0.5, 0.0, 1.0, 0.0, 1.0, 0.0, //
        -0.5, 0.5, 0.5, 0.0, 1.0, 0.0, 0.0, 0.0, //
        -0.5, 0.5, -0.5, 0.0, 1.0, 0.0, 0.0, 1.0, //
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
    let main_stride = (8 * std::mem::size_of::<GLfloat>()) as GLsizei;
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
        // texcoord attribute
        gl::VertexAttribPointer(
            2,
            2,
            gl::FLOAT,
            gl::FALSE,
            main_stride,
            (6 * std::mem::size_of::<GLfloat>()) as *const GLvoid,
        );
        gl::EnableVertexAttribArray(2);
    }

    let mut camera = Camera {
        pos: glm::vec3(0.0, 0.0, 3.0),
        front: glm::vec3(0.0, 0.0, -1.0),
        up: glm::vec3(0.0, 1.0, 0.0),
        right: glm::vec3(0.0, 0.0, 0.0),
        speed_factor: 2.0,
        fov_y: 45.0,
        fov_y_min: 1.0,
        fov_y_max: 90.0,
        speed: 0.0,
        yaw: -90.0,
        pitch: 0.0,
        aim_sensitivity: 0.03,
    };

    // shaders
    let vs = Shader::new(include_str!("shaders/phong.vs"), gl::VERTEX_SHADER);
    let fs = Shader::new(include_str!("shaders/phong.fs"), gl::FRAGMENT_SHADER);
    let phong_program = Program::new(vs, fs);

    let vs = Shader::new(include_str!("shaders/light.vs"), gl::VERTEX_SHADER);
    let fs = Shader::new(include_str!("shaders/light.fs"), gl::FRAGMENT_SHADER);
    let light_program = Program::new(vs, fs);

    // TODO ui
    let vs = Shader::new(include_str!("shaders/ui.vs"), gl::VERTEX_SHADER);
    let fs = Shader::new(include_str!("shaders/ui.fs"), gl::FRAGMENT_SHADER);
    let _ui_program = Program::new(vs, fs);

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
    let light_stride = (8 * std::mem::size_of::<GLfloat>()) as GLsizei;
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

    let tex_assets_path = format!("{}{}", env!("CARGO_MANIFEST_DIR"), "/assets/textures/");

    let material = Material {
        diffuse: Texture::new(format!("{}crate_diffuse.jpg", tex_assets_path)),
        specular: Texture::new(format!("{}crate_specular.jpg", tex_assets_path)),
        specular_strength: 32.0,
    };

    material.diffuse.bind(0);
    material.specular.bind(1);

    let cube_positions: [Vec3; 10] = [
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(2.0, 5.0, -15.0),
        Vec3::new(-1.5, -2.2, -2.5),
        Vec3::new(-3.8, -2.0, -12.3),
        Vec3::new(2.4, -0.4, -3.5),
        Vec3::new(-1.7, 3.0, -7.5),
        Vec3::new(1.3, -2.0, -2.5),
        Vec3::new(1.5, 2.0, -2.5),
        Vec3::new(1.5, 0.2, -1.5),
        Vec3::new(-1.3, 1.0, -1.5),
    ];

    let pointlight_positions: [Vec3; 4] = [
        Vec3::new(0.7, 0.2, 2.0),
        Vec3::new(2.3, -3.3, -4.0),
        Vec3::new(-4.0, 2.0, -12.0),
        Vec3::new(0.0, 0.0, -3.0),
    ];

    let spotlight = SpotLight {
        pos: Vec3::new(1.2, 1.0, 2.0),
        dir: Vec3::new(-1.2, -2.0, -0.3),
        cut_off: glm::cos(glm::radians(45.0)),
        outer_cut_off: glm::cos(glm::radians(60.0)),

        constant: 1.0,
        linear: 0.09,
        quadratic: 0.032,

        ambient: Vec3::new(0.2, 0.2, 0.2),
        diffuse: Vec3::new(0.5, 0.5, 0.5),
        specular: Vec3::new(1.0, 1.0, 1.0),
    };

    let dirlight = DirLight {
        dir: Vec3::new(-0.2, -1.0, -0.3),
        ambient: Vec3::new(0.05, 0.05, 0.05),
        diffuse: Vec3::new(0.4, 0.4, 0.4),
        specular: Vec3::new(0.5, 0.5, 0.5),
    };

    // let light = DirLight {
    //     dir: Vec3::new(-0.2, -1.0, -0.3),
    //     light: Light {
    //         ambient: Vec3::new(0.05, 0.05, 0.05),
    //         diffuse: Vec3::new(0.4, 0.4, 0.4),
    //         specular: Vec3::new(0.5, 0.5, 0.5),
    //     },
    // };

    const KEY_AMOUNT: usize = glfw::ffi::KEY_LAST as usize;
    let mut key_states = [false; KEY_AMOUNT];

    let mut mouse_last_x = WIN_WIDTH as f64 / 2.0;
    let mut mouse_last_y = WIN_HEIGHT as f64 / 2.0;
    let mut mouse_pos_x = 0.0;
    let mut mouse_pos_y = 0.0;

    let mut mouse_scroll_y = 0.0;

    while !window.should_close() {
        let frame_start_time = glfw.get_time() as f32;
        let delta_time = frame_start_time - last_frame;
        last_frame = frame_start_time;

        let mut mouse_updated = false;
        let mut mouse_scroll_updated = false;

        // SECTION render

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        camera.speed = camera.speed_factor * delta_time;

        phong_program.use_program();

        // TODO Translate - Rotate - Scale matrix manipulations queue to respect order
        let projection =
            glm::ext::perspective(glm::radians(camera.fov_y), WIN_ASPECT_RATIO, 0.1, 100.0);
        let view = glm::ext::look_at(camera.pos, camera.pos + camera.front, camera.up);

        // update local uniform values
        phong_program.set_uniform_mat4("view", &view);
        phong_program.set_uniform_mat4("projection", &projection);

        phong_program.set_uniform_vec3("camera_pos", camera.pos);

        phong_program.set_uniform_int("material.diffuse", 0);
        phong_program.set_uniform_int("material.specular", 1);
        phong_program.set_uniform_float("material.specular_strength", material.specular_strength);

        // Spot light
        phong_program.set_uniform_vec3("spotlight.pos", spotlight.pos);
        phong_program.set_uniform_vec3("spotlight.dir", spotlight.dir);

        phong_program.set_uniform_float("spotlight.cut_off", spotlight.cut_off);
        phong_program.set_uniform_float("spotlight.outer_cut_off", spotlight.outer_cut_off);

        phong_program.set_uniform_float("spotlight.constant", spotlight.constant);
        phong_program.set_uniform_float("spotlight.linear", spotlight.linear);
        phong_program.set_uniform_float("spotlight.quadratic", spotlight.quadratic);

        phong_program.set_uniform_vec3("spotlight.ambient", spotlight.ambient);
        phong_program.set_uniform_vec3("spotlight.diffuse", spotlight.diffuse);
        phong_program.set_uniform_vec3("spotlight.specular", spotlight.specular);

        // Directional light
        phong_program.set_uniform_vec3("dirlight.dir", dirlight.dir);

        phong_program.set_uniform_vec3("dirlight.ambient", dirlight.ambient);
        phong_program.set_uniform_vec3("dirlight.diffuse", dirlight.diffuse);
        phong_program.set_uniform_vec3("dirlight.specular", dirlight.specular);

        // Point lights
        for i in 0..4 {
            let pointlight = PointLight {
                pos: pointlight_positions[i],

                constant: 1.0,
                linear: 0.09,
                quadratic: 0.032,

                ambient: Vec3::new(0.2, 0.2, 0.2),
                diffuse: Vec3::new(0.5, 0.5, 0.5),
                specular: Vec3::new(1.0, 1.0, 1.0),
            };

            phong_program.set_uniform_vec3(&format!("pointlights[{}].pos", i), pointlight.pos);

            phong_program
                .set_uniform_float(&format!("pointlights[{}].constant", i), pointlight.constant);
            phong_program
                .set_uniform_float(&format!("pointlights[{}].linear", i), pointlight.linear);
            phong_program.set_uniform_float(
                &format!("pointlights[{}].quadratic", i),
                pointlight.quadratic,
            );

            phong_program
                .set_uniform_vec3(&format!("pointlights[{}].ambient", i), pointlight.ambient);
            phong_program
                .set_uniform_vec3(&format!("pointlights[{}].diffuse", i), pointlight.diffuse);
            phong_program
                .set_uniform_vec3(&format!("pointlights[{}].specular", i), pointlight.specular);
        }

        for i in 0..10 {
            let mut model = Mat4::new(
                Vec4::new(1.0, 0.0, 0.0, 0.0),
                Vec4::new(0.0, 1.0, 0.0, 0.0),
                Vec4::new(0.0, 0.0, 1.0, 0.0),
                Vec4::new(0.0, 0.0, 0.0, 1.0),
            );
            let angle = 40.0 * frame_start_time + i as f32 * 10.0;
            model = glm::ext::translate(&model, cube_positions[i]);
            model = glm::ext::rotate(&model, glm::radians(angle), Vec3::new(1.0, 0.3, 0.5));
            phong_program.set_uniform_mat4("model", &model);
            unsafe {
                gl::BindVertexArray(vao);
                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }
        }

        // SECTION light source render

        light_program.use_program();

        light_program.set_uniform_mat4("view", &view);
        light_program.set_uniform_mat4("projection", &projection);

        for i in 0..4 {
            let mut model = Mat4::new(
                Vec4::new(1.0, 0.0, 0.0, 0.0),
                Vec4::new(0.0, 1.0, 0.0, 0.0),
                Vec4::new(0.0, 0.0, 1.0, 0.0),
                Vec4::new(0.0, 0.0, 0.0, 1.0),
            );
            let angle = 40.0 * frame_start_time + i as f32 * 10.0;
            model = glm::ext::translate(&model, pointlight_positions[i]);
            model = glm::ext::rotate(&model, glm::radians(angle), Vec3::new(1.0, 0.3, 0.5));
            model = glm::ext::scale(&model, Vec3::new(0.1, 0.1, 0.1));
            light_program.set_uniform_mat4("model", &model);
            unsafe {
                gl::BindVertexArray(vao);
                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }
        }

        // TODO SECTION framerate counter UI

        // SECTION swap buffers & poll events

        window.swap_buffers();
        glfw.poll_events();

        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::CursorPos(x, y) => {
                    mouse_updated = true;
                    mouse_last_x = mouse_pos_x;
                    mouse_last_y = mouse_pos_y;
                    mouse_pos_x = x;
                    mouse_pos_y = y;
                }
                glfw::WindowEvent::Key(key, _, action, _) => {
                    key_states[key as usize] = action != glfw::Action::Release;
                }
                glfw::WindowEvent::Scroll(_x_offset, y_offset) => {
                    mouse_scroll_updated = true;
                    mouse_scroll_y = y_offset;
                }
                glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
                    gl::Viewport(0, 0, width, height);
                },
                _ => {}
            }
        }

        // SECTION keyboard input

        // W move forward
        if key_states[Key::W as usize] {
            camera.pos = camera.pos + (camera.front * camera.speed);
        }
        // A move left
        if key_states[Key::A as usize] {
            camera.pos =
                camera.pos - (glm::normalize(glm::cross(camera.front, camera.up)) * camera.speed);
        }
        // S move back
        if key_states[Key::S as usize] {
            camera.pos = camera.pos - (camera.front * camera.speed);
        }
        // D move right
        if key_states[Key::D as usize] {
            camera.pos =
                camera.pos + (glm::normalize(glm::cross(camera.front, camera.up)) * camera.speed);
        }
        // SPACE move up
        if key_states[Key::Space as usize] {
            camera.pos = camera.pos + (camera.up * camera.speed);
        }
        // LEFT CTRL move down
        if key_states[Key::LeftControl as usize] {
            camera.pos = camera.pos - (camera.up * camera.speed);
        }

        // P cycle through polygon modes
        if key_states[Key::P as usize] {
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
        // ESC close window
        if key_states[Key::Escape as usize] {
            window.set_should_close(true);
        }

        // SECTION mouse input

        // camera aim
        if mouse_updated {
            let x_offset = mouse_pos_x - mouse_last_x;
            let y_offset = mouse_last_y - mouse_pos_y;

            camera.yaw += x_offset as f32 * camera.aim_sensitivity;
            camera.pitch += y_offset as f32 * camera.aim_sensitivity;

            camera.pitch = camera.pitch.clamp(-89.9, 89.9);
            camera.yaw = camera.yaw.rem_euclid(360.0);

            camera.front = glm::normalize(Vec3::new(
                camera.yaw.to_radians().cos() * camera.pitch.to_radians().cos(),
                camera.pitch.to_radians().sin(),
                camera.yaw.to_radians().sin() * camera.pitch.to_radians().cos(),
            ));
        }

        // scroll
        if mouse_scroll_updated {
            camera.fov_y -= mouse_scroll_y as f32;
            camera.fov_y = camera.fov_y.max(camera.fov_y_min).min(camera.fov_y_max);
        }

        // SECTION framerate

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
