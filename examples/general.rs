use cgmath::{
    point3, vec3, vec4, Angle, Deg, EuclideanSpace, InnerSpace, Matrix4, SquareMatrix, Vector3,
};
use gl::types::{GLenum, GLfloat, GLsizei, GLsizeiptr, GLvoid};
use glfw::Context;
use revenant::{
    assets,
    types::{
        Camera, DirLight, Filtering, Font, ImageSize, Material, PointLight, Position, Program,
        ProjectionKind, SpotLight, Texture, TextureKind, Wrapping,
    },
    Revenant,
};

// TODO flexible window size
const WIN_DIM_X: u32 = 1600;
const WIN_DIM_Y: u32 = 900;

// TODO flexible window aspect ratio
const SCREEN_DIM_X: u32 = 1920;
const SCREEN_DIM_Y: u32 = 1080;

const KEY_AMOUNT: usize = glfw::ffi::KEY_LAST as usize;
struct State {
    mouse_pos_x: f64,
    mouse_pos_y: f64,
    speed_factor_default: f32,
    speed_factor_boost: f32,
    speed_factor: f32,
    speed: f32,
    yaw: f32,
    pitch: f32,
    aim_sensitivity: f32,
    fov_y_min: f32,
    fov_y_max: f32,
    last_time: f64,
    current_fps: u32,
    ms_per_frame: f64,
    frame_cycle: u32,
    frame_number: u32,
    last_frame: f64,
    key_states: [bool; KEY_AMOUNT],
}

fn main() {
    optick::start_capture();
    let mut revenant = Revenant::new(WIN_DIM_X, WIN_DIM_Y);
    init_revenant(&mut revenant);
    if false {
        create_assets();
    }
    load_assets(&mut revenant);
    init_gl(&mut revenant);

    let mut states = State {
        mouse_pos_x: 0.0,
        mouse_pos_y: 0.0,
        speed_factor_default: 3.0,
        speed_factor_boost: 6.0,
        speed_factor: 3.0,
        speed: 0.0,
        yaw: 200.0,
        pitch: -20.0,
        aim_sensitivity: 0.03,
        fov_y_min: 30.0,
        fov_y_max: 90.0,
        last_time: revenant.glfw.get_time(),
        frame_cycle: 0,
        current_fps: 0,
        ms_per_frame: 1000.0,
        frame_number: 0,
        last_frame: 0.0,
        key_states: [false; KEY_AMOUNT],
    };

    while !revenant.window.should_close() {
        optick::next_frame();
        input(&mut revenant, &mut states);
        // TODO update_multiplayer(&revenant);how to get screen dimension in rust GLFW
        // TODO update_AI(&revenant);
        // TODO update_physics(&revenant);
        // TODO handle_collisions(&revenant);
        update(&mut revenant);
        render(&mut revenant, &mut states);
        // TODO audio(&revenant);
        // TODO post_process(&revenant);
    }

    cleanup(&revenant);
}

#[inline]
fn init_gl(revenant: &mut Revenant) {
    revenant::set_clear_color(vec4(0.082, 0.082, 0.125, 1.0));
    unsafe {
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }

    for (_, texture) in revenant.assets.textures.iter_mut() {
        texture.gl_register();
    }
    for (_, material) in revenant.assets.materials.iter_mut() {
        material.diffuse.gl_register();
        material.specular.gl_register();
        material.emissive.gl_register();
    }
}

