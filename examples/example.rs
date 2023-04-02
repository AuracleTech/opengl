use std::{cmp::Ordering, collections::BTreeMap, ops::Mul, time::Instant};

use cgmath::{
    point3, vec3, Deg, EuclideanSpace, InnerSpace, Matrix4, Point3, Quaternion, Rotation3,
    SquareMatrix, Vector3,
};
use glfw::Key;
use revenant::{
    assets::{
        camera::{Camera, CameraProjectionKind},
        model::Model,
        Assets,
    },
    Revenant,
};

fn main() {
    let mut revenant = Revenant::new();
    let mut assets = Assets::new();

    let icon_asset = assets.new_image_foreign("icon", "png");
    let mut icons = vec![icon_asset.to_glfw_pixelimage()];
    icons.push(icon_asset.to_glfw_pixelimage());
    revenant.set_window_icon(icons);

    let mut camera_controller = CameraController {
        aim_sensitivity: 0.03,
        speed_factor: 4,
        speed: 0.00,
        yaw: 200.0,
        pitch: -20.0,
        mouse_pos_last: (0.0, 0.0),
        min_fov_y: 10.0,
        max_fov_y: 130.0,
    };

    init_assets(&mut assets);

    let mut last_cycle_time = Instant::now();
    let mut last_frame_count_total = 0;

    while !revenant.should_close() {
        input(&mut revenant, &mut assets, &mut camera_controller);
        render(&mut assets);

        if Instant::now().duration_since(last_cycle_time).as_secs() > 0 {
            println!(
                "FPS: {}",
                revenant.frame_count_total - last_frame_count_total
            );
            last_frame_count_total = revenant.frame_count_total;
            last_cycle_time = Instant::now();
        }
    }
}

// OPTIMIZE test with inline, without and always inlined
#[inline]
fn init_assets(assets: &mut Assets) {
    assets.new_camera("main", Camera::perspective(point3(1.84, 0.8, 3.1)));

    assets.new_shader_foreign("pbr", "vs");
    assets.new_shader_foreign("pbr", "fs");
    assets.new_program("pbr", vec!["pbr_vs", "pbr_fs"]);

    assets.new_shader_foreign("outliner", "vs");
    assets.new_shader_foreign("outliner", "fs");
    assets.new_program("outliner", vec!["outliner_vs", "outliner_fs"]);

    assets.new_model_foreign("cube", "gltf");
    assets.new_model_foreign("window", "gltf");
    assets.new_model_foreign("grass", "gltf");
}

#[inline]
fn input(revenant: &mut Revenant, assets: &mut Assets, camera_controller: &mut CameraController) {
    let camera_main = assets.get_mut_camera("main");

    if let Some((_, scroll_y)) = revenant.inputs.mouse_scroll {
        match camera_main.projection_kind {
            CameraProjectionKind::Perspective {
                aspect_ratio,
                far,
                fov_y,
                near,
            } => {
                let mut fov_y = fov_y - scroll_y as f32;
                fov_y = fov_y.clamp(camera_controller.min_fov_y, camera_controller.max_fov_y);
                camera_main.projection_kind = CameraProjectionKind::Perspective {
                    aspect_ratio,
                    fov_y,
                    near,
                    far,
                };
            }
            _ => {}
        };
    }

    if let Some((mouse_x, mouse_y)) = revenant.inputs.mouse_pos {
        let mouse_x_delta = mouse_x - camera_controller.mouse_pos_last.0;
        let mouse_y_delta = mouse_y - camera_controller.mouse_pos_last.1;

        camera_controller.yaw -= mouse_x_delta as f32 * camera_controller.aim_sensitivity;
        camera_controller.pitch -= mouse_y_delta as f32 * camera_controller.aim_sensitivity;

        camera_controller.pitch = camera_controller.pitch.clamp(-89.9, 89.9);
        camera_controller.yaw = camera_controller.yaw.rem_euclid(360.0);

        let quat_yaw = Quaternion::from_axis_angle(Vector3::unit_y(), Deg(camera_controller.yaw));
        let quat_pitch =
            Quaternion::from_axis_angle(Vector3::unit_x(), Deg(camera_controller.pitch));
        camera_main.quat = quat_yaw * quat_pitch;
        camera_main.update();
    }

    if revenant.inputs.is_key_down(Key::W) {
        let forward = camera_main.quat * Vector3::unit_z();
        camera_main.pos -= forward * camera_controller.speed;
        camera_main.update();
    }
    if revenant.inputs.is_key_down(Key::S) {
        let forward = camera_main.quat * Vector3::unit_z();
        camera_main.pos += forward * camera_controller.speed;
        camera_main.update();
    }
    if revenant.inputs.is_key_down(Key::A) {
        let right = camera_main.quat * Vector3::unit_x();
        camera_main.pos -= right * camera_controller.speed;
        camera_main.update();
    }
    if revenant.inputs.is_key_down(Key::D) {
        let right = camera_main.quat * Vector3::unit_x();
        camera_main.pos += right * camera_controller.speed;
        camera_main.update();
    }
    if revenant.inputs.is_key_down(Key::Space) {
        let up = camera_main.quat * Vector3::unit_y();
        camera_main.pos += up * camera_controller.speed;
        camera_main.update();
    }
    if revenant.inputs.is_key_down(Key::LeftControl) {
        let up = camera_main.quat * Vector3::unit_y();
        camera_main.pos -= up * camera_controller.speed;
        camera_main.update();
    }
    if revenant.inputs.is_key_down(Key::LeftShift) {
        camera_controller.speed_factor = match camera_controller.speed_factor {
            4 => 8,
            8 => 24,
            24 => 4,
            _ => 4,
        };
    }
    if revenant.inputs.is_key_down(Key::P) {
        revenant.cycle_polygon_mode();
    }
    if revenant.inputs.is_key_down(Key::Escape) {
        revenant.set_should_close(true);
    }

    camera_controller.update(revenant.frame_time_delta as f32, revenant.inputs.mouse_pos);
}

