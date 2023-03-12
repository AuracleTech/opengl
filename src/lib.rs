extern crate cgmath;
extern crate freetype;
extern crate gl;
extern crate glfw;

use std::collections::HashMap;
use std::sync::mpsc::Receiver;

use cgmath::{ortho, perspective, vec3, vec4, Angle, Deg, Matrix4, SquareMatrix, Vector3};
use cgmath::{point3, prelude::*};
use gl::types::*;
use glfw::{Context, Glfw, Window, WindowEvent};
use types::Fonts;

mod asset;
mod character;
mod font;
#[allow(dead_code)]
mod mesh;
#[allow(dead_code)]
mod program;
mod shader;
mod texture;
#[allow(dead_code)]
pub mod types; // TODO SET PRIVATE

use crate::types::{
    AssetManager, Camera, DirLight, Filtering, Font, ImageKind, Material, PointLight, Position,
    Program, Shader, SpotLight, Texture, Wrapping, RGBA,
};

pub struct Revenant {
    pub glfw: Glfw,
    pub window: Window,
    pub events: Receiver<(f64, WindowEvent)>,
    pub camera: Camera,
    pub fonts: Fonts,
    pub asset_manager: AssetManager,
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

        // TODO check if current_vertex_attribs <= max_vertex_attribs before initializing each vertex attributes
        let mut max_vertex_attribs = 0;
        unsafe {
            gl::GetIntegerv(gl::MAX_VERTEX_ATTRIBS, &mut max_vertex_attribs);
        }

        let camera = Camera {
            pos: point3(0.0, 0.0, 3.0),
            front: vec3(0.0, 0.0, -1.0),
            up: vec3(0.0, 1.0, 0.0),
            right: vec3(0.0, 0.0, 0.0),
            speed_factor: 2.0,
            fov_y: 45.0,
            fov_y_min: 1.0,
            fov_y_max: 90.0,
            speed: 0.0,
            yaw: -90.0,
            pitch: 0.0,
            aim_sensitivity: 0.03,
        };

        Self {
            glfw,
            window,
            events,
            camera,
            fonts: HashMap::new(),
            asset_manager: AssetManager::new(),
        }
    }

    pub fn set_clear_color(color: RGBA) {
        unsafe {
            gl::ClearColor(color.x, color.y, color.z, color.w);
        }
    }
}
