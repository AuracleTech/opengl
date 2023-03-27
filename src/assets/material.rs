use super::texture::Texture;
use serde::{Deserialize, Serialize};

// TODO remove debug everywhere
#[derive(Serialize, Deserialize, Debug)]
pub enum Material {
    Pbr {
        albedo: Texture,
        // metallic: Texture,
        // roughness: Texture,
        // ao: Texture,
        // emissive: Texture,
    },
    Phong {
        diffuse: Texture,
        specular: Texture,
        specular_strength: f32,
        emissive: Texture,
    },
    None,
}

impl Default for Material {
    fn default() -> Self {
        Self::None
    }
}
