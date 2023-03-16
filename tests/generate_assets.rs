use cgmath::{point3, vec3, Matrix4, SquareMatrix};
use revenant::types::{
    AssetCamera, AssetImage, AssetMaterial, AssetTexture, Filtering, ProjectionKind, TextureKind,
    Wrapping,
};

#[test]
fn generate_textures() {
    AssetTexture {
        name: "crate_diffuse".to_owned(),
        gl_id: 0,
        image: AssetImage::load("crate_diffuse.jpg"),
        kind: TextureKind::Diffuse,
        s_wrapping: Wrapping::Repeat,
        t_wrapping: Wrapping::Repeat,
        min_filtering: Filtering::Nearest,
        mag_filtering: Filtering::Nearest,
        mipmapping: true,
    }
    .save();

    AssetTexture {
        name: "crate_specular".to_owned(),
        gl_id: 0,
        image: AssetImage::load("crate_specular.jpg"),
        kind: TextureKind::Specular,
        s_wrapping: Wrapping::Repeat,
        t_wrapping: Wrapping::Repeat,
        min_filtering: Filtering::Nearest,
        mag_filtering: Filtering::Nearest,
        mipmapping: true,
    }
    .save();

    AssetTexture {
        name: "crate_emissive".to_owned(),
        gl_id: 0,
        image: AssetImage::load("crate_emissive.jpg"),
        kind: TextureKind::Emissive,
        s_wrapping: Wrapping::Repeat,
        t_wrapping: Wrapping::Repeat,
        min_filtering: Filtering::Nearest,
        mag_filtering: Filtering::Nearest,
        mipmapping: true,
    }
    .save();
}

#[test]
fn generate_material_and_load_textures() {
    AssetMaterial {
        name: "crate".to_owned(),
        diffuse: AssetTexture::load("crate_diffuse".to_owned()),
        specular: AssetTexture::load("crate_specular".to_owned()),
        specular_strength: 32.0,
        emissive: AssetTexture::load("crate_emissive".to_owned()),
    }
    .save();
}

#[test]
fn generate_camera() {
    AssetCamera {
        name: "main".to_owned(),
        pos: point3(1.84, 0.8, 3.1),
        front: vec3(0.0, 0.0, -1.0),
        up: vec3(0.0, 1.0, 0.0),
        right: vec3(0.0, 0.0, 0.0),

        update_projection: true,
        projection_kind: ProjectionKind::Perspective {
            aspect_ratio: 16.0 / 9.0,
            fov_y: 45.0,
            near: 0.1,
            far: 100.0,
        },
        projection: Matrix4::identity(),
    }
    .save();
}
