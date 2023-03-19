use std::collections::HashMap;
mod camera;
mod font;
mod image;
mod program;
mod shader;
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
            pointlights: HashMap::new(),
            dirlights: HashMap::new(),
            spotlights: HashMap::new(),
        }
    }
}