#[inline]
fn init_revenant(revenant: &mut Revenant) {
    // Print OpenGL version
    let version = revenant.window.get_context_version();
    println!("OpenGL version: {}.{}", version.major, version.minor);

    // Set window icon
    let icon_asset = assets::load_foreign_image("icon", "png");
    let mut icon_pixels: Vec<u32> = vec![];
    for chunk in icon_asset.data.chunks_exact(4) {
        let u32_value = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        icon_pixels.push(u32_value);
    }
    let (width, height) = match icon_asset.size {
        ImageSize::I2D { x, y } => (x, y),
        _ => panic!("Icon size is not 2D."),
    };
    let mut icons = Vec::new();
    icons.push(glfw::PixelImage {
        width: width as u32,
        height: height as u32,
        pixels: icon_pixels,
    });
    revenant.window.set_icon_from_pixels(icons);

    // Unlock framerate
    revenant.glfw.set_swap_interval(glfw::SwapInterval::None);

    // Center window
    revenant.window.set_pos(
        (SCREEN_DIM_X - WIN_DIM_X) as i32 / 2,
        (SCREEN_DIM_Y - WIN_DIM_Y) as i32 / 2,
    );
}

#[inline]
fn create_assets() {
    let image_crate_diffuse = assets::load_foreign_image("crate_diffuse", "jpg");
    let image_crate_specular = assets::load_foreign_image("crate_specular", "jpg");
    let image_crate_emissive = assets::load_foreign_image("crate_emissive", "jpg");
    let texture_crate_diffuse = Texture {
        gl_id: 0,
        image: assets::load("image_crate_diffuse"),
        kind: TextureKind::Diffuse,
        s_wrapping: Wrapping::Repeat,
        t_wrapping: Wrapping::Repeat,
        min_filtering: Filtering::Nearest,
        mag_filtering: Filtering::Nearest,
        mipmapping: true,
    };
    let texture_crate_specular = Texture {
        gl_id: 0,
        image: assets::load("image_crate_specular"),
        kind: TextureKind::Specular,
        s_wrapping: Wrapping::Repeat,
        t_wrapping: Wrapping::Repeat,
        min_filtering: Filtering::Nearest,
        mag_filtering: Filtering::Nearest,
        mipmapping: true,
    };
    let texture_crate_emissive = Texture {
        gl_id: 0,
        image: assets::load("image_crate_emissive"),
        kind: TextureKind::Emissive,
        s_wrapping: Wrapping::Repeat,
        t_wrapping: Wrapping::Repeat,
        min_filtering: Filtering::Nearest,
        mag_filtering: Filtering::Nearest,
        mipmapping: true,
    };

    let material_crate = Material {
        diffuse: assets::load("texture_crate_diffuse"),
        specular: assets::load("texture_crate_specular"),
        specular_strength: 32.0,
        emissive: assets::load("texture_crate_emissive"),
    };

    let camera_main = Camera {
        pos: point3(1.84, 0.8, 3.1),
        front: vec3(0.0, 0.0, -1.0),
        up: vec3(0.0, 1.0, 0.0),
        right: vec3(0.0, 0.0, 0.0),

        update_projection: true,
        projection_kind: ProjectionKind::Perspective {
            aspect_ratio: 16.0 / 9.0, // TODO get from window size
            fov_y: 45.0,
            near: 0.1,
            far: 100.0,
        },
        projection: Matrix4::identity(),
    };
    let camera_ui = Camera {
        pos: point3(0.0, 0.0, 0.0),
        front: vec3(0.0, 0.0, -1.0),
        up: vec3(0.0, 1.0, 0.0),
        right: vec3(0.0, 0.0, 0.0),

        update_projection: true,
        projection_kind: ProjectionKind::Orthographic {
            left: -1.0,
            right: 1.0,
            bottom: -1.0,
            top: 1.0,
            near: -1.0,
            far: 1.0,
        },
        projection: Matrix4::identity(),
    };

    let spotlight = SpotLight {
        pos: point3(1.2, 1.0, 2.0),
        dir: vec3(-1.2, -2.0, -0.3),
        cut_off: Angle::cos(Deg(12.5)),
        outer_cut_off: Angle::cos(Deg(60.0)),

        constant: 1.0,
        linear: 0.09,
        quadratic: 0.032,

        ambient: vec3(0.2, 0.2, 0.2),
        diffuse: vec3(0.5, 0.5, 0.5),
        specular: vec3(1.0, 1.0, 1.0),
    };
    let pointlight = PointLight {
        pos: point3(0.7, 0.2, 2.0),

        constant: 1.0,
        linear: 0.09,
        quadratic: 0.032,

        ambient: vec3(0.2, 0.2, 0.2),
        diffuse: vec3(0.5, 0.5, 0.5),
        specular: vec3(1.0, 1.0, 1.0),
    };
    let dirlight = DirLight {
        dir: vec3(-0.2, -1.0, -0.3),
        ambient: vec3(0.05, 0.05, 0.05),
        diffuse: vec3(0.4, 0.4, 0.4),
        specular: vec3(0.5, 0.5, 0.5),
    };
    assets::save("image_crate_diffuse", image_crate_diffuse);
    assets::save("image_crate_specular", image_crate_specular);
    assets::save("image_crate_emissive", image_crate_emissive);
    assets::save("texture_crate_diffuse", texture_crate_diffuse);
    assets::save("texture_crate_specular", texture_crate_specular);
    assets::save("texture_crate_emissive", texture_crate_emissive);
    assets::save("material_crate", material_crate);
    assets::save("camera_main", camera_main);
    assets::save("camera_ui", camera_ui);
    assets::save("spotlight", spotlight);
    assets::save("pointlight", pointlight);
    assets::save("dirlight", dirlight);
}

