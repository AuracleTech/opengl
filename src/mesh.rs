use crate::{
    program::Program,
    texture::{Texture, TextureKind},
    vertex::Vertex,
};

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub textures: Vec<Texture>,

    pub vao: u32,
    pub vbo: u32,
    pub ebo: u32,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>, textures: Vec<Texture>) -> Self {
        todo!("Implement Mesh::new");
    }

    pub fn draw(&self, program: &Program) {
        todo!("Implement Mesh::draw");
    }
}
