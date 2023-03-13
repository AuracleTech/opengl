use crate::types::{AssetTexture, Indice, Mesh, Program, Vertex};

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<Indice>, textures: Vec<AssetTexture>) -> Self {
        todo!("Implement Mesh::new");
    }

    pub fn draw(&self, program: &Program) {
        todo!("Implement Mesh::draw");
    }
}
