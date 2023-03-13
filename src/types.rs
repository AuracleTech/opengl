use std::{collections::HashMap, path::PathBuf, sync::mpsc::Receiver};

use cgmath::{Point3, Vector2, Vector3, Vector4};
use gl::types::{GLsizei, GLuint};
use glfw::{Glfw, Window, WindowEvent};
use image::DynamicImage;
use serde::{Deserialize, Serialize};

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
    pub camera: Camera,
    pub asset_manager: AssetManager,
}

#[derive(Debug)]
pub enum ImageKind {
    Diffuse,
    Specular,
    Normal,
    Height,
    Emissive,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum Wrapping {
    Repeat,
    MirroredRepeat,
    ClampToEdge,
    ClampToBorder,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct AssetImage2D {
    pub filename: String,
    pub image: DynamicImage,
}

pub struct AssetFont {
    pub filename: String,
    pub size: u32,
    pub chars: HashMap<char, Character>,
}

#[derive(Debug)]
pub struct AssetTexture {
    pub filename: String,
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

#[derive(Debug)]
pub struct AssetMaterial {
    pub filename: String,
    pub diffuse: AssetTexture,
    pub specular: AssetTexture,
    pub specular_strength: f32,
    pub emissive: AssetTexture,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssetMaterialSerialized {
    pub diffuse: String,
    pub specular: String,
    pub specular_strength: f32,
    pub emissive: String,
}

pub struct AssetManager {
    pub image_assets: HashMap<String, AssetImage2D>,
    pub image_assets_path: PathBuf,
    pub font_assets: HashMap<String, AssetFont>,
    pub font_assets_path: PathBuf,
    pub texture_assets: HashMap<String, AssetTexture>,
    pub texture_assets_path: PathBuf,
    pub material_assets: HashMap<String, AssetMaterial>,
    pub material_assets_path: PathBuf,
    pub assets_path: PathBuf,
}
