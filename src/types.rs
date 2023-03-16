use cgmath::{Matrix4, Point3, Vector2, Vector3, Vector4};
use gl::types::{GLsizei, GLuint};
use glfw::{Glfw, Window, WindowEvent};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::mpsc::Receiver};

pub type Uniaxial = f32;
pub type Position = Point3<Uniaxial>;
pub type Direction = Vector3<Uniaxial>;
pub type Normal = f32;
pub type TexCoord = f32;
pub type Indice = u32;
pub type RGB = Vector3<f32>;
pub type RGBA = Vector4<f32>;

pub struct Revenant {
    pub glfw: Glfw,
    pub window: Window,
    pub events: Receiver<(f64, WindowEvent)>,
    pub gl: GLConfig,
    pub font_assets: HashMap<String, AssetFont>,
    pub texture_assets: HashMap<String, AssetTexture>,
    pub material_assets: HashMap<String, AssetMaterial>,
    pub camera_assets: HashMap<String, AssetCamera>,
}

pub struct GLConfig {
    pub max_vertex_attribs: i32,
}

// TODO remove debug everywhere
#[derive(Serialize, Deserialize, Debug)]
pub enum TextureKind {
    Diffuse,
    Specular,
    Normal,
    Height,
    Emissive,
}

// TODO remove debug everywhere
#[derive(Serialize, Deserialize, Debug)]
pub enum ImageFormat {
    RGBA,
    RGB,
    RG,
    R,
    Unicolor,
}

#[derive(Debug)]
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

// TODO remove debug everywhere
#[derive(Serialize, Deserialize, Debug)]
pub enum Wrapping {
    Repeat,
    MirroredRepeat,
    ClampToEdge,
    ClampToBorder,
}

// TODO remove debug everywhere
#[derive(Serialize, Deserialize, Debug)]
pub enum Filtering {
    Nearest,
    Linear,
    NearestMipmapNearest,
    LinearMipmapNearest,
    NearestMipmapLinear,
    LinearMipmapLinear,
    // TODO add anisotropic filtering
}

pub struct Vertex {
    pub positions: Vec<Position>,
    pub normals: Vec<Normal>,
    pub tex_coords: Vec<TexCoord>,
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<Indice>,
    pub textures: Vec<AssetTexture>,

    pub vao: GLuint,
    pub vbo: GLuint,
    pub ebo: GLuint,
}

pub struct AssetCamera {
    pub name: String,
    pub pos: Position,
    pub up: Direction,
    pub front: Direction,
    pub right: Direction,

    // TODO make a list of assets to update or something like that to avoid adding a bool to each asset
    pub update_projection: bool,
    pub projection_kind: ProjectionKind,
    pub projection: Matrix4<f32>,
}

// TODO remove debug
#[derive(Serialize, Deserialize, Debug)]
pub struct CameraSerialized {
    pub pos: Vec<f32>,
    pub up: Vec<f32>,
    pub front: Vec<f32>,
    pub right: Vec<f32>,
    pub projection_kind: ProjectionKind,
}

// TODO remove debug
#[derive(Serialize, Deserialize, Debug)]
pub enum ProjectionKind {
    Perspective {
        aspect_ratio: f32,
        near: f32,
        far: f32,
        fov_y: f32, // TODO OPTIMIZATION use Degree
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

pub struct Character {
    pub texture: AssetTexture,
    pub size: Vector2<i32>,
    pub bearing: Vector2<i32>,
    pub advance: i64,
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

pub struct AssetFont {
    pub name: String,
    pub size: u32,
    pub chars: HashMap<char, Character>,
}

#[derive(Debug)]
pub struct AssetTexture {
    pub name: String,
    pub gl_id: GLuint,

    pub image: AssetImage,

    pub kind: TextureKind,
    pub s_wrapping: Wrapping,
    pub t_wrapping: Wrapping,
    pub min_filtering: Filtering,
    pub mag_filtering: Filtering,
    pub mipmapping: bool,
}

// TODO remove debug everywhere
#[derive(Serialize, Deserialize, Debug)]
pub struct AssetTextureSerialized {
    pub filename: String,
    pub kind: TextureKind,
    pub s_wrapping: Wrapping,
    pub t_wrapping: Wrapping,
    pub min_filtering: Filtering,
    pub mag_filtering: Filtering,
    pub mipmapping: bool,
}

#[derive(Debug)]
pub struct AssetImage {
    pub filename: String,
    pub data: Vec<u8>,
    pub format: ImageFormat,
    pub size: TextureSize,
}

#[derive(Debug)]
pub struct AssetMaterial {
    pub name: String,
    pub diffuse: AssetTexture,
    pub specular: AssetTexture,
    pub specular_strength: f32,
    pub emissive: AssetTexture,
}

// TODO remove debug everywhere
#[derive(Serialize, Deserialize, Debug)]
pub struct AssetMaterialSerialized {
    pub diffuse: String,
    pub specular: String,
    pub specular_strength: f32,
    pub emissive: String,
}
