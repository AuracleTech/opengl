use cgmath::Vector2;

use crate::texture::Texture;

pub struct Character {
    pub texture: Texture,
    pub size: Vector2<i32>,
    pub bearing: Vector2<i32>,
    pub advance: i32,
}
