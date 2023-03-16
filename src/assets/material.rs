use crate::{
    assets_path,
    serialization::{deserialize, serialize},
    types::{AssetMaterial, AssetMaterialSerialized, AssetTexture},
};

const EXT: &str = "material";
const PATH: &str = "materials";

impl AssetMaterial {
    pub fn load(name: &str) -> Self {
        let path = assets_path().join(PATH).join(&name).with_extension(EXT);
        let data = deserialize::<AssetMaterialSerialized>(path);
        let mut diffuse = AssetTexture::load(data.diffuse);
        let mut specular = AssetTexture::load(data.specular);
        let mut emissive = AssetTexture::load(data.emissive);
        diffuse.gl_register();
        specular.gl_register();
        emissive.gl_register();
        Self {
            name: name.to_string(),
            diffuse,
            specular,
            specular_strength: data.specular_strength,
            emissive,
        }
    }
    pub fn save(self) {
        let path = assets_path().join(PATH).join(self.name).with_extension(EXT);
        let data = AssetMaterialSerialized {
            diffuse: self.diffuse.name,
            specular: self.specular.name,
            specular_strength: self.specular_strength,
            emissive: self.emissive.name,
        };
        serialize(path, data);
    }
}
