pub mod assets;
#[allow(dead_code)]
pub mod types; // TODO SET PRIVATE
use glfw::Context;
use std::env;
mod inputs;
pub use types::Revenant;
use types::{Assets, GLConfig, ImageSize, Inputs};

// TODO flexible window size
const WIN_DIM_X: u32 = 1600;
const WIN_DIM_Y: u32 = 900;

impl Revenant {
    pub fn new() -> Self {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("Failed to initialize GLFW.");

        // requiest core profile 4.6
        glfw.window_hint(glfw::WindowHint::ContextVersionMajor(4));
        glfw.window_hint(glfw::WindowHint::ContextVersionMinor(6));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));

        // TODO custom window name
        // TODO custom window size, start position, fullscreen / borderless, etc
        let (mut window, events) = glfw
            .create_window(
                WIN_DIM_X,
                WIN_DIM_Y,
                env!("CARGO_PKG_NAME"),
                glfw::WindowMode::Windowed,
            )
            .expect("Failed to create GLFW window.");

        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        // Unlock framerate
        // TODO make this a setting
        glfw.set_swap_interval(glfw::SwapInterval::None);

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

        let gl_version = window.get_context_version();

        let mut revenant = Self {
            glfw,
            window,
            events,
            gl_config: GLConfig {
                max_vertex_attribs,
                gl_version,
            },
            // OPTIMIZE custom allocator? Maybe not necessary
            assets: Assets::new(),
            inputs: Inputs::new(),
            frame_time: 0.0,
            frame_time_last: 0.0,
            frame_time_delta: 0.0,
        };

        revenant.gl_init();

        revenant.set_window_icon();
        // TODO make this a setting or function
        revenant.set_position_center();
        // TODO make this configurable
        revenant
    }

    fn set_window_icon(&mut self) {
        // Set window icon
        // TODO make this a setting & serialize icon
        let icon_asset = assets::load_foreign_image("icon", "png");
        let mut icon_pixels: Vec<u32> = vec![];
        for chunk in icon_asset.data.chunks_exact(4) {
            let u32_value = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
            icon_pixels.push(u32_value);
        }
        let (width, height) = match icon_asset.size {
            ImageSize::I2D { x, y } => (x, y),
            _ => panic!("Icon size is not 2D."),
        };
        let mut icons = Vec::new();
        icons.push(glfw::PixelImage {
            width: width as u32,
            height: height as u32,
            pixels: icon_pixels,
        });
        self.window.set_icon_from_pixels(icons);
    }

    fn set_position_center(&mut self) {
        const WIN_DIM_X: u32 = 1600;
        const WIN_DIM_Y: u32 = 900;
        // TODO get this on the fly
        const SCREEN_DIM_X: u32 = 1920;
        const SCREEN_DIM_Y: u32 = 1080;
        // Center window
        self.window.set_pos(
            (SCREEN_DIM_X - WIN_DIM_X) as i32 / 2,
            (SCREEN_DIM_Y - WIN_DIM_Y) as i32 / 2,
        );
    }

    fn gl_init(&mut self) {
        // TODO make all this configurable
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            // TODO verify this AI mess
            // gl::Enable(gl::DEPTH_TEST);
            // gl::DepthFunc(gl::LESS);
            // gl::Enable(gl::CULL_FACE);
            // gl::CullFace(gl::BACK);
            // gl::FrontFace(gl::CCW);
        }
    }

    pub fn should_close(&mut self) -> bool {
        let should_close = self.window.should_close();
        self.glfw.poll_events();
        self.assets.update_assets();
        self.inputs.update(&self.events);
        should_close
    }

    pub fn set_should_close(&mut self, should_close: bool) {
        self.window.set_should_close(should_close);
    }

    pub fn start_frame(&mut self) {
        // TODO make this configurable
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    pub fn end_frame(&mut self) {
        self.window.swap_buffers();
        self.frame_time_last = self.frame_time;
        self.frame_time = self.glfw.get_time();
        self.frame_time_delta = self.frame_time - self.frame_time_last;
    }
}
