use crate::types::{Mesh, Model, Program, Vertex};
use cgmath::{point3, vec3};
use gltf::mesh::{util::ReadIndices, Mode};
use std::path::PathBuf;

impl Model {
    pub fn from_gltf(path: PathBuf) -> Self {
        let (gltf, buffers, _) = gltf::import(&path).expect("Failed to import gltf file");
        let mut meshes = Vec::new();

        for mesh in gltf.meshes() {
            for primitive in mesh.primitives() {
                let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
                let mut positions = reader.read_positions().expect("Failed to read positions");
                let mut vertices = Vec::new();

                loop {
                    let position = match positions.next() {
                        Some(position) => position,
                        None => break,
                    };
                    let position = point3(position[0], position[1], position[2]);
                    let normal = vec3(0.0, 0.0, 0.0);
                    // let tex_coords = vec2(0.0, 0.0);
                    vertices.push(Vertex {
                        position,
                        normal,
                        // tex_coords,
                    });
                }

                let gl_mode = match primitive.mode() {
                    Mode::Points => gl::POINTS,
                    Mode::Lines => gl::LINES,
                    Mode::LineLoop => gl::LINE_LOOP,
                    Mode::LineStrip => gl::LINE_STRIP,
                    Mode::Triangles => gl::TRIANGLES,
                    Mode::TriangleStrip => gl::TRIANGLE_STRIP,
                    Mode::TriangleFan => gl::TRIANGLE_FAN,
                };

                // Create indices
                let indices = match reader.read_indices().expect("Failed to read indices") {
                    ReadIndices::U16(indices) => indices.map(|x| u32::from(x)).collect(),
                    ReadIndices::U32(indices) => indices.collect(),
                    ReadIndices::U8(indices) => indices.map(|x| u32::from(x)).collect(),
                };
                // TODO textures
                meshes.push(Mesh::new(gl_mode, vertices, indices, Vec::new()));
            }
        }

        Self { meshes }
    }

    pub fn draw(&self, program: &Program) {
        for mesh in &self.meshes {
            mesh.draw(program);
        }
    }
}
