use glm::Vec3;

pub struct Light {
    pub pos: Vec3,

    pub ambient_color: Vec3,
    pub diffuse_color: Vec3,
    pub specular_color: Vec3,
}
