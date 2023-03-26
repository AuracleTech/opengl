use serde::{Deserialize, Serialize};

use crate::types::{Direction, Position, Rgb};

#[derive(Serialize, Deserialize, Debug)]
pub struct DirLight {
    pub dir: Direction,

    pub ambient: Rgb,
    pub diffuse: Rgb,
    pub specular: Rgb,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PointLight {
    pub pos: Position,

    pub constant: f32,
    pub linear: f32,
    pub quadratic: f32,

    pub ambient: Rgb,
    pub diffuse: Rgb,
    pub specular: Rgb,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpotLight {
    pub pos: Position,
    pub dir: Direction,

    pub cut_off: f32,
    pub outer_cut_off: f32,

    pub constant: f32,
    pub linear: f32,
    pub quadratic: f32,

    pub ambient: Rgb,
    pub diffuse: Rgb,
    pub specular: Rgb,
}
