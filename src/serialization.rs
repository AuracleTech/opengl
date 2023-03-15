use crate::types::{AssetCamera, CameraSerialized, Direction, Position};
use cgmath::{Matrix4, SquareMatrix};
use serde::{de::DeserializeOwned, Serialize};
use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

pub fn serialize<T>(path: PathBuf, data: T)
where
    T: Serialize,
{
    let mut file = File::create(path).expect("Failed to create file.");
    let encoded = bincode::serialize(&data).expect("Failed to serialize data.");
    file.write_all(&encoded).expect("Failed to write to file.");
}

pub fn deserialize<T>(path: &PathBuf) -> T
where
    T: Serialize + DeserializeOwned,
{
    let mut file = File::open(path).expect("Failed to open file.");
    let mut encoded = Vec::new();
    file.read_to_end(&mut encoded)
        .expect("Failed to read file.");
    let serialized = bincode::deserialize::<T>(&encoded).expect("Failed to deserialize data.");
    serialized
}

fn pos_to_vec(pos: Position) -> Vec<f32> {
    vec![pos.x, pos.y, pos.z]
}
fn dir_to_vec(dir: Direction) -> Vec<f32> {
    vec![dir.x, dir.y, dir.z]
}
fn vec_to_pos(vec: Vec<f32>) -> Position {
    Position {
        x: vec[0],
        y: vec[1],
        z: vec[2],
    }
}
fn vec_to_dir(vec: Vec<f32>) -> Direction {
    Direction {
        x: vec[0],
        y: vec[1],
        z: vec[2],
    }
}

pub fn deserialize_camera(path: &PathBuf) -> AssetCamera {
    let serialized = deserialize::<CameraSerialized>(&path);
    let filename = path
        .file_name()
        .expect("Failed to get file name.")
        .to_str()
        .expect("Failed to convert file name to str.")
        .to_string();
    AssetCamera {
        filename,
        pos: vec_to_pos(serialized.pos),
        up: vec_to_dir(serialized.up),
        front: vec_to_dir(serialized.front),
        right: vec_to_dir(serialized.right),
        update_projection: true,
        projection_kind: serialized.projection_kind,
        projection: Matrix4::identity(),
    }
}
pub fn serialize_camera(path: PathBuf, camera: AssetCamera) {
    let data = CameraSerialized {
        pos: pos_to_vec(camera.pos),
        up: dir_to_vec(camera.up),
        front: dir_to_vec(camera.front),
        right: dir_to_vec(camera.right),
        projection_kind: camera.projection_kind,
    };
    serialize(path, data);
}
