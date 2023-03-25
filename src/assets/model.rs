use crate::types::{Mesh, Model, Program, Uniaxial, Vertex};
use cgmath::{vec2, Vector2};
use gltf::mesh::{
    util::{ReadIndices, ReadTexCoords},
    Mode,
};
use std::path::PathBuf;

impl Model {
    pub fn from_gltf(path: PathBuf) -> Self {
        let (gltf, buffers, _) = gltf::import(&path).expect("Failed to import gltf file");
        let mut meshes = Vec::new();

        for mesh in gltf.meshes() {
            for primitive in mesh.primitives() {
                let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
                let mut positions = reader.read_positions().expect("Failed to read positions");
                let mut normals = reader.read_normals().expect("Failed to read normals");
                let mut vertices = Vec::new();

                let mut set = 0;
                loop {
                    let position = match positions.next() {
                        Some(position) => position,
                        None => break,
                    }
                    .into();
                    let normal = match normals.next() {
                        Some(normal) => normal,
                        None => break,
                    }
                    .into();
                    let tex_coords = match reader.read_tex_coords(set) {
                        Some(tex_coords) => next_tex_coords(tex_coords),
                        None => vec2(0.0, 0.0),
                    };
                    set += 1;
                    vertices.push(Vertex {
                        position,
                        normal,
                        tex_coords,
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

fn next_tex_coords(read_tex_coords: ReadTexCoords) -> Vector2<Uniaxial> {
    match read_tex_coords {
        ReadTexCoords::F32(mut uv) => uv.next().unwrap().into(),
        ReadTexCoords::U8(mut uv) => {
            let uv = uv.next().unwrap();
            vec2(uv[0] as f32 / 255.0, uv[1] as f32 / 255.0)
        }
        ReadTexCoords::U16(mut uv) => {
            let uv = uv.next().unwrap();
            vec2(uv[0] as f32 / 65535.0, uv[1] as f32 / 65535.0)
        }
    }
}
