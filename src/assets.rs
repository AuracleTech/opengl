use ::image::{DynamicImage, ImageBuffer, ImageFormat};
use bincode::{deserialize, serialize};
use serde::{de::DeserializeOwned, Serialize};
use std::collections::HashMap;
use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

pub mod camera;
pub mod font;
pub mod image;
pub mod light;
pub mod material;
pub mod mesh;
pub mod model;
pub mod program;
pub mod shader;
pub mod texture;
use self::camera::Camera;
use self::font::Font;
use self::image::Image;
use self::light::{DirLight, PointLight, SpotLight};
use self::mesh::Mesh;
use self::model::Model;
use self::program::Program;
use self::shader::Shader;
use self::texture::Texture;

// OPTIMIZE use hashmap of ID number instead of string
pub struct Assets {
    pub(crate) images: HashMap<String, Image>,
    pub(crate) textures: HashMap<String, Texture>,
    pub(crate) fonts: HashMap<String, Font>,
    pub(crate) cameras: HashMap<String, Camera>,
    pub(crate) pointlights: HashMap<String, PointLight>,
    pub(crate) dirlights: HashMap<String, DirLight>,
    pub(crate) spotlights: HashMap<String, SpotLight>,
    pub(crate) meshes: HashMap<String, Mesh>,
    pub(crate) models: HashMap<String, Model>,
    pub(crate) shaders: HashMap<String, Shader>,
    pub(crate) programs: HashMap<String, Program>,
}

impl Assets {
    pub fn new() -> Self {
        Self {
            images: HashMap::new(),
            textures: HashMap::new(),
            fonts: HashMap::new(),
            cameras: HashMap::new(),
            pointlights: HashMap::new(),
            dirlights: HashMap::new(),
            spotlights: HashMap::new(),
            meshes: HashMap::new(),
            models: HashMap::new(),
            shaders: HashMap::new(),
            programs: HashMap::new(),
        }
    }

    // SECTION NEW

    pub fn new_camera(&mut self, name: &str, camera: Camera) {
        self.cameras.insert(name.to_owned(), camera);
    }
    pub fn new_program(&mut self, name: &str, shader_names: Vec<&str>) {
        let mut shaders_gl_ids = Vec::new();
        for shader_name in shader_names {
            shaders_gl_ids.push(self.get_shader(shader_name).gl_id);
        }
        let program = Program::new(shaders_gl_ids);
        self.programs.insert(name.to_owned(), program);
    }

    // SECTION NEW FOREIGN

    pub fn new_font_foreign(&mut self, name: &str, extension: &str) -> &Font {
        let path = get_path(FOREIGN_FOLDER, &name, extension);
        // TODO save_image_to_png(&font.sprite.image, name).expect("Failed to save image to png");
        let font = match extension.to_lowercase().as_str() {
            "ttf" => Font::from_ttf(path),
            _ => panic!("Unsupported font extension: {}", extension),
        };
        self.fonts.insert(name.to_owned(), font);
        self.get_font(name)
    }
    pub fn new_image_foreign(&mut self, name: &str, extension: &str) -> &Image {
        let path = get_path(FOREIGN_FOLDER, &name, extension);
        let image = match extension.to_lowercase().as_str() {
            "jpg" | "png" => Image::from_file(path, extension),
            _ => panic!("Unsupported image extension: {}", extension),
        };
        self.images.insert(name.to_owned(), image);
        self.get_image(name)
    }
    pub fn new_shader_foreign(&mut self, name: &str, extension: &str) -> &Shader {
        let path = get_path(SHADER_FOLDER, &name, extension);
        let shader = match extension.to_lowercase().as_str() {
            "vs" => Shader::from_foreign(path, extension),
            "fs" => Shader::from_foreign(path, extension),
            _ => panic!("Unknown shader extension: '{}'.", extension),
        };
        self.shaders
            .insert(format!("{}_{}", name, extension), shader);
        self.get_shader(&format!("{}_{}", name, extension))
    }
    pub fn new_model_foreign(&mut self, name: &str, extension: &str) -> &Model {
        let path = get_path(FOREIGN_FOLDER, &name, extension);
        let model = match extension.to_lowercase().as_str() {
            "glb" | "gltf" => Model::from_gltf(path),
            _ => panic!("Unsupported file extension"),
        };
        self.models.insert(name.to_owned(), model);
        self.get_model(name)
    }

    // SECTION GET

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
    pub fn get_shader(&self, name: &str) -> &Shader {
        self.shaders
            .get(name)
            .expect(&format!("Shader '{}' not found.", name))
    }
    pub fn get_program(&self, name: &str) -> &Program {
        self.programs
            .get(name)
            .expect(&format!("Program '{}' not found.", name))
    }

    // SECTION GET MUT

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

pub fn save_image_to_png(image: &Image, name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let data = image.data.clone();
    let path = get_path(FOREIGN_FOLDER, &name, "png");
    let image_buffer =
        ImageBuffer::from_raw(image.width, image.height, data).ok_or("Invalid image data")?;
    let dynamic_image = DynamicImage::ImageRgba8(image_buffer);
    dynamic_image.save_with_format(path, ImageFormat::Png)?;
    Ok(())
}

pub fn save_json<T>(name: &str, data: T)
where
    T: Serialize,
{
    let path = get_path(NATIVE_FOLDER, &name, "json");
    let serialized = serde_json::to_string(&data).expect("Failed to serialize data.");
    std::fs::write(path, serialized).expect("Failed to write to file.");
}
