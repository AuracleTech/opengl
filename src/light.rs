use cgmath::Vector3;

#[allow(dead_code)]
pub struct DirLight {
    pub dir: Vector3<f32>,

    pub ambient: Vector3<f32>,
    pub diffuse: Vector3<f32>,
    pub specular: Vector3<f32>,
}

#[allow(dead_code)]
pub struct PointLight {
    pub pos: Vector3<f32>,

    pub constant: f32,
    pub linear: f32,
    pub quadratic: f32,

    pub ambient: Vector3<f32>,
    pub diffuse: Vector3<f32>,
    pub specular: Vector3<f32>,
}

#[allow(dead_code)]
pub struct SpotLight {
    pub pos: Vector3<f32>,
    pub dir: Vector3<f32>,

    pub cut_off: f32,
    pub outer_cut_off: f32,

    pub constant: f32,
    pub linear: f32,
    pub quadratic: f32,

    pub ambient: Vector3<f32>,
    pub diffuse: Vector3<f32>,
    pub specular: Vector3<f32>,
}
