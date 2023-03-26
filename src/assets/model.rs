use crate::types::{Image, Material, Mesh, Model, Program, Texture, Uniaxial, Vertex};
use cgmath::{vec2, vec3, Vector2};
use gltf::{
    image::Source,
    mesh::{
        util::{ReadColors, ReadIndices, ReadTexCoords},
        Mode,
    },
};
use std::path::PathBuf;

impl Model {
    pub fn from_gltf(path: PathBuf) -> Self {
        let (gltf, buffers, _) = gltf::import(&path).expect("Failed to import gltf file");
        let mut meshes = Vec::new();

        let mut buffer_data = Vec::new();
        for buffer in gltf.buffers() {
            match buffer.source() {
                gltf::buffer::Source::Bin => {
                    // if let Some(blob) = gltf.blob.as_deref() {
                    //     buffer_data.push(blob.into());
                    //     println!("Found a bin, saving");
                    // };
                }
                gltf::buffer::Source::Uri(uri) => {
                    let bin = std::fs::read(uri).expect("Failed to read buffer data");
                    buffer_data.push(bin);
                }
            }
        }

        let mut mesh_set = 0;
        for mesh in gltf.meshes() {
            for primitive in mesh.primitives() {
                let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

                // TODO Vertex color - Needs a new type called VertexGroup or something check on blender
                let mut vertices = Vec::new();
                if let Some(vertex_attribute) = reader.read_positions() {
                    vertex_attribute.for_each(|vertex| {
                        vertices.push(Vertex {
                            position: vertex.into(),
                            normal: vec3(0.0, 0.0, 0.0),
                            tex_coords: vec2(0.0, 0.0),
                        })
                    });
                }
                if let Some(normal_attribute) = reader.read_normals() {
                    let mut normal_index = 0;
                    normal_attribute.for_each(|normal| {
                        vertices[normal_index].normal = normal.into();

                        normal_index += 1;
                    });
                }
                if let Some(tex_coord_attribute) = reader.read_tex_coords(0).map(|v| v.into_f32()) {
                    let mut tex_coord_index = 0;
                    tex_coord_attribute.for_each(|tex_coord| {
                        vertices[tex_coord_index].tex_coords = tex_coord.into();

                        tex_coord_index += 1;
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

                let mut indices = Vec::new();
                if let Some(indices_raw) = reader.read_indices() {
                    indices.append(&mut indices_raw.into_u32().collect::<Vec<u32>>());
                }

                let mut textures = Vec::new();
                // let mut materials = Vec::new();
                // for material in gltf.materials() {
                //     let pbr = material.pbr_metallic_roughness();
                //     let base_color_texture = &pbr.base_color_texture();
                //     let texture_source = &pbr
                //         .base_color_texture()
                //         .map(|tex| {
                //             println!("Grabbing diffuse tex");
                //             dbg!(&tex.texture().source());
                //             tex.texture().source().source()
                //         })
                //         .expect("texture");

                //     match texture_source {
                //         gltf::image::Source::View { view, mime_type } => {
                //             let diffuse_image =
                //                 Image::from_data(&buffer_data[view.buffer().index()], mime_type);
                //             materials.push(Material {
                //                 name: material.name().unwrap_or("Default Material").to_string(),
                //                 diffuse_texture,
                //             });
                //         }
                //         gltf::image::Source::Uri { uri, mime_type } => {
                //             let diffuse_texture = load_texture(uri, device, queue).await?;

                //             materials.push(model::Material {
                //                 name: material.name().unwrap_or("Default Material").to_string(),
                //                 diffuse_texture,
                //             });
                //         }
                //     };
                // }

                meshes.push(Mesh::new(gl_mode, vertices, indices, textures));
                mesh_set += 1;
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
