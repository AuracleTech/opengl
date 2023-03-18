use std::collections::HashMap;
mod camera;
mod font;
mod image;
mod program;
mod texture;

impl crate::types::Assets {
    pub fn new() -> Self {
        Self {
            programs: HashMap::new(),
            images: HashMap::new(),
            textures: HashMap::new(),
            materials: HashMap::new(),
            fonts: HashMap::new(),
            // TODO meshes: HashMap::new(),
            cameras: HashMap::new(),
            point_lights: HashMap::new(),
            dir_lights: HashMap::new(),
            spot_lights: HashMap::new(),
        }
    }
}
