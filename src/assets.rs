use crate::types::{
    Assets, Camera, DirLight, Font, Image, ImageSize, Material, Mesh, Model, PointLight, Program,
    Shader, SpotLight, Texture,
};
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
            cameras: HashMap::new(),
            pointlights: HashMap::new(),
            dirlights: HashMap::new(),
            spotlights: HashMap::new(),
            meshes: HashMap::new(),
            models: HashMap::new(),
        }
    }

    pub fn add_program(&mut self, name: &str, program: Program) {
        self.programs.insert(name.to_owned(), program);
    }
    pub fn add_image(&mut self, name: &str, image: Image) {
        self.images.insert(name.to_owned(), image);
    }
    pub fn add_texture(&mut self, name: &str, texture: Texture) {
        self.textures.insert(name.to_owned(), texture);
    }
    pub fn add_material(&mut self, name: &str, material: Material) {
        self.materials.insert(name.to_owned(), material);
    }
    pub fn add_font(&mut self, name: &str, font: Font) {
        self.fonts.insert(name.to_owned(), font);
    }
    pub fn add_camera(&mut self, name: &str, camera: Camera) {
        self.cameras.insert(name.to_owned(), camera);
    }
    pub fn add_pointlight(&mut self, name: &str, pointlight: PointLight) {
        self.pointlights.insert(name.to_owned(), pointlight);
    }
    pub fn add_dirlight(&mut self, name: &str, dirlight: DirLight) {
        self.dirlights.insert(name.to_owned(), dirlight);
    }
    pub fn add_spotlight(&mut self, name: &str, spotlight: SpotLight) {
        self.spotlights.insert(name.to_owned(), spotlight);
    }
    pub fn add_mesh(&mut self, name: &str, mesh: Mesh) {
        self.meshes.insert(name.to_owned(), mesh);
    }
    pub fn add_model(&mut self, name: &str, model: Model) {
        self.models.insert(name.to_owned(), model);
    }

    pub fn get_program(&self, name: &str) -> &Program {
        self.programs
            .get(name)
            .expect(&format!("Program '{}' not found.", name))
    }
    pub fn get_image(&self, name: &str) -> &Image {
        self.images
            .get(name)
            .expect(&format!("Image '{}' not found.", name))
    }
    pub fn get_texture(&self, name: &str) -> &Texture {
        self.textures
            .get(name)
            .expect(&format!("Texture '{}' not found.", name))
    }
    pub fn get_material(&self, name: &str) -> &Material {
        self.materials
            .get(name)
            .expect(&format!("Material '{}' not found.", name))
    }
    pub fn get_font(&self, name: &str) -> &Font {
        self.fonts
            .get(name)
            .expect(&format!("Font '{}' not found.", name))
    }
    pub fn get_camera(&self, name: &str) -> &Camera {
        self.cameras
            .get(name)
            .expect(&format!("Camera '{}' not found.", name))
    }
    pub fn get_pointlight(&self, name: &str) -> &PointLight {
        self.pointlights
            .get(name)
            .expect(&format!("PointLight '{}' not found.", name))
    }
    pub fn get_dirlight(&self, name: &str) -> &DirLight {
        self.dirlights
            .get(name)
            .expect(&format!("DirLight '{}' not found.", name))
    }
    pub fn get_spotlight(&self, name: &str) -> &SpotLight {
        self.spotlights
            .get(name)
            .expect(&format!("SpotLight '{}' not found.", name))
    }
    pub fn get_mesh(&self, name: &str) -> &Mesh {
        self.meshes
            .get(name)
            .expect(&format!("Mesh '{}' not found.", name))
    }
    pub fn get_model(&self, name: &str) -> &Model {
        self.models
            .get(name)
            .expect(&format!("Model '{}' not found.", name))
    }

    pub fn get_mut_program(&mut self, name: &str) -> &mut Program {
        self.programs
            .get_mut(name)
            .expect(&format!("Program '{}' not found.", name))
    }
    pub fn get_mut_image(&mut self, name: &str) -> &mut Image {
        self.images
            .get_mut(name)
            .expect(&format!("Image '{}' not found.", name))
    }
    pub fn get_mut_texture(&mut self, name: &str) -> &mut Texture {
        self.textures
            .get_mut(name)
            .expect(&format!("Texture '{}' not found.", name))
    }
    pub fn get_mut_material(&mut self, name: &str) -> &mut Material {
        self.materials
            .get_mut(name)
            .expect(&format!("Material '{}' not found.", name))
    }
    pub fn get_mut_font(&mut self, name: &str) -> &mut Font {
        self.fonts
            .get_mut(name)
            .expect(&format!("Font '{}' not found.", name))
    }
    pub fn get_mut_camera(&mut self, name: &str) -> &mut Camera {
        self.cameras
            .get_mut(name)
            .expect(&format!("Camera '{}' not found.", name))
    }
    pub fn get_mut_pointlight(&mut self, name: &str) -> &mut PointLight {
        self.pointlights
            .get_mut(name)
            .expect(&format!("PointLight '{}' not found.", name))
    }
    pub fn get_mut_dirlight(&mut self, name: &str) -> &mut DirLight {
        self.dirlights
            .get_mut(name)
            .expect(&format!("DirLight '{}' not found.", name))
    }
    pub fn get_mut_spotlight(&mut self, name: &str) -> &mut SpotLight {
        self.spotlights
            .get_mut(name)
            .expect(&format!("SpotLight '{}' not found.", name))
    }
    pub fn get_mut_mesh(&mut self, name: &str) -> &mut Mesh {
        self.meshes
            .get_mut(name)
            .expect(&format!("Mesh '{}' not found.", name))
    }
    pub fn get_mut_model(&mut self, name: &str) -> &mut Model {
        self.models
            .get_mut(name)
            .expect(&format!("Model '{}' not found.", name))
    }

    // FIX use for textures, materials, etc
    // FIX make a hashmap of assets to be updated
    fn _gl_register_assets(&mut self) {
        for (_, texture) in self.textures.iter_mut() {
            texture.gl_register();
        }
        for (_, material) in self.materials.iter_mut() {
            material.diffuse.gl_register();
            material.specular.gl_register();
            material.emissive.gl_register();
        }
    }

    pub fn update_assets(&mut self) {
        // OPTIMIZE .update(); iteration to use a custom HashMap / a bitset for assets to be updated
        for (_, camera) in self.cameras.iter_mut() {
            if camera.update_projection {
                camera.update();
            }
        }
        // FIX iterate through update hashmap and register gl textures
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
