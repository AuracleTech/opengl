#[allow(dead_code)]
pub mod asset; // TODO SET PRIVATE maybe
mod camera;
mod character;
#[allow(dead_code, unused_variables)]
mod mesh;
#[allow(dead_code)]
mod program;
mod serialization;
mod shader;
mod texture;
#[allow(dead_code)]
pub mod types; // TODO SET PRIVATE

use glfw::Context;
use std::env;

use crate::types::{AssetManager, RGBA};
pub use types::Revenant;

impl Revenant {
    pub fn new(width: u32, height: u32) -> Self {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("Failed to initialize GLFW.");

        let (mut window, events) = glfw
            .create_window(
                width,
                height,
                env!("CARGO_PKG_NAME"),
                glfw::WindowMode::Windowed,
            )
            .expect("Failed to create GLFW window.");

        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        window.set_framebuffer_size_polling(true);
        window.set_key_polling(true);
        window.set_scroll_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_cursor_mode(glfw::CursorMode::Disabled);
        window.make_current();

        // TODO check if current_vertex_attribs <= max_vertex_attribs before initializing each vertex attributes
        let mut max_vertex_attribs = 0;
        unsafe {
            gl::GetIntegerv(gl::MAX_VERTEX_ATTRIBS, &mut max_vertex_attribs);
        }

        Self {
            glfw,
            window,
            events,
            asset_manager: AssetManager::new(),
        }
    }
}

pub fn set_clear_color(color: RGBA) {
    unsafe {
        gl::ClearColor(color.x, color.y, color.z, color.w);
    }
}
