use crate::types::{Image, Mesh, Model, Program, Texture, Vertex};
use cgmath::{point3, vec2, vec3};
use gltf::{image::Source, mesh::util::ReadTexCoords};
use std::path::PathBuf;

impl Model {
    pub fn from_gltf(path: PathBuf) -> Self {
        let (gltf, buffers, _) = gltf::import(path).expect("Failed to import gltf file");
        let mut meshes = Vec::new();
        let mut textures = Vec::new();
        for mesh in gltf.meshes() {
            for primitive in mesh.primitives() {
                let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
                if let Some(iter) = reader.read_positions() {
                    let vertices = iter
                        .map(|positions| {
                            let mut normals =
                                reader.read_normals().expect("Failed to read normals");
                            let tex_coords = reader
                                .read_tex_coords(0)
                                .expect("Failed to read tex coords");
                            let position = point3(positions[0], positions[1], positions[2]);
                            let normal = normals.next().expect("Failed to read normals");
                            let normal = vec3(normal[0], normal[1], normal[2]);
                            let tex_coord = match tex_coords {
                                ReadTexCoords::U8(mut iter) => vec2(
                                    iter.next().expect("Failed to read tex coords")[0] as f32,
                                    iter.next().expect("Failed to read tex coords")[1] as f32,
                                ),
                                ReadTexCoords::U16(mut iter) => vec2(
                                    iter.next().expect("Failed to read tex coords")[0] as f32,
                                    iter.next().expect("Failed to read tex coords")[1] as f32,
                                ),
                                ReadTexCoords::F32(mut iter) => vec2(
                                    iter.next().expect("Failed to read tex coords")[0],
                                    iter.next().expect("Failed to read tex coords")[1],
                                ),
                            };
                            Vertex {
                                position,
                                normal,
                                tex_coord,
                            }
                        })
                        .collect();
                    let indices = match reader.read_indices().unwrap() {
                        gltf::mesh::util::ReadIndices::U16(indices) => {
                            indices.map(|x| u32::from(x)).collect()
                        }
                        gltf::mesh::util::ReadIndices::U32(indices) => indices.collect(),
                        gltf::mesh::util::ReadIndices::U8(indices) => {
                            indices.map(|x| u32::from(x)).collect()
                        }
                    };
                    let mesh = Mesh::new(vertices, indices, Vec::new());
                    meshes.push(mesh);
                }
            }
        }
        for texture in gltf.textures() {
            let source = texture.source().source();
            let texture = match source {
                Source::View { view, mime_type } => {
                    let data = view.buffer().source();
                    let data = match data {
                        gltf::buffer::Source::Bin => panic!("Unsupported image source"),
                        gltf::buffer::Source::Uri(uri) => uri,
                    };
                    let image = Image::from_data(data.as_bytes(), mime_type);
                    Texture::from_image(image)
                }
                _ => panic!("Unsupported image source"),
            };
            textures.push(texture);
        }
        Self { meshes, textures }
    }

    pub fn draw(&self, program: &Program) {
        for mesh in &self.meshes {
            mesh.draw(program);
        }
    }
}