#[inline]
fn load_assets(revenant: &mut Revenant) {
    // camera
    let camera_ui: Camera = assets::load("camera_ui");
    let camera_main: Camera = assets::load("camera_main");

    // shaders
    let phong_vs = assets::load_foreign_shader("phong", "vs");
    let phong_fs = assets::load_foreign_shader("phong", "fs");

    let light_vs = assets::load_foreign_shader("light", "vs");
    let light_fs = assets::load_foreign_shader("light", "fs");

    let ui_vs = assets::load_foreign_shader("ui", "vs");
    let ui_fs = assets::load_foreign_shader("ui", "fs");

    // programs
    let phong = Program::new(phong_vs, phong_fs);
    let light = Program::new(light_vs, light_fs);
    let ui = Program::new(ui_vs, ui_fs);

    ui.use_program();
    ui.set_uniform_mat4("projection", &camera_ui.projection);

    revenant
        .assets
        .programs
        .insert("phong_program".to_string(), phong);
    revenant
        .assets
        .programs
        .insert("light_program".to_string(), light);
    revenant
        .assets
        .programs
        .insert("ui_program".to_string(), ui);
    revenant
        .assets
        .cameras
        .insert("camera_main".to_string(), camera_main);
    revenant
        .assets
        .cameras
        .insert("camera_ui".to_string(), camera_ui);

    // fonts
    let comfortaa_font = assets::load_foreign_font("comfortaa", "ttf");
    let teko_font = assets::load_foreign_font("teko", "ttf");
    revenant
        .assets
        .fonts
        .insert("comfortaa_font".to_string(), comfortaa_font);
    revenant
        .assets
        .fonts
        .insert("teko_font".to_string(), teko_font);

    // material
    let material: Material = assets::load("material_crate");
    revenant
        .assets
        .materials
        .insert("material_crate".to_string(), material);

    // lights
    let spotlight: SpotLight = assets::load("spotlight");
    let pointlight: PointLight = assets::load("pointlight");
    let dirlight: DirLight = assets::load("dirlight");

    revenant
        .assets
        .spotlights
        .insert("spotlight".to_string(), spotlight);
    revenant
        .assets
        .pointlights
        .insert("pointlight".to_string(), pointlight);
    revenant
        .assets
        .dirlights
        .insert("dirlight".to_string(), dirlight);

    // GLTF
    // // TODO 3d models & more
    // let binding = assets_path().join("models").join("tree_cam_light.glb");
    // let blend_path = binding.to_str().expect("Failed to convert path to string");
    // let gltf = Gltf::open(blend_path).expect("Failed to open gltf file");
    // let scenes = gltf.scenes();
    // for scene in scenes {
    //     println!("Scene #{} has {} nodes", scene.index(), scene.nodes().len());
    //     for node in scene.nodes() {
    //         let cameras = node.camera();
    //         if let Some(camera) = cameras {
    //             match camera.projection() {
    //                 gltf::camera::Projection::Orthographic(ortho) => {
    //                     println!("Orthographic camera");
    //                     println!("xmag: {}", ortho.xmag());
    //                     println!("ymag: {}", ortho.ymag());
    //                     println!("znear: {}", ortho.znear());
    //                     println!("zfar: {}", ortho.zfar());
    //                     println!("extras: {:?}", ortho.extras());
    //                 }
    //                 gltf::camera::Projection::Perspective(persp) => {
    //                     println!("Perspective camera");
    //                     println!("aspect_ratio: {:?}", persp.aspect_ratio());
    //                     println!("yfov: {}", persp.yfov());
    //                     println!("znear: {}", persp.znear());
    //                     println!("zfar: {:?}", persp.zfar());
    //                     println!("extras: {:?}", persp.extras());
    //                 }
    //             }
    //         }
    //     }
    // }
}

