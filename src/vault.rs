use crate::types::{Font, Image, Shader};
use bincode::{deserialize, serialize};
use serde::{de::DeserializeOwned, Serialize};
use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

const VAULT_FOLDER: &str = "vault";

const FOREIGN_FOLDER: &str = "foreign";
const SHADER_FOLDER: &str = "shaders";
const NATIVE_FOLDER: &str = "soul";
const NATIVE_EXT: &str = "soul";

// TODO OPTIMIZE there's certainly a better way, compiler should be able to optimize this
#[cfg(not(debug_assertions))]
fn assets_path() -> PathBuf {
    let exe_path = env::current_exe().expect("Failed to get current executable path.");
    let mut assets_path =
        PathBuf::from(exe_path.parent().expect("Failed to get parent directory."));
    assets_path.push(VAULT_FOLDER);
    assets_path
}

#[cfg(debug_assertions)]
fn assets_path() -> PathBuf {
    let mut assets_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    assets_path.push(VAULT_FOLDER);
    assets_path
}

fn get_path(name: &str, folder: &str, extension: &str) -> PathBuf {
    assets_path()
        .join(folder)
        .join(name)
        .with_extension(extension)
}

// TODO load and export with a prefix from the struct name
pub fn save<T>(name: &str, data: T)
where
    T: Serialize,
{
    let path = get_path(&name, NATIVE_FOLDER, NATIVE_EXT);
    let mut file = File::create(path).expect("Failed to create file.");
    let encoded = serialize(&data).expect("Failed to serialize data.");
    file.write_all(&encoded).expect("Failed to write to file.");
}
pub fn load<T>(name: &str) -> T
where
    T: DeserializeOwned,
{
    let path = get_path(&name, NATIVE_FOLDER, NATIVE_EXT);
    dbg!(&path);
    let mut file = File::open(path).expect("Failed to open file.");
    let mut encoded = Vec::new();
    file.read_to_end(&mut encoded)
        .expect("Failed to read file.");
    let serialized = deserialize::<T>(&encoded).expect("Failed to deserialize data.");
    serialized
}

pub fn load_foreign_font(name: &str, extension: &str) -> Font {
    let path = get_path(&name, FOREIGN_FOLDER, extension);
    Font::from_foreign(path, extension)
}

pub fn load_foreign_image(name: &str, extension: &str) -> Image {
    let path = get_path(&name, FOREIGN_FOLDER, extension);
    Image::from_foreign(path, extension)
}

pub fn load_foreign_shader(name: &str, extension: &str) -> Shader {
    let path = get_path(&name, SHADER_FOLDER, extension);
    Shader::from_foreign(path, extension)
}
