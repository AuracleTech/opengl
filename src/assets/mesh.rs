use crate::types::{Indice, Mesh, Normal, Position, Program, Texture, Vertex};
use gl::types::{GLenum, GLsizei, GLsizeiptr, GLvoid};
use std::ffi::c_void;

// TODO default texture similar to garry's mod (white & black checkerboard)
impl Mesh {
    pub fn new(gl_mode: GLenum, vertices: Vec<Vertex>, indices: Vec<Indice>) -> Self {
        let mut mesh = Self {
            gl_mode,
            vertices,
            indices,
            vao: 0,
            vbo: 0,
            ebo: 0,
        };
        mesh.setup_mesh();
        mesh
    }

    pub fn setup_mesh(&mut self) {
        let size = (self.vertices.len() * std::mem::size_of::<Vertex>()) as GLsizeiptr;
        let data = self.vertices.as_ptr();
        let stride = std::mem::size_of::<Vertex>() as GLsizei;
        let offset_normals = std::mem::size_of::<Position>();
        let offset_tex_coords = std::mem::size_of::<Position>() + std::mem::size_of::<Normal>();
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
            // // vertex texture coords
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

    pub fn draw(&self, _program: &Program) {
        self.gl_bind_vao();
        self.draw_elements();
        match self.gl_mode {
            gl::TRIANGLES => (),
            gl::LINES => (),
            gl::QUADS => panic!("QUADS are not supported!"), // OPTIMIZE should be deprecated in favor of gl::TRIANGLES
            _ => panic!("Unsupported gl_mode yet!"),
        }
        self.gl_unbind();
        Texture::gl_unbind();
    }

    fn gl_bind_vao(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
        }
    }

    fn gl_unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    fn draw_elements(&self) {
        let count = self.indices.len() as GLsizei;
        unsafe {
            gl::DrawElements(self.gl_mode, count, gl::UNSIGNED_INT, std::ptr::null());
            let error = gl::GetError();
            if error != gl::NO_ERROR {
                println!("GL ERROR: {}", error);
            }
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
