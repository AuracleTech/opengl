use cgmath::{Matrix4, Point3, Vector2, Vector3, Vector4};
use gl::types::{GLenum, GLsizei, GLuint};
use glfw::{Glfw, Window, WindowEvent};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::mpsc::Receiver};

// TODO replace all Point3 and Vec3 by [f32; 3]
pub type Uniaxial = f32;
pub type Position = Point3<Uniaxial>;
pub type Direction = Vector3<Uniaxial>;
pub type Normal = Vector3<Uniaxial>;
pub type TexCoords = Vector2<Uniaxial>; // OPTIMIZE use u16 if possible or even u8
pub type ColorChannel = f32;
pub type RGB = Vector3<ColorChannel>;
pub type RGBA = Vector4<ColorChannel>;
pub type Indice = u32; // OPTIMIZE use u16 if possible

pub struct Revenant {
    pub glfw: Glfw,
    pub window: Window,
    pub events: Receiver<(f64, WindowEvent)>,
    pub gl_config: GLConfig,
    pub assets: Assets,
}

pub struct Assets {
    pub programs: HashMap<String, Program>,
    pub images: HashMap<String, Image>,
    pub textures: HashMap<String, Texture>,
    pub materials: HashMap<String, Material>,
    pub fonts: HashMap<String, Font>,
    pub cameras: HashMap<String, Camera>,
    pub pointlights: HashMap<String, PointLight>,
    pub dirlights: HashMap<String, DirLight>,
    pub spotlights: HashMap<String, SpotLight>,
    pub meshes: HashMap<String, Mesh>,
    pub models: HashMap<String, Model>,
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
    Ambient,
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

#[derive(Serialize, Deserialize, Debug)]
pub enum ImageSize {
    I2D { x: GLsizei, y: GLsizei },
    I3D { x: GLsizei, y: GLsizei, z: GLsizei },
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Vertex {
    pub position: Position,
    pub normal: Normal,
    pub tex_coords: TexCoords,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mesh {
    pub gl_mode: GLenum,

    pub vertices: Vec<Vertex>,
    pub indices: Vec<Indice>,
    pub textures: Vec<Texture>,

    pub vao: GLuint, // FIX SHOULD BE PRIVATE pub(crate) maybe ?
    pub vbo: GLuint, // FIX SHOULD BE PRIVATE pub(crate) maybe ?
    pub ebo: GLuint, // FIX SHOULD BE PRIVATE pub(crate) maybe ?
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Model {
    pub meshes: Vec<Mesh>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Camera {
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
pub enum ProjectionKind {
    Perspective {
        aspect_ratio: f32,
        near: f32,
        far: f32,
        fov_y: f32, // OPTIMIZE use Degree instead of f32 ?
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

#[derive(Serialize, Deserialize, Debug)]
pub struct DirLight {
    pub dir: Direction,

    pub ambient: RGB,
    pub diffuse: RGB,
    pub specular: RGB,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PointLight {
    pub pos: Position,

    pub constant: f32,
    pub linear: f32,
    pub quadratic: f32,

    pub ambient: RGB,
    pub diffuse: RGB,
    pub specular: RGB,
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

    pub ambient: RGB,
    pub diffuse: RGB,
    pub specular: RGB,
}

pub struct Shader {
    pub gl_id: GLuint,
}

pub struct Program {
    pub gl_id: GLuint,
}

pub struct Glyph {
    pub width: i32,
    pub height: i32,
    pub sprite_x: u32,
    pub sprite_y: u32,
    pub bearing_x: i32,
    pub bearing_y: i32,
    pub advance_x: i64,
    pub advance_y: i64,
}

// TODO remove debug everywhere
#[derive(Serialize, Deserialize, Debug)]
pub struct Texture {
    pub gl_id: GLuint,
    pub image: Image,
    pub kind: TextureKind,
    pub s_wrapping: Wrapping,
    pub t_wrapping: Wrapping,
    pub min_filtering: Filtering,
    pub mag_filtering: Filtering,
    pub mipmapping: bool,
}

pub struct Font {
    pub sprite: Texture,
    pub glyphs: HashMap<char, Glyph>,
    pub width: u32,
    pub height: u32,
    pub line_height: u32,
}

// TODO remove debug everywhere
#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
    pub data: Vec<u8>,
    pub format: ImageFormat,
    pub size: ImageSize,
}

// TODO remove debug everywhere
#[derive(Serialize, Deserialize, Debug)]
pub struct Material {
    pub diffuse: Texture,
    pub specular: Texture,
    pub specular_strength: f32,
    pub emissive: Texture,
}
