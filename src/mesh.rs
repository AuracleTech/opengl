use crate::types::{Indice, Mesh, Program, Texture, Vertex};

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<Indice>, textures: Vec<Texture>) -> Self {
        todo!("Implement Mesh::new");
    }

    pub fn draw(&self, program: &Program) {
        todo!("Implement Mesh::draw");
    }
}
