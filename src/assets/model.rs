use super::{
    image::Image,
    material::Material,
    // TODO remove Vertex and create a function inside mesh to load the mesh 🧠
    mesh::{Mesh, Vertex},
    program::Program,
    texture::Texture,
};
use base64::{engine::general_purpose, Engine};
use cgmath::{vec2, vec3};
use gltf::{
    image::Source,
    mesh::Mode,
    texture::{MagFilter, MinFilter, WrappingMode},
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf}; // TODO put in sub module?

#[derive(Serialize, Deserialize, Debug)]
pub struct Model {
    meshes: Vec<Mesh>,
    materials: Vec<Material>,
    material_meshes_pairs: HashMap<usize, Vec<usize>>,
}

impl Model {
    pub fn from_gltf(path: PathBuf) -> Self {
        let (gltf, buffers, _) = gltf::import(&path).expect("Failed to import gltf file");

        let mut buffer_data = Vec::new();
        for buffer in gltf.buffers() {
            match buffer.source() {
                gltf::buffer::Source::Bin => {
                    buffer_data.push(buffers[buffer.index()].clone());
                }
                gltf::buffer::Source::Uri(uri) => {
                    if path.extension().unwrap() != "glb" {
                        let uri = uri.trim_start_matches("data:application/octet-stream;base64,");
                        let data = uri.as_bytes();
                        let bin = general_purpose::STANDARD
                            .decode(data)
                            .expect("Failed to decode buffer data");
                        buffer_data.push(gltf::buffer::Data(bin));
                    } else {
                        let bin = std::fs::read(uri).expect("Failed to read buffer data");
                        buffer_data.push(gltf::buffer::Data(bin));
                    }
                }
            }
        }

        let mut materials = Vec::new();
        let mut material_meshes_pairs = HashMap::new();
        for gltf_material in gltf.materials() {
            let pbr = gltf_material.pbr_metallic_roughness();
            if let Some(gltf_texture) = &pbr.base_color_texture().map(|tex| tex.texture()) {
                // println!(
                //     "Loading PBR for model {} with texture {}",
                //     path.display(),
                //     gltf_texture.index()
                // );

                let wrap_s = match gltf_texture.sampler().wrap_s() {
                    WrappingMode::ClampToEdge => gl::CLAMP_TO_EDGE,
                    WrappingMode::MirroredRepeat => gl::MIRRORED_REPEAT,
                    WrappingMode::Repeat => gl::REPEAT,
                };
                let wrap_t = match gltf_texture.sampler().wrap_t() {
                    WrappingMode::ClampToEdge => gl::CLAMP_TO_EDGE,
                    WrappingMode::MirroredRepeat => gl::MIRRORED_REPEAT,
                    WrappingMode::Repeat => gl::REPEAT,
                };
                if let Some(filter_min) = gltf_texture.sampler().min_filter() {
                    match filter_min {
                        MinFilter::Nearest => gl::NEAREST,
                        MinFilter::Linear => gl::LINEAR,
                        MinFilter::NearestMipmapNearest => gl::NEAREST_MIPMAP_NEAREST,
                        MinFilter::LinearMipmapNearest => gl::LINEAR_MIPMAP_NEAREST,
                        MinFilter::NearestMipmapLinear => gl::NEAREST_MIPMAP_LINEAR,
                        MinFilter::LinearMipmapLinear => gl::LINEAR_MIPMAP_LINEAR,
                    };
                }
                if let Some(filter_mag) = gltf_texture.sampler().mag_filter() {
                    match filter_mag {
                        MagFilter::Nearest => gl::NEAREST,
                        MagFilter::Linear => gl::LINEAR,
                    };
                }

                let texture_source = gltf_texture.source().source();

                let albedo_image = match texture_source {
                    Source::Uri { uri, .. } => Image::from_uri(uri),
                    Source::View { view, .. } => {
                        let data = &buffer_data[view.buffer().index()][view.offset()..];
                        Image::from_data(data)
                    }
                };
                let mut albedo = Texture::new(albedo_image);
                albedo.gl_s_wrapping = wrap_s;
                albedo.gl_t_wrapping = wrap_t;
                // TODO add min and mag filter
                // TODO mipmaps? and all the other texture options

                materials.push(Material::Pbr { albedo });

                // TEMPORARY - ASSIGN EVERY MESH TO THE FIRST MATERIAL
                material_meshes_pairs.insert(0, vec![0]);
            }

            // if let Some(normal_texture) = gltf_material.normal_texture() {
            //     println!(
            //         "Loading normal texture for model {} with texture {}",
            //         path.display(),
            //         normal_texture.texture().index()
            //     );

            //     let wrap_s = match normal_texture.texture().sampler().wrap_s() {
            //         WrappingMode::ClampToEdge => gl::CLAMP_TO_EDGE,
            //         WrappingMode::MirroredRepeat => gl::MIRRORED_REPEAT,
            //         WrappingMode::Repeat => gl::REPEAT,
            //     };

            //     let wrap_t = match normal_texture.texture().sampler().wrap_t() {
            //         WrappingMode::ClampToEdge => gl::CLAMP_TO_EDGE,
            //         WrappingMode::MirroredRepeat => gl::MIRRORED_REPEAT,
            //         WrappingMode::Repeat => gl::REPEAT,
            //     };

            //     if let Some(filter_min) = normal_texture.texture().sampler().min_filter() {
            //         match filter_min {
            //             MinFilter::Nearest => gl::NEAREST,
            //             MinFilter::Linear => gl::LINEAR,
            //             MinFilter::NearestMipmapNearest => gl::NEAREST_MIPMAP_NEAREST,
            //             MinFilter::LinearMipmapNearest => gl::LINEAR_MIPMAP_NEAREST,
            //             MinFilter::NearestMipmapLinear => gl::NEAREST_MIPMAP_LINEAR,
            //             MinFilter::LinearMipmapLinear => gl::LINEAR_MIPMAP_LINEAR,
            //         };
            //     }

            //     if let Some(filter_mag) = normal_texture.texture().sampler().mag_filter() {
            //         match filter_mag {
            //             MagFilter::Nearest => gl::NEAREST,
            //             MagFilter::Linear => gl::LINEAR,
            //         };
            //     }

            //     let texture_source = normal_texture.texture().source().source();

            //     let normal_image = match texture_source {
            //         Source::Uri { uri, .. } => Image::from_uri(uri),
            //         Source::View { view, .. } => {
            //             let data = &buffer_data[view.buffer().index()][view.offset()..];
            //             Image::from_data(data)
            //         }
            //     };

            //     let mut normal = Texture::new(normal_image);
            //     normal.gl_s_wrapping = wrap_s;
            //     normal.gl_t_wrapping = wrap_t;
            //     // TODO add min and mag filter
            //     // TODO mipmaps? and all the other texture options

            //     materials.push(Material::Normal { normal });
            // }
        }

        let mut meshes = Vec::new();
        for mesh in gltf.meshes() {
            for primitive in mesh.primitives() {
                let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

                // TODO Vertex color - Needs a new type called VertexGroup or something check on blender
                let mut vertices = Vec::new();
                if let Some(position_attribute) = reader.read_positions() {
                    position_attribute.for_each(|position| {
                        vertices.push(Vertex {
                            position: position.into(),
                            normal: vec3(0.0, 0.0, 0.0),
                            tex_coord: vec2(0.0, 0.0),
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
                        vertices[tex_coord_index].tex_coord = tex_coord.into();
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

                meshes.push(Mesh::new(gl_mode, vertices, indices));
            }
        }

        Self {
            meshes,
            materials,
            material_meshes_pairs,
        }
    }

    pub fn draw(&self, program: &Program) {
        // TODO draw default objects with a hardcoded material program
        for (mat_index, mesh_indexes) in &self.material_meshes_pairs {
            let material = &self.materials[*mat_index as usize];
            material.activate(&program);
            for mesh_index in mesh_indexes {
                let mesh = &self.meshes[*mesh_index as usize];
                match mesh.gl_mode {
                    gl::TRIANGLES => mesh.draw(),
                    // OPTIMIZE quads should be deprecated in favor of gl::TRIANGLES
                    gl::QUADS => panic!("QUADS are deprecated no longer supported!"),
                    _ => panic!("Unsupported gl_mode yet!"),
                }
            }

            #[cfg(feature = "pillow")]
            material.deactivate();
        }
    }
}
