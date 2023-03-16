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
        Self {
            name: name.to_string(),
            diffuse: AssetTexture::load(data.diffuse),
            specular: AssetTexture::load(data.specular),
            specular_strength: data.specular_strength,
            emissive: AssetTexture::load(data.emissive),
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
