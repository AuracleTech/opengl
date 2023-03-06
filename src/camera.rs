use cgmath::{Point3, Vector3};

pub struct Camera {
    pub pos: Point3<f32>,
    pub front: Vector3<f32>,
    pub up: Vector3<f32>,
    pub right: Vector3<f32>,
    pub speed_factor: f32,
    pub fov_y: f32,
    pub fov_y_min: f32,
    pub fov_y_max: f32,
    pub speed: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub aim_sensitivity: f32,
}
