use glm::Vec3;

pub struct Material {
    pub ambient_color: Vec3,
    pub diffuse_color: Vec3,
    pub specular_color: Vec3,
    pub specular_strength: f32,
}
