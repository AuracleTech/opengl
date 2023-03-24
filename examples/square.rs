use cgmath::{point3, vec3, vec4, InnerSpace, Matrix4, SquareMatrix};
use glfw::Key;
use revenant::{
    assets,
    types::{Camera, Mesh, Program, ProjectionKind},
    Revenant,
};

fn main() {
    let mut revenant = Revenant::new();
    let mut camera_controller = CameraController {
        aim_sensitivity: 0.1,
        speed_factor: 300.0,
        speed: 0.00,
        yaw: 200.0,
        pitch: -20.0,
        mouse_pos_last: (0.0, 0.0),
        max_fov_y: 10.0,
        min_fov_y: 130.0,
    };

    // mesh
    const VERTICES: [f32; 12] = [
        0.5, 0.5, 0.0, //
        0.5, -0.5, 0.0, //
        -0.5, -0.5, 0.0, //
        -0.5, 0.5, 0.0, //
    ];
    const INDICES: [u32; 6] = [0, 1, 3, 1, 2, 3];
    let square_mesh = Mesh::temporary(gl::TRIANGLES, &VERTICES, &INDICES);

    // camera
    let camera_main = Camera {
        pos: point3(1.84, 0.8, 3.1),
        front: vec3(0.0, 0.0, -1.0),
        up: vec3(0.0, 1.0, 0.0),
        right: vec3(0.0, 0.0, 0.0),

        update_projection: true,
        projection_kind: ProjectionKind::Perspective {
            aspect_ratio: 16.0 / 9.0,
            fov_y: 45.0,
            near: 0.1,
            far: 100.0,
        },
        projection: Matrix4::identity(),
    };

    // shader
    let shader_light_vs = assets::load_foreign_shader("light", "vs");
    let shader_light_fs = assets::load_foreign_shader("light", "fs");
    let program_light = Program::new(shader_light_vs, shader_light_fs);

    // load assets
    revenant.assets.add_mesh("square_mesh", square_mesh);
    revenant.assets.add_camera("main", camera_main);
    revenant.assets.add_program("light", program_light);

    while !revenant.should_close() {
        input(&mut revenant, &mut camera_controller);
        render(&mut revenant);
    }
}

#[inline]
fn input(revenant: &mut Revenant, camera_controller: &mut CameraController) {
    let camera_main = revenant.assets.get_mut_camera("main");

    if let Some((_, scroll_y)) = revenant.inputs.mouse_scroll {
        match camera_main.projection_kind {
            ProjectionKind::Perspective { fov_y, .. } => {
                let fov_y = fov_y - scroll_y as f32;
                let fov_y = fov_y.clamp(camera_controller.min_fov_y, camera_controller.max_fov_y);
                camera_main.projection_kind = ProjectionKind::Perspective {
                    aspect_ratio: 16.0 / 9.0,
                    fov_y,
                    near: 0.1,
                    far: 100.0,
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

        camera_main.front = vec3(
            camera_controller.pitch.to_radians().cos() * camera_controller.yaw.to_radians().cos(),
            camera_controller.pitch.to_radians().sin(),
            camera_controller.pitch.to_radians().cos() * camera_controller.yaw.to_radians().sin(),
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
    if revenant.inputs.is_key_down(Key::LeftShift) {
        camera_main.pos -= camera_main.up * camera_controller.speed;
    }
    if revenant.inputs.is_key_down(Key::P) {
        let mut polygon_mode: [i32; 2] = [0; 2];
        unsafe {
            gl::GetIntegerv(gl::POLYGON_MODE, polygon_mode.as_mut_ptr());
        }
        let polygon_mode = match polygon_mode[0] as u32 {
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
    if revenant.inputs.is_key_down(Key::Escape) {
        revenant.set_should_close(true);
    }

    camera_controller.update(revenant.frame_time_delta as f32, revenant.inputs.mouse_pos);
}

#[inline]
fn render(revenant: &mut Revenant) {
    revenant.start_frame();

    let camera_main = revenant.assets.get_camera("main");
    let program_light = revenant.assets.get_program("light");
    let square_mesh = revenant.assets.get_mesh("square_mesh");

    program_light.use_program();
    program_light.set_uniform_mat4("model", &Matrix4::identity());
    program_light.set_uniform_mat4(
        "view",
        &Matrix4::look_at_rh(
            camera_main.pos,
            camera_main.pos + camera_main.front,
            camera_main.up,
        ),
    );
    program_light.set_uniform_mat4("projection", &camera_main.projection);
    program_light.set_uniform_vec4("color", vec4(0.322, 0.561, 0.871, 1.0));

    square_mesh.draw(program_light);

    revenant.end_frame();
}

struct CameraController {
    aim_sensitivity: f32,
    speed_factor: f32,
    speed: f32,
    yaw: f32,
    pitch: f32,
    mouse_pos_last: (f64, f64),
    min_fov_y: f32,
    max_fov_y: f32,
}

impl CameraController {
    fn update(&mut self, frame_time_delta: f32, mouse_pos: Option<(f64, f64)>) {
        self.speed = self.speed_factor * frame_time_delta;

        if let Some((mouse_x, mouse_y)) = mouse_pos {
            self.mouse_pos_last = (mouse_x, mouse_y);
        }
    }
}
