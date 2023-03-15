use cgmath::{perspective, Deg};

use crate::types::{AssetCamera, ProjectionKind};

impl AssetCamera {
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
                left: _,
                right: _,
                bottom: _,
                top: _,
                near: _,
                far: _,
            } => {
                self.update_projection = false;
                // TODO
                // FIX
            }
        }
    }
}
