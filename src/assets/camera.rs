use crate::types::{Camera, ProjectionKind};
use cgmath::{ortho, perspective, Deg};

impl Camera {
    pub fn update(&mut self) {
        match self.projection_kind {
            ProjectionKind::Perspective {
                aspect_ratio,
                near,
                far,
                fov_y,
            } => {
                self.update_projection = false;
                self.projection = perspective(Deg(fov_y), aspect_ratio, near, far);
            }
            ProjectionKind::Orthographic {
                left,
                right,
                bottom,
                top,
                near,
                far,
            } => {
                self.update_projection = false;
                self.projection = ortho(left, right, bottom, top, near, far);
            }
        }
    }
}
