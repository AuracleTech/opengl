use cgmath::{Point3, Vector2, Vector3, Vector4};

// TODO replace all Point3 and Vec3 by [f32; 3]
pub type Uniaxial = f32;
pub type Position = Point3<Uniaxial>;
pub type Direction = Vector3<Uniaxial>;
pub type Normal = Vector3<Uniaxial>;
pub type TexCoord = Vector2<Uniaxial>; // OPTIMIZE use u16 if possible or even u8
pub type ColorChannel = f32;
pub type Rgb = Vector3<ColorChannel>;
pub type Rgba = Vector4<ColorChannel>;
pub type Indice = u32; // OPTIMIZE use u16 if possible
