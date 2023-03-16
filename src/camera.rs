use crate::{
    assets_path,
    serialization::{deserialize, dir_to_vec, pos_to_vec, serialize, vec_to_dir, vec_to_pos},
    types::{AssetCamera, CameraSerialized, ProjectionKind},
};
use cgmath::{perspective, Deg, Matrix4, SquareMatrix};

const EXT: &str = "camera";
const PATH: &str = "cameras";

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

    pub fn load(name: String) -> Self {
        let path = assets_path().join(PATH).join(&name).with_extension(EXT);
        let data = deserialize::<CameraSerialized>(path);
        Self {
            name,
            pos: vec_to_pos(data.pos),
            up: vec_to_dir(data.up),
            front: vec_to_dir(data.front),
            right: vec_to_dir(data.right),
            update_projection: true,
            projection_kind: data.projection_kind,
            projection: Matrix4::identity(),
        }
    }
    pub fn save(self) {
        let path = assets_path().join(PATH).join(self.name).with_extension(EXT);
        let data = CameraSerialized {
            pos: pos_to_vec(self.pos),
            up: dir_to_vec(self.up),
            front: dir_to_vec(self.front),
            right: dir_to_vec(self.right),
            projection_kind: self.projection_kind,
        };
        serialize(path, data);
    }
}
