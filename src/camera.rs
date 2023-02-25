use glm::Vec3;

pub struct Camera {
    pub pos: Vec3,
    pub front: Vec3,
    pub up: Vec3,
    pub right: Vec3,
    pub speed_factor: f32,
    pub fov_y: f32,
}
