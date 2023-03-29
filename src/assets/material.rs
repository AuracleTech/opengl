use super::{program::Program, texture::Texture};
use serde::{Deserialize, Serialize};

// TODO remove debug everywhere
#[derive(Serialize, Deserialize, Debug)]
pub enum MaterialFormat {
    Pbr {
        albedo: Texture,
    },
    Phong {
        diffuse: Texture,
        specular: Texture,
        specular_strength: f32,
        emissive: Texture,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Material {
    pub program: Program,
    pub format: MaterialFormat,
}

impl Material {
    pub fn activate(&self) {
        match self.format {
            MaterialFormat::Pbr { ref albedo, .. } => {
                albedo.gl_bind(0);
                self.program.set_uniform_int("material.albedo", 0);
            }
            MaterialFormat::Phong { .. } => panic!("Phong material not implemented"),
        }
    }

    pub fn deactivate(&self) {
        match self.format {
            MaterialFormat::Pbr { ref albedo, .. } => {
                albedo.gl_unbind();
            }
            MaterialFormat::Phong { .. } => panic!("Phong material not implemented"),
        }
    }
}
