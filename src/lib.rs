pub mod assets;
#[cfg(feature = "pillow")]
mod benchmark;
mod cubemap;
mod framebuffer;
mod inputs;
mod types;
use gl::types::{GLenum, GLuint};
use glfw::{Context, Glfw, PixelImage, Window, WindowEvent};
use inputs::Inputs;
use std::{
    env,
    ffi::{c_void, CStr},
    sync::mpsc::Receiver,
};

// TODO flexible window size
const WIN_DIM_X: u32 = 1600;
const WIN_DIM_Y: u32 = 900;

pub struct Revenant {
    pub(crate) glfw: Glfw,
    pub(crate) window: Window,
    pub(crate) events: Receiver<(f64, WindowEvent)>,
    pub(crate) frame_time: f64,
    pub(crate) frame_time_last: f64,
    pub frame_time_delta: f64,
    pub frame_count_total: u64,
    pub inputs: Inputs,
}

extern "system" fn debug_callback(
    source: GLenum,
    gltype: GLenum,
    id: GLuint,
    severity: GLenum,
    _length: i32,
    message: *const i8,
    _user_param: *mut c_void,
) {
    let source_str = match source {
        gl::DEBUG_SOURCE_API => "API",
        gl::DEBUG_SOURCE_WINDOW_SYSTEM => "Window System",
        gl::DEBUG_SOURCE_SHADER_COMPILER => "Shader Compiler",
        gl::DEBUG_SOURCE_THIRD_PARTY => "Third Party",
        gl::DEBUG_SOURCE_APPLICATION => "Application",
        _ => "Other",
    };
    let type_str = match gltype {
        gl::DEBUG_TYPE_ERROR => "Error",
        gl::DEBUG_TYPE_DEPRECATED_BEHAVIOR => "Deprecated Behavior",
        gl::DEBUG_TYPE_UNDEFINED_BEHAVIOR => "Undefined Behavior",
        gl::DEBUG_TYPE_PORTABILITY => "Portability",
        gl::DEBUG_TYPE_PERFORMANCE => "Performance",
        gl::DEBUG_TYPE_MARKER => "Marker",
        gl::DEBUG_TYPE_PUSH_GROUP => "Push Group",
        gl::DEBUG_TYPE_POP_GROUP => "Pop Group",
        _ => "Other",
    };
    let severity_str = match severity {
        gl::DEBUG_SEVERITY_HIGH => "High",
        gl::DEBUG_SEVERITY_MEDIUM => "Medium",
        gl::DEBUG_SEVERITY_LOW => "Low",
        _ => "Notification",
    };
    unsafe {
        let message_str = CStr::from_ptr(message).to_str().unwrap();
        println!(
            "OpenGL Debug: [{}] [{}] [{}] ({}) - {}",
            source_str, type_str, id, severity_str, message_str
        );
    }
}

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

        let mut revenant = Self {
            glfw,
            window,
            events,
            // OPTIMIZE custom allocator? May not necessary
            frame_time: 0.0,
            frame_time_last: 0.0,
            frame_time_delta: 0.0,
            frame_count_total: 0,
            inputs: Inputs::new(),
        };

        revenant.gl_init();
        // TODO make this a setting or function
        revenant.set_position_center();
        // TODO make this configurable

        revenant
    }

    pub fn set_window_icon(&mut self, images: Vec<PixelImage>) {
        self.window.set_icon_from_pixels(images);
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

    pub fn should_close(&mut self) -> bool {
        self.window.swap_buffers();
        self.frame_count_total += 1;
        self.frame_time_last = self.frame_time;
        self.frame_time = self.glfw.get_time();
        self.frame_time_delta = self.frame_time - self.frame_time_last;
        self.glfw.poll_events();
        self.inputs.update(&self.events);
        self.window.should_close()
    }

    pub fn set_should_close(&mut self, should_close: bool) {
        self.window.set_should_close(should_close);
    }

    pub fn cycle_polygon_mode(&mut self) {
        let mut polygon_mode: [i32; 2] = [0; 2];
        unsafe {
            gl::GetIntegerv(gl::POLYGON_MODE, polygon_mode.as_mut_ptr());
        }
        let polygon_mode = match polygon_mode[0] as u32 {
            gl::FILL => gl::LINE,
            gl::LINE => gl::POINT,
            gl::POINT => gl::FILL,
            _ => panic!("Unknown polygon mode"),
        };
        unsafe {
            gl::PolygonMode(gl::FRONT_AND_BACK, polygon_mode);
        }
        let text = match polygon_mode {
            gl::FILL => "FILL",
            gl::LINE => "LINE",
            gl::POINT => "POINT",
            _ => panic!("Unknown polygon mode"),
        };
        println!("Polygon mode: {}", text);
    }

    #[inline]
    fn gl_init(&mut self) {
        // TODO make all this configurable
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Enable(gl::STENCIL_TEST);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        #[cfg(feature = "pillow")]
        unsafe {
            gl::Enable(gl::DEBUG_OUTPUT);
            gl::DebugMessageCallback(
                Some(
                    debug_callback
                        as extern "system" fn(
                            GLenum,
                            GLenum,
                            GLuint,
                            GLenum,
                            i32,
                            *const i8,
                            *mut c_void,
                        ),
                ),
                std::ptr::null(),
            );
        }
    }
}
