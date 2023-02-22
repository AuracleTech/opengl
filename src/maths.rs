pub struct Mat4 {
    pub elements: [f32; 16],
}

impl Mat4 {
    pub fn new() -> Self {
        Self {
            elements: [0.0; 16],
        }
    }

    pub fn identity() -> Self {
        let mut result = Self::new();
        result.elements[0 + 0 * 4] = 1.0;
        result.elements[1 + 1 * 4] = 1.0;
        result.elements[2 + 2 * 4] = 1.0;
        result.elements[3 + 3 * 4] = 1.0;
        result
    }

    pub fn as_ptr(&self) -> *const f32 {
        self.elements.as_ptr()
    }

    pub fn translate(&mut self, translation: Vec3) {
        self.elements[0 + 3 * 4] = translation.x;
        self.elements[1 + 3 * 4] = translation.y;
        self.elements[2 + 3 * 4] = translation.z;
    }

    pub fn scale(&mut self, scale: Vec3) {
        self.elements[0 + 0 * 4] = scale.x;
        self.elements[1 + 1 * 4] = scale.y;
        self.elements[2 + 2 * 4] = scale.z;
    }

    pub fn rotate(&mut self, angle: f32, axis: Vec3) {
        let r = angle.to_radians();
        let c = r.cos();
        let s = r.sin();
        let omc = 1.0 - c;

        let x = axis.x;
        let y = axis.y;
        let z = axis.z;

        self.elements[0 + 0 * 4] = x * omc + c;
        self.elements[1 + 0 * 4] = y * x * omc + z * s;
        self.elements[2 + 0 * 4] = x * z * omc - y * s;

        self.elements[0 + 1 * 4] = x * y * omc - z * s;
        self.elements[1 + 1 * 4] = y * omc + c;
        self.elements[2 + 1 * 4] = y * z * omc + x * s;

        self.elements[0 + 2 * 4] = x * z * omc + y * s;
        self.elements[1 + 2 * 4] = y * z * omc - x * s;
        self.elements[2 + 2 * 4] = z * omc + c;
    }

    pub fn ortho(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        let mut result = Self::new();
        result.elements[0 + 0 * 4] = 2.0 / (right - left);
        result.elements[1 + 1 * 4] = 2.0 / (top - bottom);
        result.elements[2 + 2 * 4] = 2.0 / (near - far);
        result.elements[0 + 3 * 4] = (left + right) / (left - right);
        result.elements[1 + 3 * 4] = (bottom + top) / (bottom - top);
        result.elements[2 + 3 * 4] = (far + near) / (far - near);
        result.elements[3 + 3 * 4] = 1.0;
        result
    }

    pub fn perspective(fov: f32, aspect_ratio: f32, near: f32, far: f32) -> Self {
        let mut result = Self::new();
        let q = 1.0 / (fov / 2.0).tan();
        let a = q / aspect_ratio;
        let b = (near + far) / (near - far);
        let c = (2.0 * near * far) / (near - far);

        result.elements[0 + 0 * 4] = a;
        result.elements[1 + 1 * 4] = q;
        result.elements[2 + 2 * 4] = b;
        result.elements[2 + 3 * 4] = -1.0;
        result.elements[3 + 2 * 4] = c;
        result
    }
}

pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}
