use crate::types::{Asset, AssetTexture, Filtering, ImageFormat, ImageKind, TextureSize, Wrapping};
use freetype::Bitmap;
use gl::types::{GLenum, GLint, GLvoid};

impl AssetTexture {
    pub fn from_bitmap(bitmap: &Bitmap) -> Self {
        let data = bitmap.buffer().to_vec();

        let width = bitmap.width();
        let height = bitmap.rows();

        let kind = ImageKind::Diffuse;

        let size = TextureSize::TwoD { width, height };

        let format = ImageFormat::Unicolor;

        let s_wrapping = Wrapping::Repeat;
        let t_wrapping = Wrapping::Repeat;

        let min_filtering = Filtering::Linear;
        let mag_filtering = Filtering::Linear;

        let mipmapping = false;

        // TODO bitmap is not an asset
        let asset = Asset {
            name: "".to_string(),
            path: "".to_string(),
        };

        AssetTexture::create_texture(
            asset,
            data,
            kind,
            size,
            format,
            s_wrapping,
            t_wrapping,
            min_filtering,
            mag_filtering,
            mipmapping,
        )
    }

    // TODO deal with max amount of texture units
    pub fn bind(&self, texture_unit: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + texture_unit);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    pub fn set_param_i(&self, param: u32, value: i32) {
        unsafe {
            // TODO add texture type (2D, 3D ... ) in Texture struct
            gl::TexParameteri(gl::TEXTURE_2D, param, value);
        }
    }

    pub fn create_texture(
        asset: Asset,
        data: Vec<u8>,
        kind: ImageKind,
        size: TextureSize,
        format: ImageFormat,
        s_wrapping: Wrapping,
        t_wrapping: Wrapping,
        min_filtering: Filtering,
        mag_filtering: Filtering,
        mipmapping: bool,
    ) -> AssetTexture {
        // TODO 3D texture
        let target = gl::TEXTURE_2D;

        let (width, height) = match size {
            TextureSize::TwoD { width, height } => (width, height),
            _ => panic!("Texture size not supported yet."),
        };

        let internal_format = match format {
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

        let gl_s_wrapping = gl_wrapping_from(&s_wrapping);
        let gl_t_wrapping = gl_wrapping_from(&t_wrapping);
        let gl_min_filtering = gl_filtering_from(&min_filtering);
        let gl_mag_filtering = gl_filtering_from(&mag_filtering);

        let mut id = 0;
        unsafe {
            // generate texture id
            gl::GenTextures(1, &mut id);
            gl::BindTexture(target, id);
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
                data.as_ptr() as *const GLvoid,
            );
            // wrapping
            gl::TexParameteri(target, gl::TEXTURE_WRAP_S, gl_s_wrapping as i32);
            gl::TexParameteri(target, gl::TEXTURE_WRAP_T, gl_t_wrapping as i32);
            // filtering
            gl::TexParameteri(target, gl::TEXTURE_MIN_FILTER, gl_min_filtering as i32);
            gl::TexParameteri(target, gl::TEXTURE_MAG_FILTER, gl_mag_filtering as i32);
        }
        // mipmapping
        if mipmapping {
            unsafe {
                gl::GenerateMipmap(target);
            }
        }

        Self {
            asset,
            id,
            kind,
            format,
            size,
            s_wrapping,
            t_wrapping,
            min_filtering,
            mag_filtering,
            mipmapping,
        }
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
