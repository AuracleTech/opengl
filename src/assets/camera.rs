use crate::types::Position;
use cgmath::{point3, vec3, Deg, EuclideanSpace, Matrix4, Quaternion, Rotation3, SquareMatrix};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Camera {
    pub pos: Position,
    pub quat: Quaternion<f32>,
    pub projection_kind: CameraProjectionKind,
    pub projection: Matrix4<f32>,
    // OPTIMIZE one view instead of two
    pub view: Matrix4<f32>,
    pub view_skybox: Matrix4<f32>,
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
    pub fn perspective(pos: Position) -> Self {
        let mut camera = Self::default();
        camera.pos = pos;
        camera
    }

    pub fn orthographic(pos: Position) -> Self {
        let mut camera = Self::default();
        camera.pos = pos;
        camera.projection_kind = CameraProjectionKind::Orthographic {
            left: -1.0,
            right: 1.0,
            bottom: -1.0,
            top: 1.0,
            near: 0.1,
            far: 100.0,
        };
        camera
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
        self.view = Matrix4::from(self.quat.conjugate())
            * Matrix4::from_translation(self.pos.to_vec() * -1.0);
        self.view_skybox = Matrix4::from(self.quat.conjugate());
    }
}

impl Default for Camera {
    fn default() -> Self {
        let mut camera = Self {
            pos: point3(0.0, 0.0, 0.0),
            quat: Quaternion::from_axis_angle(vec3(0.0, 1.0, 0.0), Deg(0.0)),
            projection_kind: CameraProjectionKind::Perspective {
                aspect_ratio: 16.0 / 9.0,
                fov_y: 45.0,
                near: 0.1,
                far: 100.0,
            },
            projection: Matrix4::identity(),
            view: Matrix4::identity(),
            view_skybox: Matrix4::identity(),
        };
        camera.update();
        camera
    }
}
