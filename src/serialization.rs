use crate::types::{Direction, Position};
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

pub fn deserialize<T>(path: PathBuf) -> T
where
    T: DeserializeOwned,
{
    dbg!("Deserializing: ");
    dbg!(&path);
    let mut file = File::open(path).expect("Failed to open file.");
    let mut encoded = Vec::new();
    file.read_to_end(&mut encoded)
        .expect("Failed to read file.");
    let serialized = bincode::deserialize::<T>(&encoded).expect("Failed to deserialize data.");
    serialized
}

pub fn pos_to_vec(pos: Position) -> Vec<f32> {
    vec![pos.x, pos.y, pos.z]
}
pub fn dir_to_vec(dir: Direction) -> Vec<f32> {
    vec![dir.x, dir.y, dir.z]
}
pub fn vec_to_pos(vec: Vec<f32>) -> Position {
    Position {
        x: vec[0],
        y: vec[1],
        z: vec[2],
    }
}
pub fn vec_to_dir(vec: Vec<f32>) -> Direction {
    Direction {
        x: vec[0],
        y: vec[1],
        z: vec[2],
    }
}
