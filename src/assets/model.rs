use cgmath::{vec2, vec3};
use gltf::{
    image::Source,
    mesh::Mode,
    texture::{MagFilter, MinFilter, WrappingMode},
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use super::{
    image::Image,
    mesh::{Mesh, Vertex},
    program::Program, // TODO remove Vertex and create a function inside mesh to load the mesh 🧠
    texture::Texture,
}; // TODO put in sub module?

#[derive(Serialize, Deserialize, Debug)]
pub struct Model {
    pub(crate) meshes: Vec<Mesh>,
    pub(crate) textures: Vec<Texture>,
}

impl Model {
    pub fn from_gltf(path: PathBuf) -> Self {
        let (gltf, buffers, _) = gltf::import(&path).expect("Failed to import gltf file");

        let mut meshes = Vec::new();

        let mut buffer_data = Vec::new();
        for buffer in gltf.buffers() {
            match buffer.source() {
                gltf::buffer::Source::Bin => {
                    buffer_data.push(buffers[buffer.index()].clone());
                }
                gltf::buffer::Source::Uri(uri) => {
                    let bin = std::fs::read(uri).expect("Failed to read buffer data");
                    buffer_data.push(gltf::buffer::Data(bin));
                }
            }
        }

        let mut textures = Vec::new();
        for material in gltf.materials() {
            let pbr = material.pbr_metallic_roughness();
            let base_color_texture = &pbr.base_color_texture();
            if let Some(texture_source) = &pbr
                .base_color_texture()
                .map(|tex| tex.texture().source().source())
            {
                let texture = base_color_texture
                    .as_ref()
                    .expect("Expected a texture but found none")
                    .texture();

                let wrap_s = match texture.sampler().wrap_s() {
                    WrappingMode::ClampToEdge => gl::CLAMP_TO_EDGE,
                    WrappingMode::MirroredRepeat => gl::MIRRORED_REPEAT,
                    WrappingMode::Repeat => gl::REPEAT,
                };
                let wrap_t = match texture.sampler().wrap_t() {
                    WrappingMode::ClampToEdge => gl::CLAMP_TO_EDGE,
                    WrappingMode::MirroredRepeat => gl::MIRRORED_REPEAT,
                    WrappingMode::Repeat => gl::REPEAT,
                };
                if let Some(filter_min) = texture.sampler().min_filter() {
                    match filter_min {
                        MinFilter::Nearest => gl::NEAREST,
                        MinFilter::Linear => gl::LINEAR,
                        MinFilter::NearestMipmapNearest => gl::NEAREST_MIPMAP_NEAREST,
                        MinFilter::LinearMipmapNearest => gl::LINEAR_MIPMAP_NEAREST,
                        MinFilter::NearestMipmapLinear => gl::NEAREST_MIPMAP_LINEAR,
                        MinFilter::LinearMipmapLinear => gl::LINEAR_MIPMAP_LINEAR,
                    };
                }
                if let Some(filter_mag) = texture.sampler().mag_filter() {
                    match filter_mag {
                        MagFilter::Nearest => gl::NEAREST,
                        MagFilter::Linear => gl::LINEAR,
                    };
                }
                let diffuse_image = match texture_source {
                    Source::Uri { uri, .. } => Image::from_uri(uri),
                    Source::View { view, .. } => {
                        let data = &buffer_data[view.buffer().index()][view.offset()..];
                        Image::from_data(data)
                    }
                };
                let mut texture = Texture::from_image(diffuse_image);
                texture.gl_s_wrapping = wrap_s;
                texture.gl_t_wrapping = wrap_t;
                textures.push(texture);
            }
        }

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

        Self { meshes, textures }
    }

    pub fn draw(&self, program: &Program) {
        for mesh in &self.meshes {
            mesh.draw(program);
        }
    }
}
