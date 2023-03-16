use cgmath::{
    ortho, point3, vec3, vec4, Angle, Deg, EuclideanSpace, InnerSpace, Matrix4, SquareMatrix,
    Vector3,
};
use gl::types::{GLenum, GLfloat, GLsizei, GLsizeiptr, GLvoid};
use glfw::Context;
use gltf::Gltf;
use revenant::{
    assets_path,
    types::{
        AssetCamera, AssetFont, AssetImage, AssetMaterial, AssetTexture, DirLight, Filtering,
        PointLight, Position, Program, ProjectionKind, Shader, SpotLight, TextureKind, TextureSize,
        Wrapping,
    },
    Revenant,
};

const WIN_DIM_X: u32 = 1600;
const WIN_DIM_Y: u32 = 900;
const WIN_RATIO_X: f32 = WIN_DIM_X as f32 / WIN_DIM_Y as f32;

// TODO flexible window aspect ratio
const SCREEN_DIM_X: u32 = 1920;
const SCREEN_DIM_Y: u32 = 1080;

#[test]
fn main() {
    // Create assets
    let diffuse = AssetTexture {
        name: "crate_diffuse".to_owned(),
        gl_id: 0,
        image: AssetImage::load("crate_diffuse.jpg"),
        kind: TextureKind::Diffuse,
        s_wrapping: Wrapping::Repeat,
        t_wrapping: Wrapping::Repeat,
        min_filtering: Filtering::Nearest,
        mag_filtering: Filtering::Nearest,
        mipmapping: true,
    };

    let specular = AssetTexture {
        name: "crate_specular".to_owned(),
        gl_id: 0,
        image: AssetImage::load("crate_specular.jpg"),
        kind: TextureKind::Specular,
        s_wrapping: Wrapping::Repeat,
        t_wrapping: Wrapping::Repeat,
        min_filtering: Filtering::Nearest,
        mag_filtering: Filtering::Nearest,
        mipmapping: true,
    };

    let emissive = AssetTexture {
        name: "crate_emissive".to_owned(),
        gl_id: 0,
        image: AssetImage::load("crate_emissive.jpg"),
        kind: TextureKind::Emissive,
        s_wrapping: Wrapping::Repeat,
        t_wrapping: Wrapping::Repeat,
        min_filtering: Filtering::Nearest,
        mag_filtering: Filtering::Nearest,
        mipmapping: true,
    };

    AssetMaterial {
        name: "crate".to_owned(),
        diffuse,
        specular,
        specular_strength: 32.0,
        emissive,
    }
    .save();

    let diffuse = AssetTexture {
        name: "crate_diffuse".to_owned(),
        gl_id: 0,
        image: AssetImage::load("crate_diffuse.jpg"),
        kind: TextureKind::Diffuse,
        s_wrapping: Wrapping::Repeat,
        t_wrapping: Wrapping::Repeat,
        min_filtering: Filtering::Nearest,
        mag_filtering: Filtering::Nearest,
        mipmapping: true,
    };

    let specular = AssetTexture {
        name: "crate_specular".to_owned(),
        gl_id: 0,
        image: AssetImage::load("crate_specular.jpg"),
        kind: TextureKind::Specular,
        s_wrapping: Wrapping::Repeat,
        t_wrapping: Wrapping::Repeat,
        min_filtering: Filtering::Nearest,
        mag_filtering: Filtering::Nearest,
        mipmapping: true,
    };

    let emissive = AssetTexture {
        name: "crate_emissive".to_owned(),
        gl_id: 0,
        image: AssetImage::load("crate_emissive.jpg"),
        kind: TextureKind::Emissive,
        s_wrapping: Wrapping::Repeat,
        t_wrapping: Wrapping::Repeat,
        min_filtering: Filtering::Nearest,
        mag_filtering: Filtering::Nearest,
        mipmapping: true,
    };

    diffuse.save();
    specular.save();
    emissive.save();

    AssetCamera {
        name: "main".to_owned(),
        pos: point3(1.84, 0.8, 3.1),
        front: vec3(0.0, 0.0, -1.0),
        up: vec3(0.0, 1.0, 0.0),
        right: vec3(0.0, 0.0, 0.0),

        update_projection: true,
        projection_kind: ProjectionKind::Perspective {
            aspect_ratio: WIN_RATIO_X,
            fov_y: 45.0,
            near: 0.1,
            far: 100.0,
        },
        projection: Matrix4::identity(),
    }
    .save();

    let mut revenant = Revenant::new(WIN_DIM_X, WIN_DIM_Y);

    // Print OpenGL version
    let version = revenant.window.get_context_version();
    println!("OpenGL version: {}.{}", version.major, version.minor);

    // Set window icon
    let icon_filename = "icon.png";
    let icon_asset = AssetImage::load(&icon_filename);
    let mut icon_pixels: Vec<u32> = vec![];
    for chunk in icon_asset.data.chunks_exact(4) {
        let u32_value = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        icon_pixels.push(u32_value);
    }

    let (width, height) = match icon_asset.size {
        TextureSize::TwoD { width, height } => (width, height),
        _ => panic!("Icon size is not 2D"),
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

    // FIX
    // GLTF loading
    let binding = assets_path().join("models").join("pot_light_cam.gltf");
    // let binding = asset_manager.assets_path.join("models/tree.glb");
    let blend_path = binding.to_str().expect("Failed to convert path to string");

    let gltf = Gltf::open(blend_path).expect("Failed to open gltf file");
    // get the first camera in the scene

    let scenes = gltf.scenes();

    for scene in scenes {
        println!("Scene #{} has {} nodes", scene.index(), scene.nodes().len());
        for node in scene.nodes() {
            let cameras = node.camera();
            if let Some(camera) = cameras {
                match camera.projection() {
                    gltf::camera::Projection::Orthographic(ortho) => {
                        println!("Orthographic camera");
                        println!("xmag: {}", ortho.xmag());
                        println!("ymag: {}", ortho.ymag());
                        println!("znear: {}", ortho.znear());
                        println!("zfar: {}", ortho.zfar());
                        println!("extras: {:?}", ortho.extras());
                    }
                    gltf::camera::Projection::Perspective(persp) => {
                        println!("Perspective camera");
                        println!("aspect_ratio: {:?}", persp.aspect_ratio());
                        println!("yfov: {}", persp.yfov());
                        println!("znear: {}", persp.znear());
                        println!("zfar: {:?}", persp.zfar());
                        println!("extras: {:?}", persp.extras());
                    }
                }
            }
            // for child in node.children() {
            //     println!(
            //         "Child #{} has {} children",
            //         child.index(),
            //         child.children().len()
            //     );
            // }
        }
    }

    // Camera
    let main_camera_name = "main".to_owned();
    let mut main_camera = AssetCamera {
        name: main_camera_name.clone(),
        pos: point3(1.84, 0.8, 3.1),
        front: vec3(0.0, 0.0, -1.0),
        up: vec3(0.0, 1.0, 0.0),
        right: vec3(0.0, 0.0, 0.0),

        update_projection: true,
        projection_kind: ProjectionKind::Perspective {
            aspect_ratio: WIN_RATIO_X,
            fov_y: 45.0,
            near: 0.1,
            far: 100.0,
        },
        projection: Matrix4::identity(),
    };
    // let mut main_camera = asset_manager.load_camera_asset(&main_camera_filename);

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

    // shaders
    let vs = Shader::new(include_str!("../src/shaders/phong.vs"), gl::VERTEX_SHADER);
    let fs = Shader::new(include_str!("../src/shaders/phong.fs"), gl::FRAGMENT_SHADER);
    let phong_program = Program::new(vs, fs);

    let vs = Shader::new(include_str!("../src/shaders/light.vs"), gl::VERTEX_SHADER);
    let fs = Shader::new(include_str!("../src/shaders/light.fs"), gl::FRAGMENT_SHADER);
    let light_program = Program::new(vs, fs);

    // TODO ui
    let vs = Shader::new(include_str!("../src/shaders/ui.vs"), gl::VERTEX_SHADER);
    let fs = Shader::new(include_str!("../src/shaders/ui.fs"), gl::FRAGMENT_SHADER);
    let ui_program = Program::new(vs, fs);

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

    revenant::set_clear_color(vec4(0.082, 0.082, 0.125, 1.0));

    unsafe {
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }

    // UI
    ui_program.use_program();
    // TODO Camera UI
    // FIX
    let ui_projection = ortho(0.0, WIN_DIM_X as f32, 0.0, WIN_DIM_Y as f32, -1.0, 1.0);
    ui_program.set_uniform_mat4("projection", &ui_projection);

    // font Comfortaa
    let comfortaa_name = "comfortaa".to_owned();
    let comfortaa_size = 32;
    let comfortaa = AssetFont::load(comfortaa_name, comfortaa_size);

    // font Teko
    let teko_name = "teko".to_owned();
    let teko_size = 22;
    let teko = AssetFont::load(teko_name, teko_size);

    // calculate fps declarations
    let mut last_time = revenant.glfw.get_time();
    let mut frames_rendered = 0;

    // last frame time and delta time
    let mut last_frame = 0.0;
    let mut current_fps = 0.0;
    let mut ms_per_frame = 1000.0;

    let material = AssetMaterial::load("crate");

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

    let pointlight_positions: [Position; 4] = [
        point3(0.7, 0.2, 2.0),
        point3(2.3, -3.3, -4.0),
        point3(-4.0, 2.0, -12.0),
        point3(0.0, 0.0, -3.0),
    ];

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

    let dirlight = DirLight {
        dir: vec3(-0.2, -1.0, -0.3),
        ambient: vec3(0.05, 0.05, 0.05),
        diffuse: vec3(0.4, 0.4, 0.4),
        specular: vec3(0.5, 0.5, 0.5),
    };

    const KEY_AMOUNT: usize = glfw::ffi::KEY_LAST as usize;
    let mut key_states = [false; KEY_AMOUNT];

    let mut mouse_pos_x = 0.0;
    let mut mouse_pos_y = 0.0;

    let speed_factor_default = 3.0;
    let speed_factor_boost = 6.0;
    let mut speed_factor;
    let mut speed = 0.0;
    let mut yaw = 200.0;
    let mut pitch = -20.0;
    let aim_sensitivity = 0.03;
    let fov_y_min = 30.0;
    let fov_y_max = 90.0;

    // SECTION ALIVE LOOP

    while !revenant.window.should_close() {
        let frame_start_time = revenant.glfw.get_time() as f32;
        let delta_time = frame_start_time - last_frame;
        last_frame = frame_start_time;

        // SECTION update variables

        // TODO iterate over all cameras
        main_camera.update();

        // SECTION poll events

        revenant.glfw.poll_events();

        for (_, event) in glfw::flush_messages(&revenant.events) {
            match event {
                glfw::WindowEvent::CursorPos(x, y) => {
                    let mouse_last_x = mouse_pos_x;
                    let mouse_last_y = mouse_pos_y;
                    mouse_pos_x = x;
                    mouse_pos_y = y;

                    let x_offset = mouse_pos_x - mouse_last_x;
                    let y_offset = mouse_last_y - mouse_pos_y;

                    yaw += x_offset as f32 * aim_sensitivity;
                    pitch += y_offset as f32 * aim_sensitivity;

                    pitch = pitch.clamp(-89.9, 89.9); // FIX use quaternions
                    yaw = yaw.rem_euclid(360.0);

                    main_camera.front = vec3(
                        pitch.to_radians().cos() * yaw.to_radians().cos(),
                        pitch.to_radians().sin(),
                        pitch.to_radians().cos() * yaw.to_radians().sin(),
                    )
                    .normalize();
                    main_camera.right = main_camera.front.cross(main_camera.up);
                }
                glfw::WindowEvent::Key(key, _, action, _) => {
                    key_states[key as usize] = action != glfw::Action::Release;
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
                            fov_y = fov_y.clamp(fov_y_min, fov_y_max);
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

        // W move forward
        if key_states[glfw::Key::W as usize] {
            main_camera.pos += main_camera.front * speed;
        }
        // S move backward
        if key_states[glfw::Key::S as usize] {
            main_camera.pos -= main_camera.front * speed;
        }
        // A move left
        if key_states[glfw::Key::A as usize] {
            main_camera.pos -= main_camera.right * speed;
        }
        // D move right
        if key_states[glfw::Key::D as usize] {
            main_camera.pos += main_camera.right * speed;
        }
        // SPACE move up
        if key_states[glfw::Key::Space as usize] {
            main_camera.pos += main_camera.up * speed;
        }
        // LEFT CTRL move down
        if key_states[glfw::Key::LeftControl as usize] {
            main_camera.pos -= main_camera.up * speed;
        }
        // LEFT SHIFT increase speed
        if key_states[glfw::Key::LeftShift as usize] {
            speed_factor = speed_factor_boost;
        } else {
            speed_factor = speed_factor_default;
        }

        // P cycle through polygon modes
        if key_states[glfw::Key::P as usize] {
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
        if key_states[glfw::Key::Escape as usize] {
            revenant.window.set_should_close(true);
        }

        // SECTION phong render

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        speed = speed_factor * delta_time;

        phong_program.use_program();

        material.diffuse.bind(0);
        material.specular.bind(1);
        material.emissive.bind(2);

        // TODO Translate - Rotate - Scale matrix manipulations queue to respect order
        // FIX rework
        let view = Matrix4::look_at_rh(
            main_camera.pos,
            main_camera.pos + main_camera.front,
            main_camera.up,
        );

        // update local uniform values
        phong_program.set_uniform_mat4("view", &view);
        phong_program.set_uniform_mat4("projection", &main_camera.projection);

        phong_program.set_uniform_point3("camera_pos", main_camera.pos);

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
                pos: pointlight_positions[i],

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
            let mut model = Matrix4::identity();
            let angle = 40.0 * frame_start_time + i as f32 * 10.0;
            model = model * Matrix4::from_translation(cube_positions[i]);
            model = model
                * Matrix4::from_axis_angle(vec3(1.0, 0.3, 0.5).normalize(), cgmath::Deg(angle));
            phong_program.set_uniform_mat4("model", &model);

            unsafe {
                gl::BindVertexArray(vao);
                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }
        }

        // SECTION light source render

        light_program.use_program();

        light_program.set_uniform_mat4("view", &view);
        light_program.set_uniform_mat4("projection", &main_camera.projection);
        for i in 0..4 {
            let mut model = Matrix4::identity();
            let angle = 40.0 * frame_start_time + i as f32 * 10.0;
            model = model * Matrix4::from_translation(pointlight_positions[i].to_vec());
            model = model
                * Matrix4::from_axis_angle(vec3(1.0, 0.3, 0.5).normalize(), cgmath::Deg(angle));
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
            format!("{} FPS", current_fps),
            40.0,
            600.0,
            scale,
            &color,
            &ui_program,
            &comfortaa,
            &ui_vao,
            &ui_vbo,
        );

        render_text(
            format!("{:0.4} MS/FRAME", ms_per_frame),
            40.0,
            570.0,
            scale,
            &color,
            &ui_program,
            &teko,
            &ui_vao,
            &ui_vbo,
        );

        render_text(
            format!(
                "Camera.pos x{:0.2} y{:0.2} z{:0.2}",
                main_camera.pos.x, main_camera.pos.y, main_camera.pos.z
            ),
            20.0,
            20.0,
            scale,
            &color,
            &ui_program,
            &teko,
            &ui_vao,
            &ui_vbo,
        );

        render_text(
            format!("Camera.yaw {:0.2} Camera.pitch {:0.2}", yaw, pitch),
            20.0,
            50.0,
            scale,
            &color,
            &ui_program,
            &teko,
            &ui_vao,
            &ui_vbo,
        );

        frames_rendered += 1;
        let current_time = revenant.glfw.get_time();
        if current_time - last_time >= 1.0 {
            current_fps = frames_rendered as f64;
            ms_per_frame = 1000.0 / frames_rendered as f64;
            println!("{} fps {:0.4} ms/draw", current_fps, ms_per_frame);
            frames_rendered = 0;
            last_time = current_time;
        }

        // SECTION swap buffers

        revenant.window.swap_buffers();
    }

    // SECTION cleanup

    unsafe {
        gl::DeleteVertexArrays(1, &vao);
        gl::DeleteBuffers(1, &ui_vbo);

        gl::DeleteProgram(ui_program.id);
        gl::DeleteProgram(light_program.id);
        gl::DeleteProgram(phong_program.id);
    }
}

// TODO performance improvements : Loss of 500 frames per second
// To fix this, we need to use a VAO for each character, and then render all the characters in one draw call.
// This is called "text batching", and can be a real performance improvement.
fn render_text(
    text: String,
    x: f32,
    y: f32,
    scale: f32,
    color: &Vector3<f32>,
    program: &Program,
    font: &AssetFont,
    vao: &u32,
    vbo: &u32,
) {
    // TODO anchor pos
    // TODO render scale
    let mut x = x;

    program.use_program();
    program.set_uniform_vec3("color", *color);
    unsafe {
        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindVertexArray(*vao);
    }

    // iterate through all characters
    for (_, c) in text.chars().enumerate() {
        let character = font
            .chars
            .get(&c)
            .expect(format!("Character {} not found", c).as_str());

        let xpos = x + character.bearing.x as f32 * scale;
        let ypos = y - (character.size.y - character.bearing.y) as f32 * scale;

        let w = character.size.x as f32 * scale;
        let h = character.size.y as f32 * scale;
        // update VBO for each character
        let vertices = [
            xpos,
            ypos + h,
            0.0,
            0.0, // bottom left
            xpos,
            ypos,
            0.0,
            1.0, // top left
            xpos + w,
            ypos,
            1.0,
            1.0, // top right
            xpos,
            ypos + h,
            0.0,
            0.0, // bottom left
            xpos + w,
            ypos,
            1.0,
            1.0, // top right
            xpos + w,
            ypos + h,
            1.0,
            0.0, // bottom right
        ];

        // render glyph texture over quad
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, character.texture.gl_id);
            // update content of VBO memory
            gl::BindBuffer(gl::ARRAY_BUFFER, *vbo);
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                0,
                (vertices.len() * std::mem::size_of::<f32>()) as isize,
                vertices.as_ptr() as *const GLvoid,
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            // render quad
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
        }

        // now advance cursors for next glyph (note that advance is number of 1/64 pixels)
        x += (character.advance >> 6) as f32 * scale; // bitshift by 6 to get value in pixels (2^6 = 64)
    }
}
