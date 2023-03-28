use crate::types::{Direction, Position};
use cgmath::{vec3, Deg, Matrix4, SquareMatrix};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Camera {
    pub pos: Position,
    pub up: Direction,
    pub front: Direction,
    pub right: Direction,

    // TODO make a list of assets to update or something like that to avoid adding a bool to each asset
    pub update_projection: bool,
    pub projection_kind: CameraProjectionKind,
    pub projection: Matrix4<f32>,
}

// TODO remove debug
#[derive(Serialize, Deserialize, Debug)]
pub enum CameraProjectionKind {
    Perspective {
        aspect_ratio: f32,
        near: f32,
        far: f32,
        fov_y: f32, // OPTIMIZE use Degree instead of f32 ?
    },
    Orthographic {
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    },
}

impl Camera {
    pub fn new_perspective(pos: Position) -> Self {
        Self {
            pos,
            front: vec3(0.0, 0.0, -1.0),
            up: vec3(0.0, 1.0, 0.0),
            right: vec3(0.0, 0.0, 0.0),

            update_projection: true,
            projection_kind: CameraProjectionKind::Perspective {
                aspect_ratio: 16.0 / 9.0,
                fov_y: 45.0,
                near: 0.1,
                far: 100.0,
            },
            projection: Matrix4::identity(),
        }
    }

    pub fn new_orthographic(pos: Position) -> Self {
        Self {
            pos,
            front: vec3(0.0, 0.0, -1.0),
            up: vec3(0.0, 1.0, 0.0),
            right: vec3(0.0, 0.0, 0.0),

            update_projection: true,
            projection_kind: CameraProjectionKind::Orthographic {
                left: -1.0,
                right: 1.0,
                bottom: -1.0,
                top: 1.0,
                near: 0.1,
                far: 100.0,
            },
            projection: Matrix4::identity(),
        }
    }

    pub fn update(&mut self) {
        match self.projection_kind {
            CameraProjectionKind::Perspective {
                aspect_ratio,
                near,
                far,
                fov_y,
            } => {
                self.projection = cgmath::perspective(Deg(fov_y), aspect_ratio, near, far);
            }
            CameraProjectionKind::Orthographic {
                left,
                right,
                bottom,
                top,
                near,
                far,
            } => {
                self.projection = cgmath::ortho(left, right, bottom, top, near, far);
            }
        }
        self.update_projection = false;
    }
}
