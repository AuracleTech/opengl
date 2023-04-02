use super::{program::Program, texture::Texture};
use serde::{Deserialize, Serialize};

// TODO remove debug everywhere
#[derive(Serialize, Deserialize, Debug)]
pub enum Material {
    Pbr {
        albedo: Texture,
    },
    Phong {
        diffuse: Texture,
        specular: Texture,
        specular_strength: f32,
        emissive: Texture,
    },
    Normal {
        normal: Texture,
    },
}

impl Material {
    pub fn activate(&self, program: &Program) {
        match self {
            Material::Pbr { ref albedo, .. } => {
                // TODO bind texture unit
                albedo.gl_bind(0);
                program.set_uniform_int("material.albedo", 0);
            }
            _ => panic!("Phong material not implemented"),
        }
    }

    #[cfg(feature = "pillow")]
    pub fn deactivate(&self) {
        match self {
            Material::Pbr { ref albedo, .. } => {
                // TODO unbind texture unit
                albedo.gl_unbind();
            }
            _ => panic!("Phong material not implemented"),
        }
    }
}
