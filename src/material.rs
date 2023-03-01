use crate::texture::Texture;
use glm::Vec3;

pub struct Material {
    pub diffuse_map: Texture,
    pub specular_color: Vec3,
    pub specular_strength: f32,
}
