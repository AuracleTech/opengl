use super::{program::Program, texture::Texture};
use serde::{Deserialize, Serialize};

// TODO remove debug everywhere
#[derive(Serialize, Deserialize, Debug)]
pub enum MaterialKind {
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
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Material {
    pub program: Program,
    pub kind: MaterialKind,
}

impl Material {
    pub fn activate(&self) {
        self.program.use_program();
        match self.kind {
            MaterialKind::Pbr { ref albedo, .. } => {
                albedo.gl_bind(0);
                self.program.set_uniform_int("material.albedo", 0);
            }
            MaterialKind::Phong { .. } => panic!("Phong material not implemented"),
        }
    }

    pub fn deactivate(&self) {
        match self.kind {
            MaterialKind::Pbr { ref albedo, .. } => {
                albedo.gl_unbind();
            }
            MaterialKind::Phong { .. } => panic!("Phong material not implemented"),
        }
    }
}
