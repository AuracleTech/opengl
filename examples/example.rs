use cgmath::{point3, vec3, InnerSpace, Matrix4, SquareMatrix};
use glfw::Key;
use revenant::{
    assets::{
        camera::{Camera, CameraProjectionKind},
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

    while !revenant.should_close() {
        assets.update();
        input(&mut revenant, &mut assets, &mut camera_controller);
        render(&mut assets);
    }
}

#[inline]
fn init_assets(assets: &mut Assets) {
    assets.new_camera("main", Camera::new_perspective(point3(1.84, 0.8, 3.1)));

    assets.new_shader_foreign("pbr", "vs");
    assets.new_shader_foreign("pbr", "fs");
    assets.new_program("pbr", vec!["pbr_vs", "pbr_fs"]);

    // assets.new_shader_foreign("outliner", "vs");
    // assets.new_shader_foreign("outliner", "fs");
    // assets.new_program("outliner", vec!["outliner_vs", "outliner_fs"]);

    assets.new_model_foreign("cube", "glb");
    assets.new_model_foreign("tree", "glb");
    assets.new_model_foreign("cube_textured", "glb");
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
                camera_main.update_projection = true;
            }
            _ => {}
        };
    }

    if let Some((mouse_x, mouse_y)) = revenant.inputs.mouse_pos {
        let mouse_x_delta = mouse_x - camera_controller.mouse_pos_last.0;
        let mouse_y_delta = mouse_y - camera_controller.mouse_pos_last.1;

        camera_controller.yaw += mouse_x_delta as f32 * camera_controller.aim_sensitivity;
        camera_controller.pitch -= mouse_y_delta as f32 * camera_controller.aim_sensitivity;

        camera_controller.pitch = camera_controller.pitch.clamp(-89.9, 89.9);
        camera_controller.yaw = camera_controller.yaw.rem_euclid(360.0);

        let pitch_radians_cos = camera_controller.pitch.to_radians().cos();
        camera_main.front = vec3(
            pitch_radians_cos * camera_controller.yaw.to_radians().cos(),
            camera_controller.pitch.to_radians().sin(),
            pitch_radians_cos * camera_controller.yaw.to_radians().sin(),
        )
        .normalize();
        camera_main.right = camera_main.front.cross(camera_main.up);
    }

    if revenant.inputs.is_key_down(Key::W) {
        camera_main.pos += camera_main.front * camera_controller.speed;
    }
    if revenant.inputs.is_key_down(Key::S) {
        camera_main.pos -= camera_main.front * camera_controller.speed;
    }
    if revenant.inputs.is_key_down(Key::A) {
        camera_main.pos -= camera_main.right * camera_controller.speed;
    }
    if revenant.inputs.is_key_down(Key::D) {
        camera_main.pos += camera_main.right * camera_controller.speed;
    }
    if revenant.inputs.is_key_down(Key::Space) {
        camera_main.pos += camera_main.up * camera_controller.speed;
    }
    if revenant.inputs.is_key_down(Key::LeftControl) {
        camera_main.pos -= camera_main.up * camera_controller.speed;
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
    let camera_main = assets.get_camera("main");
    let program_pbr = assets.get_program("pbr");
    // let program_outliner = assets.get_program("outliner");
    let cube = assets.get_model("cube");
    let tree = assets.get_model("tree");
    let cube_textured = assets.get_model("cube_textured");

    program_pbr.use_program();
    program_pbr.set_uniform_mat4("model", &Matrix4::identity());
    program_pbr.set_uniform_mat4(
        "view",
        &Matrix4::look_at_rh(
            camera_main.pos,
            camera_main.pos + camera_main.front,
            camera_main.up,
        ),
    );
    program_pbr.set_uniform_mat4("projection", &camera_main.projection);

    // unsafe {
    //     gl::StencilOp(gl::KEEP, gl::KEEP, gl::REPLACE);
    //     gl::StencilFunc(gl::ALWAYS, 1, 0xFF); // all fragments should pass the stencil test
    //     gl::StencilMask(0xFF); // enable writing to the stencil buffer
    // }
    tree.draw();
    program_pbr.set_uniform_mat4("model", &Matrix4::from_translation(vec3(0.0, 0.0, 4.0)));
    cube.draw();
    program_pbr.set_uniform_mat4("model", &Matrix4::from_translation(vec3(0.0, 0.0, -4.0)));
    cube_textured.draw();
    // program_pbr.set_uniform_mat4("model", &Matrix4::from_translation(vec3(0.0, 0.0, 8.0)));

    // unsafe {
    //     gl::StencilFunc(gl::NOTEQUAL, 1, 0xFF);
    //     gl::StencilMask(0x00); // disable writing to the stencil buffer
    //     gl::Disable(gl::DEPTH_TEST);
    // }
    // program_outliner.use_program();
    // program_outliner.set_uniform_mat4("model", &Matrix4::identity());
    // program_outliner.set_uniform_mat4(
    //     "view",
    //     &Matrix4::look_at_rh(
    //         camera_main.pos,
    //         camera_main.pos + camera_main.front,
    //         camera_main.up,
    //     ),
    // );
    // program_outliner.set_uniform_mat4("projection", &camera_main.projection);
    // tree.draw(program_outliner);
    // program_outliner.set_uniform_mat4("model", &Matrix4::from_translation(vec3(0.0, 0.0, 4.0)));
    // cube.draw(program_outliner);
    // program_outliner.set_uniform_mat4("model", &Matrix4::from_translation(vec3(0.0, 0.0, -4.0)));
    // cube_textured.draw(program_outliner);
    // program_outliner.set_uniform_mat4("model", &Matrix4::from_translation(vec3(0.0, 0.0, 8.0)));
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
