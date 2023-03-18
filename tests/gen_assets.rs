use cgmath::{point3, vec3, Angle, Deg, Matrix4, SquareMatrix};
use revenant::{
    types::{
        Camera, DirLight, Filtering, Material, PointLight, ProjectionKind, SpotLight, Texture,
        TextureKind, Wrapping,
    },
    vault,
};

#[test]
fn generate_assets() {
    let image_crate_diffuse = vault::load_foreign_image("crate_diffuse", "jpg");
    let image_crate_specular = vault::load_foreign_image("crate_specular", "jpg");
    let image_crate_emissive = vault::load_foreign_image("crate_emissive", "jpg");
    vault::save("image_crate_diffuse", image_crate_diffuse);
    vault::save("image_crate_specular", image_crate_specular);
    vault::save("image_crate_emissive", image_crate_emissive);

    let texture_crate_diffuse = Texture {
        gl_id: 0,
        image: vault::load("image_crate_diffuse"),
        kind: TextureKind::Diffuse,
        s_wrapping: Wrapping::Repeat,
        t_wrapping: Wrapping::Repeat,
        min_filtering: Filtering::Nearest,
        mag_filtering: Filtering::Nearest,
        mipmapping: true,
    };
    let texture_crate_specular = Texture {
        gl_id: 0,
        image: vault::load("image_crate_specular"),
        kind: TextureKind::Specular,
        s_wrapping: Wrapping::Repeat,
        t_wrapping: Wrapping::Repeat,
        min_filtering: Filtering::Nearest,
        mag_filtering: Filtering::Nearest,
        mipmapping: true,
    };
    let texture_crate_emissive = Texture {
        gl_id: 0,
        image: vault::load("image_crate_emissive"),
        kind: TextureKind::Emissive,
        s_wrapping: Wrapping::Repeat,
        t_wrapping: Wrapping::Repeat,
        min_filtering: Filtering::Nearest,
        mag_filtering: Filtering::Nearest,
        mipmapping: true,
    };
    vault::save("texture_crate_diffuse", texture_crate_diffuse);
    vault::save("texture_crate_specular", texture_crate_specular);
    vault::save("texture_crate_emissive", texture_crate_emissive);

    let material_crate = Material {
        diffuse: vault::load("texture_crate_diffuse"),
        specular: vault::load("texture_crate_specular"),
        specular_strength: 32.0,
        emissive: vault::load("texture_crate_emissive"),
    };
    vault::save("material_crate", material_crate);

    let camera_main = Camera {
        pos: point3(1.84, 0.8, 3.1),
        front: vec3(0.0, 0.0, -1.0),
        up: vec3(0.0, 1.0, 0.0),
        right: vec3(0.0, 0.0, 0.0),

        update_projection: true,
        projection_kind: ProjectionKind::Perspective {
            aspect_ratio: 16.0 / 9.0, // TODO get from window size
            fov_y: 45.0,
            near: 0.1,
            far: 100.0,
        },
        projection: Matrix4::identity(),
    };
    vault::save("camera_main", camera_main);

    let spotlight = SpotLight {
        pos: point3(1.2, 1.0, 2.0),
        dir: vec3(-1.2, -2.0, -0.3),
        cut_off: Angle::cos(Deg(12.5)),
        outer_cut_off: Angle::cos(Deg(60.0)),

        constant: 1.0,
        linear: 0.09,
        quadratic: 0.032,

        ambient: vec3(0.2, 0.2, 0.2),
        diffuse: vec3(0.5, 0.5, 0.5),
        specular: vec3(1.0, 1.0, 1.0),
    };
    vault::save("spotlight", spotlight);

    let pointlight = PointLight {
        pos: point3(0.7, 0.2, 2.0),

        constant: 1.0,
        linear: 0.09,
        quadratic: 0.032,

        ambient: vec3(0.2, 0.2, 0.2),
        diffuse: vec3(0.5, 0.5, 0.5),
        specular: vec3(1.0, 1.0, 1.0),
    };
    vault::save("pointlight", pointlight);

    let dirlight = DirLight {
        dir: vec3(-0.2, -1.0, -0.3),
        ambient: vec3(0.05, 0.05, 0.05),
        diffuse: vec3(0.4, 0.4, 0.4),
        specular: vec3(0.5, 0.5, 0.5),
    };
    vault::save("dirlight", dirlight);
}
