use crate::types::{Indice, Normal, Position, TexCoord};
use gl::types::{GLenum, GLsizei, GLsizeiptr, GLuint, GLvoid};
use serde::{Deserialize, Serialize};
use std::ffi::c_void;

#[derive(Serialize, Deserialize, Debug)]
pub struct Mesh {
    pub(crate) gl_mode: GLenum,

    pub(crate) vertices: Vec<Vertex>,
    pub(crate) indices: Vec<Indice>,

    pub vao: GLuint, // FIX set private
    pub vbo: GLuint, // FIX set private
    pub ebo: GLuint, // FIX set private

    pub draw_type: MeshDrawType,
}

// IMPLEMENT
pub enum MeshDrawMode {
    Static,
    Dynamic,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum MeshDrawType {
    DrawElements,
    DrawArrays,
}

#[derive(Serialize, Deserialize, Debug)]
#[repr(C)]
pub struct Vertex {
    pub position: Position,
    pub normal: Normal,
    pub tex_coord: TexCoord,
}

// TODO default texture similar to garry's mod (white & black checkerboard)
impl Mesh {
    pub fn new(
        gl_mode: GLenum,
        vertices: Vec<Vertex>,
        indices: Vec<Indice>,
        draw_type: MeshDrawType,
    ) -> Self {
        let mut mesh = Self {
            gl_mode,
            vertices,
            indices,
            vao: 0,
            vbo: 0,
            ebo: 0,
            draw_type,
        };
        mesh.setup_vao_vbo_ebo();
        mesh.gl_setup_vertex_attribs();
        mesh
    }

    // TEMP needs to make vertex attrib more flexible
    // FIX remake this whole thing ( DISGUSTANG )
    pub fn quad() -> Self {
        let mut mesh = Self {
            vao: 0,
            vbo: 0,
            ebo: 0,
            vertices: vec![
                Vertex {
                    position: Position::new(1.0, 1.0, 0.0),
                    normal: Normal::new(0.0, 0.0, 1.0),
                    tex_coord: TexCoord::new(1.0, 1.0),
                },
                Vertex {
                    position: Position::new(1.0, -1.0, 0.0),
                    normal: Normal::new(0.0, 0.0, 1.0),
                    tex_coord: TexCoord::new(1.0, 0.0),
                },
                Vertex {
                    position: Position::new(-1.0, -1.0, 0.0),
                    normal: Normal::new(0.0, 0.0, 1.0),
                    tex_coord: TexCoord::new(0.0, 0.0),
                },
                Vertex {
                    position: Position::new(-1.0, 1.0, 0.0),
                    normal: Normal::new(0.0, 0.0, 1.0),
                    tex_coord: TexCoord::new(0.0, 1.0),
                },
            ],
            indices: vec![0, 1, 3, 1, 2, 3],
            gl_mode: gl::TRIANGLES,
            draw_type: MeshDrawType::DrawElements,
        };
        mesh.setup_vao_vbo_ebo();
        let stride = std::mem::size_of::<Vertex>() as GLsizei;
        let offset_tex_coords = std::mem::size_of::<Position>() + std::mem::size_of::<Normal>();
        unsafe {
            // vertex positions
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
            // vertex texture coords
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                stride,
                offset_tex_coords as *const c_void,
            );
        }
        mesh
    }

    pub fn skybox() -> Self {
        let vertices_positions = [
            -1.0, 1.0, -1.0, -1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0,
            -1.0, 1.0, -1.0, -1.0, -1.0, 1.0, -1.0, -1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0,
            -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
            1.0, 1.0, 1.0, 1.0, -1.0, 1.0, -1.0, -1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0,
            1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0,
            1.0, 1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, -1.0, -1.0, -1.0, -1.0, -1.0,
            -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 1.0,
        ];
        let mut vertices = Vec::new();
        for i in 0..vertices_positions.len() / 3 {
            vertices.push(Vertex {
                position: Position::new(
                    vertices_positions[i * 3],
                    vertices_positions[i * 3 + 1],
                    vertices_positions[i * 3 + 2],
                ),
                normal: Normal::new(0.0, 0.0, 0.0),
                tex_coord: TexCoord::new(0.0, 0.0),
            });
        }
        let mut mesh = Mesh {
            vao: 0,
            vbo: 0,
            ebo: 0,
            vertices,
            indices: vec![],
            gl_mode: gl::TRIANGLES,
            draw_type: MeshDrawType::DrawArrays,
        };
        mesh.setup_vao_vbo_ebo();
        let stride = std::mem::size_of::<Vertex>() as GLsizei;
        let offset_tex_coords = std::mem::size_of::<Position>() + std::mem::size_of::<Normal>();
        unsafe {
            // vertex positions
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
            // vertex texture coords
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                stride,
                offset_tex_coords as *const c_void,
            );
        }
        mesh
    }

    pub fn setup_vao_vbo_ebo(&mut self) {
        let size = (self.vertices.len() * std::mem::size_of::<Vertex>()) as GLsizeiptr;
        let data = self.vertices.as_ptr();
        let ebo_size = (self.indices.len() * std::mem::size_of::<Indice>()) as GLsizeiptr;
        unsafe {
            gl::GenBuffers(1, &mut self.vbo);
            gl::GenVertexArrays(1, &mut self.vao);
            gl::GenBuffers(1, &mut self.ebo);

            // VAO
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);

            // VBO
            gl::BufferData(
                gl::ARRAY_BUFFER,
                size,
                data as *const c_void,
                gl::STATIC_DRAW,
            );

            // EBO
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                ebo_size,
                self.indices.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );
        }
    }

    #[inline]
    fn gl_setup_vertex_attribs(&self) {
        /*
        // TODO check if current_vertex_attribs <= max_vertex_attribs before initializing each vertex attributes
        let mut gl_max_vertex_attribs = 0;
        unsafe {
            gl::GetIntegerv(gl::MAX_VERTEX_ATTRIBS, &mut gl_max_vertex_attribs);
        }
         */

        let stride = std::mem::size_of::<Vertex>() as GLsizei;
        let offset_normals = std::mem::size_of::<Position>();
        let offset_tex_coords = std::mem::size_of::<Position>() + std::mem::size_of::<Normal>();
        unsafe {
            // FIX flexiblity of vertex attributes
            // vertex positions
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
            // vertex normals
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                stride,
                offset_normals as *const c_void,
            );
            // vertex texture coords
            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(
                2,
                2,
                gl::FLOAT,
                gl::FALSE,
                stride,
                offset_tex_coords as *const c_void,
            );
        }
    }

    pub fn draw(&self) {
        self.gl_bind_vao();

        match self.draw_type {
            MeshDrawType::DrawArrays => unsafe {
                gl::DrawArrays(self.gl_mode, 0, self.vertices.len() as GLsizei);
            },
            MeshDrawType::DrawElements => unsafe {
                gl::DrawElements(
                    self.gl_mode,
                    self.indices.len() as GLsizei,
                    gl::UNSIGNED_INT,
                    std::ptr::null(),
                );
            },
        }

        #[cfg(debug_assertions)]
        {
            let error;
            unsafe {
                error = gl::GetError();
            }
            if error != gl::NO_ERROR {
                println!("GL ERROR: {}", error);
            }
        }
        self.gl_unbind_vao();
    }

    pub fn gl_bind_vao(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
        }
    }

    pub fn gl_unbind_vao(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}

impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteBuffers(1, &self.ebo);
        };
    }
}
