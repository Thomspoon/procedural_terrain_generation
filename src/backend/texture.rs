use std::path::Path;

use crate::backend::gl_gen::gl;
use image::GenericImageView;

pub struct Texture(gl::types::GLuint);

impl Texture {
    pub fn new<S: Into<String>>(path_name: S) -> (Self, gl::types::GLuint) {
        let path_name = path_name.into();
        let path = Path::new(&path_name);

        let image =
            image::open(&path).unwrap_or_else(|_| panic!("Unable to open file: {:?}", path));

        let data = image.raw_pixels();

        let mut texture_id = 0;
        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);

            // set the texture wrapping parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as _);
            // set texture filtering parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as _);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as _,
                image.width() as _,
                image.height() as _,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                &data[0] as *const u8 as *const _,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        (Self(texture_id), texture_id)
    }

    pub fn bind(&self) {
        unsafe {
            // bind textures on corresponding texture units
            //gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.0);
        }
    }
}