#[inline]
fn render(assets: &mut Assets) {
    let program_pbr = assets.get_program("pbr");
    // let program_outliner = assets.get_program("outliner");
    let cube = assets.get_model("cube");
    let camera_main = assets.get_camera("main");
    let window = assets.get_model("window");
    let grass = assets.get_model("grass");
    let mut window_positions = vec![
        point3(-1.5, 0.0, -0.48),
        point3(1.5, 0.0, 0.51),
        point3(0.0, 0.0, 0.7),
        point3(-0.3, 0.0, -2.3),
        point3(0.5, 0.0, -0.6),
    ];
    window_positions.sort_unstable_by(|a, b| {
        let distance_a = (camera_main.pos - *a).magnitude();
        let distance_b = (camera_main.pos - *b).magnitude();
        distance_b
            .partial_cmp(&distance_a)
            .unwrap_or(Ordering::Equal)
    });

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        // gl::Enable(gl::STENCIL_TEST);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        // CLEAR | gl::STENCIL_BUFFER_BIT
        // gl::StencilOp(gl::KEEP, gl::KEEP, gl::REPLACE);
        // gl::StencilMask(0xFF);
        // gl::StencilFunc(gl::ALWAYS, 1, 0xFF);
    }
    {
        program_pbr.use_program();
        program_pbr.set_uniform_mat4("model", &Matrix4::identity());
        program_pbr.set_uniform_mat4("view", &camera_main.view);
        program_pbr.set_uniform_mat4("projection", &camera_main.projection);
        program_pbr.set_uniform_mat4("model", &Matrix4::from_translation(vec3(12.0, 0.0, 0.0)));
        cube.draw(program_pbr);
        program_pbr.set_uniform_mat4("model", &Matrix4::from_translation(vec3(-12.0, 0.0, 0.0)));
        grass.draw(program_pbr);

        for window_pos in window_positions {
            let mut model = Matrix4::from_translation(window_pos.to_vec());
            model = model.mul(&Matrix4::from_scale(0.25));
            program_pbr.set_uniform_mat4("model", &model);
            window.draw(program_pbr);
        }
    }

    // unsafe {
    //     gl::StencilFunc(gl::NOTEQUAL, 1, 0xFF);
    //     gl::StencilMask(0x00);
    //     gl::Disable(gl::DEPTH_TEST);
    // }
    // {
    //     program_outliner.use_program();
    //     program_outliner.set_uniform_mat4("model", &Matrix4::from_scale(1.1));
    //     program_outliner.set_uniform_mat4("view", &camera_main.view);
    //     program_outliner.set_uniform_mat4("projection", &camera_main.projection);
    //     cube.draw(program_outliner);
    // }

    // unsafe {
    //     gl::StencilMask(0xFF);
    //     gl::Enable(gl::DEPTH_TEST);
    // }
}

struct CameraController {
    aim_sensitivity: f32,
    speed_factor: u16,
    speed: f32,
    yaw: f32,
    pitch: f32,
    mouse_pos_last: (f64, f64),
    min_fov_y: f32,
    max_fov_y: f32,
}

impl CameraController {
    fn update(&mut self, frame_time_delta: f32, mouse_pos: Option<(f64, f64)>) {
        self.speed = frame_time_delta * self.speed_factor as f32;

        if let Some((mouse_x, mouse_y)) = mouse_pos {
            self.mouse_pos_last = (mouse_x, mouse_y);
        }
    }
}
