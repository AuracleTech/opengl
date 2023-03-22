use crate::types::{Indice, Mesh, Program, Texture, TextureKind, Vertex};
use gl::types::{GLsizeiptr, GLvoid};

// FIX TODO default texture similar to garry's mod (white & black checkerboard)
impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<Indice>, textures: Vec<Texture>) -> Self {
        let mut mesh = Self {
            vertices,
            indices,
            textures,
            vao: 0,
            vbo: 0,
            ebo: 0,
        };
        mesh.setup_mesh();
        mesh
    }

    pub fn setup_mesh(&mut self) {
        unsafe {
            // VBO (Vertex Buffer Object)
            gl::GenBuffers(1, &mut self.vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.vertices.len() * std::mem::size_of::<Vertex>()) as GLsizeiptr,
                self.vertices.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );

            // VAO (Vertex Array Object)
            gl::GenVertexArrays(1, &mut self.vao);
            gl::BindVertexArray(self.vao);
            // vertex positions
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<Vertex>() as i32,
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(0);
            // vertex normals
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<Vertex>() as i32,
                (3 * std::mem::size_of::<f32>()) as *const GLvoid,
            );
            gl::EnableVertexAttribArray(1);
            // vertex texture coords
            gl::VertexAttribPointer(
                2,
                2,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<Vertex>() as i32,
                (6 * std::mem::size_of::<f32>()) as *const GLvoid,
            );
            gl::EnableVertexAttribArray(2);

            // EBO (Element Buffer Object)
            gl::GenBuffers(1, &mut self.ebo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (self.indices.len() * std::mem::size_of::<Indice>()) as GLsizeiptr,
                self.indices.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );

            gl::BindVertexArray(0); // OPTIMIZE TODO is this necessary?
        }
    }

    pub fn draw(&self, program: &Program) {
        let mut diffuse_number = 0;
        let mut specular_number = 0;
        let mut emissive_number = 0;
        for (i, texture) in self.textures.iter().enumerate() {
            let number;
            let name = match texture.kind {
                TextureKind::Diffuse => {
                    diffuse_number += 1;
                    number = diffuse_number;
                    "diffuse"
                }
                TextureKind::Specular => {
                    specular_number += 1;
                    number = specular_number;
                    "specular"
                }
                TextureKind::Emissive => {
                    emissive_number += 1;
                    number = emissive_number;
                    "emissive"
                }
                _ => panic!("Unsupported texture kind yet!"),
            };
            texture.gl_bind(i as u32);
            program.set_uniform_int(&format!("material.{}{}", name, number), i as i32);
        }
        self.gl_bind();
        self.draw_elements();
        Mesh::gl_unbind();
        Texture::gl_unbind();
    }

    fn gl_bind(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
        }
    }

    fn gl_unbind() {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    fn draw_elements(&self) {
        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                self.indices.len() as i32,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
        }
    }
}