// TODO create struct for input handling & keep track of assigned keys
#[inline]
fn input(revenant: &mut Revenant, states: &mut State) {
    optick::event!();
    revenant.glfw.poll_events();

    let mut main_camera = revenant
        .assets
        .cameras
        .get_mut("camera_main") // TODO get mut might be an optimization issue -> to verify
        .expect("Failed to get main camera");

    for (_, event) in glfw::flush_messages(&revenant.events) {
        match event {
            glfw::WindowEvent::CursorPos(x, y) => {
                let mouse_last_x = states.mouse_pos_x;
                let mouse_last_y = states.mouse_pos_y;
                states.mouse_pos_x = x;
                states.mouse_pos_y = y;

                let x_offset = states.mouse_pos_x - mouse_last_x;
                let y_offset = mouse_last_y - states.mouse_pos_y;

                states.yaw += x_offset as f32 * states.aim_sensitivity;
                states.pitch += y_offset as f32 * states.aim_sensitivity;

                states.pitch = states.pitch.clamp(-89.9, 89.9); // FIX use quaternions
                states.yaw = states.yaw.rem_euclid(360.0);

                main_camera.front = vec3(
                    states.pitch.to_radians().cos() * states.yaw.to_radians().cos(),
                    states.pitch.to_radians().sin(),
                    states.pitch.to_radians().cos() * states.yaw.to_radians().sin(),
                )
                .normalize();
                main_camera.right = main_camera.front.cross(main_camera.up);
            }
            glfw::WindowEvent::Key(key, _, action, _) => {
                states.key_states[key as usize] = action != glfw::Action::Release;
            }
            glfw::WindowEvent::Scroll(_x_offset, y_offset) => {
                let mouse_scroll_y = y_offset;

                match main_camera.projection_kind {
                    ProjectionKind::Perspective {
                        fov_y,
                        near,
                        far,
                        aspect_ratio,
                    } => {
                        let mut fov_y = fov_y - mouse_scroll_y as f32;
                        fov_y = fov_y.clamp(states.fov_y_min, states.fov_y_max);
                        main_camera.projection_kind = ProjectionKind::Perspective {
                            fov_y,
                            near,
                            far,
                            aspect_ratio,
                        };
                    }
                    ProjectionKind::Orthographic {
                        left: _,
                        right: _,
                        bottom: _,
                        top: _,
                        near: _,
                        far: _,
                    } => {
                        // TODO implement
                    }
                };
            }
            glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
                gl::Viewport(0, 0, width, height);
            },
            _ => {}
        }
    }

    // SECTION keyboard input

    // TODO make a hotkey manager
    // W move forward
    if states.key_states[glfw::Key::W as usize] {
        main_camera.pos += main_camera.front * states.speed;
    }
    // S move backward
    if states.key_states[glfw::Key::S as usize] {
        main_camera.pos -= main_camera.front * states.speed;
    }
    // A move left
    if states.key_states[glfw::Key::A as usize] {
        main_camera.pos -= main_camera.right * states.speed;
    }
    // D move right
    if states.key_states[glfw::Key::D as usize] {
        main_camera.pos += main_camera.right * states.speed;
    }
    // SPACE move up
    if states.key_states[glfw::Key::Space as usize] {
        main_camera.pos += main_camera.up * states.speed;
    }
    // LEFT CTRL move down
    if states.key_states[glfw::Key::LeftControl as usize] {
        main_camera.pos -= main_camera.up * states.speed;
    }
    // LEFT SHIFT increase speed
    if states.key_states[glfw::Key::LeftShift as usize] {
        states.speed_factor = states.speed_factor_boost;
    } else {
        states.speed_factor = states.speed_factor_default;
    }

    // P cycle through polygon modes
    if states.key_states[glfw::Key::P as usize] {
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
    if states.key_states[glfw::Key::Escape as usize] {
        revenant.window.set_should_close(true);
    }
}

