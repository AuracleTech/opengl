use std::collections::HashMap;

use cgmath::{Point3, Vector2, Vector3, Vector4};
use gl::types::{GLsizei, GLuint};

pub type Uniaxial = f32;

pub type Position = Point3<Uniaxial>;
pub type Positions = Vec<Position>;

pub type Direction = Vector3<Uniaxial>;
pub type Directions = Vec<Direction>;

pub type Normal = f32;
pub type Normals = Vector3<Normal>;

pub type TexCoord = f32;
pub type TexCoords = Vector2<TexCoord>;

pub type Indice = u32;
pub type Indices = Vec<Indice>;

pub type RGB = Vector3<f32>;
pub type RGBA = Vector4<f32>;

pub struct Texture {
    pub id: GLuint,
    pub kind: ImageKind,
    pub format: ImageFormat,
    pub size: TextureSize,
    pub s_wrapping: Wrapping,
    pub t_wrapping: Wrapping,
    pub min_filtering: Filtering,
    pub mag_filtering: Filtering,
    pub mipmapping: bool,
}
pub type Textures = Vec<Texture>;

pub enum ImageKind {
    Diffuse,
    Specular,
    Normal,
    Height,
}

pub enum ImageFormat {
    RGBA,
    RGB,
    RG,
    R,
    Unicolor,
}

pub enum TextureSize {
    TwoD {
        width: GLsizei,
        height: GLsizei,
    },
    ThreeD {
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
    },
}

pub enum Wrapping {
    Repeat,
    MirroredRepeat,
    ClampToEdge,
    ClampToBorder,
}

pub enum Filtering {
    Nearest,
    Linear,
    NearestMipmapNearest,
    LinearMipmapNearest,
    NearestMipmapLinear,
    LinearMipmapLinear,
    // TODO add anisotropic filtering
}

pub struct Material {
    pub diffuse: Texture,
    pub specular: Texture,
    pub specular_strength: f32,
}

pub type Vertecies = Vec<Vertex>;
pub struct Vertex {
    pub positions: Positions,
    pub normals: Normals,
    pub tex_coords: TexCoords,
}

pub type Meshes = Vec<Mesh>;
pub struct Mesh {
    pub vertices: Vertecies,
    pub indices: Indices,
    pub textures: Textures,

    pub vao: GLuint,
    pub vbo: GLuint,
    pub ebo: GLuint,
}

pub struct Camera {
    pub pos: Position,

    pub up: Direction,
    pub front: Direction,
    pub right: Direction,

    pub fov_y: f32,
    pub fov_y_min: f32,
    pub fov_y_max: f32,

    pub speed: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub aim_sensitivity: f32,
    pub speed_factor: f32,
}

pub struct Character {
    pub texture: Texture,
    pub size: Vector2<i32>,
    pub bearing: Vector2<i32>,
    pub advance: i64,
}

pub struct Ascii {
    pub name: String,
    pub size: u32,
    pub chars: HashMap<char, Character>,
}

pub struct DirLight {
    pub dir: Direction,

    pub ambient: RGB,
    pub diffuse: RGB,
    pub specular: RGB,
}

pub struct PointLight {
    pub pos: Position,

    pub constant: f32,
    pub linear: f32,
    pub quadratic: f32,

    pub ambient: RGB,
    pub diffuse: RGB,
    pub specular: RGB,
}

pub struct SpotLight {
    pub pos: Position,
    pub dir: Direction,

    pub cut_off: f32,
    pub outer_cut_off: f32,

    pub constant: f32,
    pub linear: f32,
    pub quadratic: f32,

    pub ambient: RGB,
    pub diffuse: RGB,
    pub specular: RGB,
}

pub struct Shader {
    pub id: GLuint,
}

pub struct Program {
    pub id: GLuint,
}
