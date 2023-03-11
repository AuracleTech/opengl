use cgmath::Vector2;
use cgmath::Vector3;

pub struct Vertex {
    pub positions: Vector3<f32>,
    pub normals: Vector3<f32>,
    pub tex_coords: Vector2<f32>,
}
