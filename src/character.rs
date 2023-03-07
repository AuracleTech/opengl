use crate::texture::Texture;
use cgmath::Vector2;

pub struct Character {
    pub texture: Texture,
    pub size: Vector2<i32>,
    pub bearing: Vector2<i32>,
    pub advance: i32,
}