#[inline]
fn update(revenant: &mut Revenant) {
    // TODO optimize .update(); iteration to use a custom HashMap / a bitset for assets to be updated
    for (_, camera) in revenant.assets.cameras.iter_mut() {
        if camera.update_projection {
            camera.update();
        }
    }
}

#[inline]
fn render(revenant: &mut Revenant, states: &mut State) {
    let light_program = revenant
        .assets
        .programs
        .get("light_program")
        .expect("Failed to get light_program");

    let ui_program = revenant
        .assets
        .programs
        .get("ui_program")
        .expect("Failed to get ui_program");

    let comfortaa_font = revenant
        .assets
        .fonts
        .get("comfortaa_font")
        .expect("Failed to get comfortaa");

    let teko_font = revenant
        .assets
        .fonts
        .get("comfortaa_font")
        .expect("Failed to get teko");

    const POINTLIGHT_POSITION: [Position; 4] = [
        point3(0.7, 0.2, 2.0),
        point3(2.3, -3.3, -4.0),
        point3(-4.0, 2.0, -12.0),
        point3(0.0, 0.0, -3.0),
    ];

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

    let mut ui_vbo = 0;
    let mut ui_vao = 0;

    unsafe {
        gl::GenVertexArrays(1, &mut ui_vao);
        gl::GenBuffers(1, &mut ui_vbo);
        gl::BindVertexArray(ui_vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, ui_vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            6 * 4 * std::mem::size_of::<GLfloat>() as GLsizeiptr,
            std::ptr::null(),
            gl::DYNAMIC_DRAW,
        );

        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            4,
            gl::FLOAT,
            gl::FALSE,
            4 * std::mem::size_of::<GLfloat>() as GLsizei,
            std::ptr::null(),
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
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

    let cube_positions: [Vector3<f32>; 10] = [
        vec3(0.0, 0.0, 0.0),
        vec3(2.0, 5.0, -15.0),
        vec3(-1.5, -2.2, -2.5),
        vec3(-3.8, -2.0, -12.3),
        vec3(2.4, -0.4, -3.5),
        vec3(-1.7, 3.0, -7.5),
        vec3(1.3, -2.0, -2.5),
        vec3(1.5, 2.0, -2.5),
        vec3(1.5, 0.2, -1.5),
        vec3(-1.3, 1.0, -1.5),
    ];

    let camera_main = revenant
        .assets
        .cameras
        .get("camera_main")
        .expect("Camera not found");
    let spotlight = revenant
        .assets
        .spotlights
        .get("spotlight")
        .expect("Spotlight not found");
    let dirlight = revenant
        .assets
        .dirlights
        .get("dirlight")
        .expect("Directional light not found");

    let frame_start_time = revenant.glfw.get_time();
    let delta_time = frame_start_time - states.last_frame;
    states.last_frame = frame_start_time;

    // SECTION retrieve frame assets

    let material = revenant
        .assets
        .materials
        .get("material_crate")
        .expect("Material not found");
    let phong_program = revenant
        .assets
        .programs
        .get("phong_program")
        .expect("Phong program not found");

    // SECTION phong render

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }

    states.speed = states.speed_factor * delta_time as f32;

    phong_program.use_program();

    material.diffuse.bind(0);
    material.specular.bind(1);
    material.emissive.bind(2);

    // TODO Translate - Rotate - Scale matrix manipulations queue to respect order
    // FIX rework
    let view = Matrix4::look_at_rh(
        camera_main.pos,
        camera_main.pos + camera_main.front,
        camera_main.up,
    );

    // update local uniform values
    phong_program.set_uniform_mat4("view", &view);
    phong_program.set_uniform_mat4("projection", &camera_main.projection);

    phong_program.set_uniform_point3("camera_pos", camera_main.pos);

    phong_program.set_uniform_int("material.diffuse", 0);
    phong_program.set_uniform_int("material.specular", 1);
    phong_program.set_uniform_float("material.specular_strength", material.specular_strength);
    phong_program.set_uniform_int("material.emissive", 2);

    // Spot light
    phong_program.set_uniform_point3("spotlight.pos", spotlight.pos);
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
            pos: POINTLIGHT_POSITION[i],

            constant: 1.0,
            linear: 0.09,
            quadratic: 0.032,

            ambient: vec3(0.2, 0.2, 0.2),
            diffuse: vec3(0.5, 0.5, 0.5),
            specular: vec3(1.0, 1.0, 1.0),
        };

        phong_program.set_uniform_point3(&format!("pointlights[{}].pos", i), pointlight.pos);

        phong_program
            .set_uniform_float(&format!("pointlights[{}].constant", i), pointlight.constant);
        phong_program.set_uniform_float(&format!("pointlights[{}].linear", i), pointlight.linear);
        phong_program.set_uniform_float(
            &format!("pointlights[{}].quadratic", i),
            pointlight.quadratic,
        );

        phong_program.set_uniform_vec3(&format!("pointlights[{}].ambient", i), pointlight.ambient);
        phong_program.set_uniform_vec3(&format!("pointlights[{}].diffuse", i), pointlight.diffuse);
        phong_program
            .set_uniform_vec3(&format!("pointlights[{}].specular", i), pointlight.specular);
    }

    for i in 0..10 {
        let mut model = Matrix4::identity();
        let angle = 40.0 * frame_start_time as f32;
        model = model * Matrix4::from_translation(cube_positions[i]);
        model =
            model * Matrix4::from_axis_angle(vec3(1.0, 0.3, 0.5).normalize(), cgmath::Deg(angle));
        phong_program.set_uniform_mat4("model", &model);

        unsafe {
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }
    }

    // SECTION light source render

    light_program.use_program();

    light_program.set_uniform_mat4("view", &view);
    light_program.set_uniform_mat4("projection", &camera_main.projection);
    for i in 0..4 {
        let mut model = Matrix4::identity();
        let angle = 40.0 * frame_start_time as f32;
        model = model * Matrix4::from_translation(POINTLIGHT_POSITION[i].to_vec());
        model =
            model * Matrix4::from_axis_angle(vec3(1.0, 0.3, 0.5).normalize(), cgmath::Deg(angle));
        model = model * Matrix4::from_scale(0.1);
        light_program.set_uniform_mat4("model", &model);
        unsafe {
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }
    }

    // SECTION text render

    unsafe {
        gl::Disable(gl::DEPTH_TEST);
    }

    let color = vec3(0.8, 0.8, 0.67);
    let scale = 1.0;

    render_text(
        format!("{} FPS", states.current_fps),
        40.0,
        600.0,
        scale,
        &color,
        &ui_program,
        &comfortaa_font,
        &ui_vao,
        &ui_vbo,
    );

    render_text(
        format!("{:0.4} MS/FRAME", states.ms_per_frame),
        40.0,
        570.0,
        scale,
        &color,
        &ui_program,
        &teko_font,
        &ui_vao,
        &ui_vbo,
    );

    render_text(
        format!(
            "Camera.pos x{:0.2} y{:0.2} z{:0.2}",
            camera_main.pos.x, camera_main.pos.y, camera_main.pos.z
        ),
        20.0,
        20.0,
        scale,
        &color,
        &ui_program,
        &teko_font,
        &ui_vao,
        &ui_vbo,
    );

    render_text(
        format!(
            "Camera.yaw {:0.2} Camera.pitch {:0.2}",
            states.yaw, states.pitch
        ),
        20.0,
        50.0,
        scale,
        &color,
        &ui_program,
        &teko_font,
        &ui_vao,
        &ui_vbo,
    );

    // SECTION swap buffers

    revenant.window.swap_buffers();

    // SECTION frame end

    states.frame_number += 1;
    states.frame_cycle += 1;
    let current_time = revenant.glfw.get_time();
    if current_time - states.last_time >= 1.0 {
        states.current_fps = states.frame_cycle;
        states.ms_per_frame = 1000.0 / states.frame_cycle as f64;
        println!(
            "{} fps {:0.4} ms/draw",
            states.current_fps, states.ms_per_frame
        );
        states.frame_cycle = 0;
        states.last_time = current_time;
    }

    optick::event!("frame_end");
    optick::tag!("frame", states.frame_number);
}

