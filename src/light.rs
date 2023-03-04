use glm::Vec3;

#[allow(dead_code)]
pub struct DirLight {
    pub dir: Vec3,

    pub ambient: Vec3,
    pub diffuse: Vec3,
    pub specular: Vec3,
}

#[allow(dead_code)]
pub struct PointLight {
    pub pos: Vec3,

    pub constant: f32,
    pub linear: f32,
    pub quadratic: f32,

    pub ambient: Vec3,
    pub diffuse: Vec3,
    pub specular: Vec3,
}

#[allow(dead_code)]
pub struct SpotLight {
    pub pos: Vec3,
    pub dir: Vec3,

    pub cut_off: f32,
    pub outer_cut_off: f32,

    pub constant: f32,
    pub linear: f32,
    pub quadratic: f32,

    pub ambient: Vec3,
    pub diffuse: Vec3,
    pub specular: Vec3,
}
