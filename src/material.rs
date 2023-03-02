use crate::texture::Texture;

pub struct Material {
    pub diffuse_map: Texture,
    pub specular_map: Texture,
    pub specular_strength: f32,
}
