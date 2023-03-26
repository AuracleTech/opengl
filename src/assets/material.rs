use super::texture::Texture;
use serde::{Deserialize, Serialize};

// TODO remove debug everywhere
#[derive(Serialize, Deserialize, Debug)]
pub struct Material {
    pub diffuse: Texture,
    pub specular: Texture,
    pub specular_strength: f32,
    pub emissive: Texture,
}
