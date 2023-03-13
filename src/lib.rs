extern crate cgmath;
extern crate freetype;
extern crate gl;
extern crate glfw;

use cgmath::point3;
use cgmath::vec3;
use glfw::Context;
use std::env;
use std::path::PathBuf;

mod asset;
mod character;
#[allow(dead_code, unused_variables)]
mod mesh;
#[allow(dead_code)]
mod program;
mod shader;
mod texture;
#[allow(dead_code)]
pub mod types; // TODO SET PRIVATE

use crate::types::{AssetManager, Camera, RGBA};
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

        let camera = Camera {
            pos: point3(1.84, 0.8, 3.1),
            front: vec3(0.0, 0.0, -1.0),
            up: vec3(0.0, 1.0, 0.0),
            right: vec3(0.0, 0.0, 0.0),
            fov_y: 45.0,
            fov_y_min: 1.0,
            fov_y_max: 90.0,
            speed_factor_default: 3.0,
            speed_factor_boost: 6.0,
            speed_factor: 3.0,
            speed: 0.0,
            yaw: 200.0,
            pitch: -20.0,
            aim_sensitivity: 0.03,
        };

        Self {
            glfw,
            window,
            events,
            camera,
            asset_manager: AssetManager::new(get_assets_path()),
        }
    }
}

pub fn set_clear_color(color: RGBA) {
    unsafe {
        gl::ClearColor(color.x, color.y, color.z, color.w);
    }
}

#[cfg(not(debug_assertions))]
fn get_assets_path() -> PathBuf {
    let exe_path = env::current_exe().expect("Failed to get current exe path");
    let mut assets_path = PathBuf::from(exe_path.parent().expect("Failed to get parent directory"));
    assets_path.push("assets");
    assets_path
}

#[cfg(debug_assertions)]
fn get_assets_path() -> PathBuf {
    let mut assets_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    assets_path.push("assets");
    assets_path
}
