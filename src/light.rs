use glm::Vec3;

pub struct Light {
    pub ambient: Vec3,
    pub diffuse: Vec3,
    pub specular: Vec3,

    pub constant: f32,
    pub linear: f32,
    pub quadratic: f32,
}

pub struct DirLight {
    pub dir: Vec3,
    pub light: Light,
}

pub struct PointLight {
    pub pos: Vec3,
    pub light: Light,
}
