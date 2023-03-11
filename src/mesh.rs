use crate::{
    program::Program,
    types::{Indices, Mesh, Textures, Vertecies},
};

impl Mesh {
    pub fn new(vertices: Vertecies, indices: Indices, textures: Textures) -> Self {
        todo!("Implement Mesh::new");
    }

    pub fn draw(&self, program: &Program) {
        todo!("Implement Mesh::draw");
    }
}
