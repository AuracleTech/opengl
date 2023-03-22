use crate::types::{Assets, Font, Image, ImageSize, Model, Shader};
use ::image::{DynamicImage, ImageBuffer, ImageFormat};
use bincode::{deserialize, serialize};
use serde::{de::DeserializeOwned, Serialize};
use std::collections::HashMap;
use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};
mod camera;
mod font;
mod image;
mod mesh;
mod model;
mod program;
mod shader;
mod texture;

impl Assets {
    pub fn new() -> Self {
        Self {
            // OPTIMIZE use hashmap of ID u64 instead of string
            programs: HashMap::new(),
            images: HashMap::new(),
            textures: HashMap::new(),
            materials: HashMap::new(),
            fonts: HashMap::new(),
            meshes: HashMap::new(),
            cameras: HashMap::new(),
            pointlights: HashMap::new(),
            dirlights: HashMap::new(),
            spotlights: HashMap::new(),
            models: HashMap::new(),
        }
    }
}

const ASSETS_FOLDER: &str = "assets";

const FOREIGN_FOLDER: &str = "foreign";
const SHADER_FOLDER: &str = "shaders";
const NATIVE_FOLDER: &str = "soul";
const NATIVE_EXT: &str = "soul";

// OPTIMIZE there's certainly a better way, compiler should be able to optimize this
#[cfg(not(debug_assertions))]
fn assets_path() -> PathBuf {
    let exe_path = env::current_exe().expect("Failed to get current executable path.");
    let mut assets_path =
        PathBuf::from(exe_path.parent().expect("Failed to get parent directory."));
    assets_path.push(ASSETS_FOLDER);
    assets_path
}

#[cfg(debug_assertions)]
fn assets_path() -> PathBuf {
    let mut assets_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    assets_path.push(ASSETS_FOLDER);
    assets_path
}

fn get_path(folder: &str, name: &str, extension: &str) -> PathBuf {
    let path = assets_path()
        .join(folder)
        .join(name)
        .with_extension(extension);
    if !path.exists() {
        panic!("File does not exist '{:?}'", path);
    }
    path
}

// TODO load and export with a prefix from the struct name
// NOTE either this or you make a shit tons of folders, what will it be cupcake?
pub fn save<T>(name: &str, data: T)
where
    T: Serialize,
{
    let path = get_path(NATIVE_FOLDER, &name, NATIVE_EXT);
    let mut file = File::create(path).expect("Failed to create file.");
    let encoded = serialize(&data).expect("Failed to serialize data.");
    file.write_all(&encoded).expect("Failed to write to file.");
}
pub fn load<T>(name: &str) -> T
where
    T: DeserializeOwned,
{
    let path = get_path(NATIVE_FOLDER, &name, NATIVE_EXT);
    let mut file = File::open(path).expect("Failed to open file.");
    let mut encoded = Vec::new();
    file.read_to_end(&mut encoded)
        .expect("Failed to read file.");
    let serialized = deserialize::<T>(&encoded).expect("Failed to deserialize data.");
    serialized
}

pub fn load_foreign_font(name: &str, extension: &str) -> Font {
    let path = get_path(FOREIGN_FOLDER, &name, extension);
    // TODO save_image_to_png(&font.sprite.image, name).expect("Failed to save image to png");
    match extension.to_lowercase().as_str() {
        "ttf" => Font::from_ttf(path),
        _ => panic!("Unsupported font extension: {}", extension),
    }
}
pub fn load_foreign_image(name: &str, extension: &str) -> Image {
    let path = get_path(FOREIGN_FOLDER, &name, extension);
    match extension.to_lowercase().as_str() {
        "jpg" | "png" => Image::from_file(path, extension),
        _ => panic!("Unsupported image extension: {}", extension),
    }
}
pub fn load_foreign_shader(name: &str, extension: &str) -> Shader {
    let path = get_path(SHADER_FOLDER, &name, extension);
    match extension.to_lowercase().as_str() {
        "vs" => Shader::from_foreign(path, extension),
        "fs" => Shader::from_foreign(path, extension),
        _ => panic!("Unknown shader extension: '{}'.", extension),
    }
}
pub fn load_foreign_model(name: &str, extension: &str) -> Model {
    let path = get_path(FOREIGN_FOLDER, &name, extension);
    match extension.to_lowercase().as_str() {
        "glb" | "gltf" => Model::from_gltf(path),
        _ => panic!("Unsupported file extension"),
    }
}

pub fn save_image_to_png(image: &Image, name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let data = image.data.clone();
    let (width, height) = match image.size {
        ImageSize::I2D { x, y } => (x as u32, y as u32),
        _ => panic!("Only 2D images are supported."),
    };
    let path = get_path(FOREIGN_FOLDER, &name, "png");
    let image_buffer = ImageBuffer::from_raw(width, height, data).ok_or("Invalid image data")?;
    let dynamic_image = DynamicImage::ImageRgba8(image_buffer);
    dynamic_image.save_with_format(path, ImageFormat::Png)?;
    Ok(())
}
