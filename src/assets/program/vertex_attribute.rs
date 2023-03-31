use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VertexAttribute {
    pub(crate) name: String,
    pub(crate) location: i32,
}

impl VertexAttribute {
    // pub fn get_all_vertex_attributes(program_gl_id: GLuint) -> Vec<Self> {
    //     let mut vertex_attributes = Vec::new();

    //     let mut vertex_attribute_count = 0;

    //     unsafe {
    //         gl::GetProgramiv(
    //             program_gl_id,
    //             gl::ACTIVE_ATTRIBUTES,
    //             &mut vertex_attribute_count,
    //         );
    //     }

    //     for i in 0..vertex_attribute_count {
    //         let vertex_attribute = VertexAttribute::new(program_gl_id, i as u32);
    //         vertex_attributes.push(vertex_attribute);
    //     }

    //     vertex_attributes
    // }

    // TODO verify code below
    // pub fn new(program_gl_id: GLuint, index: GLuint) -> Self {
    //     let mut name = [0 as GLchar; 256];
    //     let mut name_length = 0;
    //     let mut size = 0;
    //     let mut attribute_type = 0;

    //     unsafe {
    //         gl::GetActiveAttrib(
    //             program_gl_id,
    //             index,
    //             256,
    //             &mut name_length,
    //             &mut size,
    //             &mut attribute_type,
    //             name.as_mut_ptr(),
    //         );
    //     }

    //     let name = unsafe { std::ffi::CStr::from_ptr(name.as_ptr()) }
    //         .to_str()
    //         .unwrap()
    //         .to_string();

    //     let location =
    //         unsafe { gl::GetAttribLocation(program_gl_id, name.as_ptr() as *const GLchar) };

    //     Self { name, location }
    // }
}