#[inline]
// TODO remove arguments except revenant instance as argument
fn cleanup(_revenant: &Revenant) {
    optick::event!();
    // TODO automatically cleanup all resources (e.g. VAOs, VBOs, shaders, etc.)
    // unsafe {
    // TODO delete all current buffers
    // gl::DeleteVertexArrays(1, &vao);
    // gl::DeleteBuffers(1, &ui_vbo);

    // TODO delete all current programs
    // gl::DeleteProgram(ui_program.gl_id);
    // gl::DeleteProgram(light_program.gl_id);
    // gl::DeleteProgram(phong_program.gl_id);
    // }
    optick::stop_capture("target/revenant");
}

fn render_text(
    text: String,
    x: f32,
    y: f32,
    scale: f32,
    color: &Vector3<f32>,
    program: &Program,
    font: &Font,
    vao: &u32,
    vbo: &u32,
) {
    // Activate the program and bind the texture
    program.use_program();
    program.set_uniform_vec3("color", *color);
    unsafe {
        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindVertexArray(*vao);
    }

    let mut x_pos = x;
    let y_pos = y - font.line_height as f32 * scale;
    for c in text.chars() {
        let glyph = match font.glyphs.get(&c) {
            Some(glyph) => glyph,
            None => continue,
        };

        let w = glyph.width as f32 * scale;
        let h = glyph.height as f32 * scale;

        // Calculate the glyph's texture coordinates
        let x_tex = glyph.sprite_x as f32 / font.width as f32;
        let y_tex = glyph.sprite_y as f32 / font.height as f32;
        let w_tex = glyph.width as f32 / font.width as f32;
        let h_tex = glyph.height as f32 / font.height as f32;

        // Update the vertex buffer with the glyph's quad
        let vertices: [GLfloat; 24] = [
            x_pos,
            y_pos + h,
            x_tex,
            y_tex + h_tex,
            x_pos,
            y_pos,
            x_tex,
            y_tex,
            x_pos + w,
            y_pos,
            x_tex + w_tex,
            y_tex,
            x_pos,
            y_pos + h,
            x_tex,
            y_tex + h_tex,
            x_pos + w,
            y_pos,
            x_tex + w_tex,
            y_tex,
            x_pos + w,
            y_pos + h,
            x_tex + w_tex,
            y_tex + h_tex,
        ];
        unsafe {
            // update content of VBO memory
            gl::BindBuffer(gl::ARRAY_BUFFER, *vbo);
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                0,
                (vertices.len() * std::mem::size_of::<GLfloat>()) as isize,
                vertices.as_ptr() as *const GLvoid,
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            // render quad
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
        }
        // Advance the x position to the next glyph
        x_pos += glyph.advance_x as f32 * scale;
    }
}
