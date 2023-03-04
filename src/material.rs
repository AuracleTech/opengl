use crate::texture::Texture;

pub struct Material {
    pub diffuse: Texture,
    pub specular: Texture,
    pub specular_strength: f32,
}
