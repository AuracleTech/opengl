use glfw::{Action, Key, MouseButton, WindowEvent};
use std::{collections::HashMap, sync::mpsc::Receiver};

pub struct Inputs {
    pub(crate) keys: HashMap<Key, Action>,
    pub(crate) mouse_buttons: HashMap<MouseButton, Action>,
    pub mouse_pos: Option<(f64, f64)>,
    pub mouse_scroll: Option<(f64, f64)>,
}

impl Inputs {
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
            mouse_buttons: HashMap::new(),
            mouse_pos: None,
            mouse_scroll: None,
        }
    }

    pub fn update(&mut self, events: &Receiver<(f64, WindowEvent)>) {
        self.mouse_pos = None;
        self.mouse_scroll = None;
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                WindowEvent::Key(key, _, action, _) => {
                    self.keys.insert(key, action);
                }
                WindowEvent::MouseButton(button, action, _) => {
                    self.mouse_buttons.insert(button, action);
                }
                WindowEvent::CursorPos(x, y) => {
                    self.mouse_pos = Some((x, y));
                }
                WindowEvent::Scroll(x, y) => {
                    self.mouse_scroll = Some((x, y));
                }
                // TODO make this configurable
                WindowEvent::FramebufferSize(width, height) => unsafe {
                    gl::Viewport(0, 0, width, height);
                },
                _ => {}
            }
        }
    }

    pub fn is_key_down(&self, key: Key) -> bool {
        match self.keys.get(&key) {
            Some(Action::Press) | Some(Action::Repeat) => true,
            _ => false,
        }
    }
}
