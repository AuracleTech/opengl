use crate::{
    assets_path,
    serialization::{deserialize, serialize},
    types::{
        AssetImage, AssetTexture, AssetTextureSerialized, Filtering, ImageFormat, TextureKind,
        TextureSize, Wrapping,
    },
};
use freetype::Bitmap;
use gl::types::{GLenum, GLint, GLvoid};

const EXT: &str = "texture";
const PATH: &str = "textures";

impl AssetTexture {
    pub fn from_bitmap(bitmap: &Bitmap, character: usize, font: &str) -> Self {
        let data = bitmap.buffer().to_vec();
        let size = TextureSize::TwoD {
            width: bitmap.width(),
            height: bitmap.rows(),
        };
        let format = ImageFormat::Unicolor;

        // TODO make all these configurable
        let kind = TextureKind::Diffuse;
        let s_wrapping = Wrapping::Repeat;
        let t_wrapping = Wrapping::Repeat;
        let min_filtering = Filtering::Linear;
        let mag_filtering = Filtering::Linear;

        let mipmapping = false;

        Self {
            name: font.to_owned() + "_" + &character.to_string(),
            gl_id: 0,

            image: AssetImage {
                filename: character.to_string(),
                data,
                format,
                size,
            },

            kind,
            s_wrapping,
            t_wrapping,
            min_filtering,
            mag_filtering,
            mipmapping,
        }
    }

    pub fn gl_register(&mut self) {
        let internal_format = match self.image.format {
            ImageFormat::RGB => gl::RGB,
            ImageFormat::RGBA => gl::RGBA,
            ImageFormat::RG => panic!("RG format not supported yet."),
            ImageFormat::R => panic!("R format not supported yet."),
            ImageFormat::Unicolor => gl::RED,
        };
        let alignment = match internal_format {
            gl::RGB => 1,
            gl::RGBA => 4,
            gl::RED => 1,
            _ => panic!("Texture format not supported yet."),
        };
        unsafe {
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, alignment);
        }

        let gl_s_wrapping = gl_wrapping_from(&self.s_wrapping);
        let gl_t_wrapping = gl_wrapping_from(&self.t_wrapping);
        let gl_min_filtering = gl_filtering_from(&self.min_filtering);
        let gl_mag_filtering = gl_filtering_from(&self.mag_filtering);

        // TODO 3D texture
        let target = gl::TEXTURE_2D;

        let mut id = 0;
        unsafe {
            // generate texture id
            gl::GenTextures(1, &mut id);
            gl::BindTexture(target, id);
        }
        match self.image.size {
            TextureSize::TwoD { width, height } => {
                unsafe {
                    // texture data
                    gl::TexImage2D(
                        target,
                        0,
                        internal_format as GLint,
                        width,
                        height,
                        0,
                        internal_format,
                        gl::UNSIGNED_BYTE,
                        self.image.data.as_ptr() as *const GLvoid,
                    );
                }
            }
            // TODO 3D texture
            _ => panic!("Texture size not supported yet."),
        }
        unsafe {
            // wrapping
            gl::TexParameteri(target, gl::TEXTURE_WRAP_S, gl_s_wrapping as i32);
            gl::TexParameteri(target, gl::TEXTURE_WRAP_T, gl_t_wrapping as i32);
            // filtering
            gl::TexParameteri(target, gl::TEXTURE_MIN_FILTER, gl_min_filtering as i32);
            gl::TexParameteri(target, gl::TEXTURE_MAG_FILTER, gl_mag_filtering as i32);
        }
        // mipmapping
        if self.mipmapping {
            unsafe {
                gl::GenerateMipmap(target);
            }
        }

        self.gl_id = id;
    }

    // TODO deal with max amount of texture units
    pub fn bind(&self, texture_unit: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + texture_unit);
            gl::BindTexture(gl::TEXTURE_2D, self.gl_id);
        }
    }

    pub fn set_param_i(&self, param: u32, value: i32) {
        unsafe {
            // TODO add texture type (2D, 3D ... ) in Texture struct
            gl::TexParameteri(gl::TEXTURE_2D, param, value);
        }
    }

    pub fn load(name: String) -> Self {
        let path = assets_path().join(PATH).join(&name).with_extension(EXT);
        let data = deserialize::<AssetTextureSerialized>(path);
        let mut texture = Self {
            name,
            gl_id: 0,
            image: AssetImage::load(&data.filename),
            kind: data.kind,
            s_wrapping: data.s_wrapping,
            t_wrapping: data.t_wrapping,
            min_filtering: data.min_filtering,
            mag_filtering: data.mag_filtering,
            mipmapping: data.mipmapping,
        };
        texture.gl_register();
        texture
    }
    pub fn save(self) {
        let path = assets_path()
            .join(PATH)
            .join(&self.name)
            .with_extension(EXT);
        let data = AssetTextureSerialized {
            filename: self.image.filename,
            kind: self.kind,
            s_wrapping: self.s_wrapping,
            t_wrapping: self.t_wrapping,
            min_filtering: self.min_filtering,
            mag_filtering: self.mag_filtering,
            mipmapping: self.mipmapping,
        };
        serialize(path, data);
    }
}

fn gl_filtering_from(filtering: &Filtering) -> GLenum {
    match filtering {
        Filtering::Nearest => gl::NEAREST,
        Filtering::Linear => gl::LINEAR,
        Filtering::NearestMipmapNearest => gl::NEAREST_MIPMAP_NEAREST,
        Filtering::NearestMipmapLinear => gl::NEAREST_MIPMAP_LINEAR,
        Filtering::LinearMipmapNearest => gl::LINEAR_MIPMAP_NEAREST,
        Filtering::LinearMipmapLinear => gl::LINEAR_MIPMAP_LINEAR,
    }
}

fn gl_wrapping_from(wrapping: &Wrapping) -> GLenum {
    match wrapping {
        Wrapping::Repeat => gl::REPEAT,
        Wrapping::MirroredRepeat => gl::MIRRORED_REPEAT,
        Wrapping::ClampToEdge => gl::CLAMP_TO_EDGE,
        Wrapping::ClampToBorder => gl::CLAMP_TO_BORDER,
    }
}
