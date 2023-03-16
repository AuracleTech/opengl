mod assets {
    mod camera;
    mod font;
    mod image;
    mod material;
    mod texture;
}

#[allow(dead_code, unused_variables)]
mod mesh;
#[allow(dead_code)]
mod program;
mod serialization;
mod shader;
#[allow(dead_code)]
pub mod types; // TODO SET PRIVATE

use glfw::Context;
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use types::GLConfig;

use crate::types::RGBA;
pub use types::Revenant;

// TODO OPTIMIZE there's certainly a better way, compiler should be able to optimize this
#[cfg(not(debug_assertions))]
pub fn get_assets_path() -> PathBuf {
    let exe_path = env::current_exe().expect("Failed to get current exe path");
    let mut assets_path = PathBuf::from(exe_path.parent().expect("Failed to get parent directory"));
    assets_path.push("assets");
    assets_path
}

#[cfg(debug_assertions)]
pub fn assets_path() -> PathBuf {
    let mut assets_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    assets_path.push("assets");
    assets_path
}

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

        let mut max_vertex_attribs = 0;
        unsafe {
            gl::GetIntegerv(gl::MAX_VERTEX_ATTRIBS, &mut max_vertex_attribs);
        }

        Self {
            glfw,
            window,
            events,
            // TODO check if current_vertex_attribs <= max_vertex_attribs before initializing each vertex attributes
            gl: GLConfig { max_vertex_attribs },
            font_assets: HashMap::new(),
            texture_assets: HashMap::new(),
            material_assets: HashMap::new(),
            camera_assets: HashMap::new(),
        }
    }
}

pub fn set_clear_color(color: RGBA) {
    unsafe {
        gl::ClearColor(color.x, color.y, color.z, color.w);
    }
}
