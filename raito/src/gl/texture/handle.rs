use super::{Pixel, TextureData};
use crate::gl::{
    enums::{
        InternalFormat, PixelFormat, PixelType, TextureParameterName, TextureTarget, TextureUnit,
    },
    Gl,
};
use std::marker::PhantomData;

pub struct Handle<T: Pixel> {
    id: u32,
    target: TextureTarget,
    gl: Gl,
    ghost: PhantomData<T>,
}

impl<T: Pixel> Handle<T> {
    pub fn new(gl: &Gl, target: TextureTarget) -> Self {
        Self {
            id: gl.new_texture(),
            gl: gl.clone(),
            target,
            ghost: PhantomData,
        }
    }

    pub(super) fn bind(&self) {
        let Self { gl, target, id, .. } = self;
        gl.bind_texture(*target, *id);
    }

    pub(super) fn bind_unit(&self, unit: i32) {
        let unit = Self::unit(unit);
        self.gl.active_texture(unit);
        self.gl.bind_texture(self.target, self.id);
    }

    pub(super) fn set_parameter(&mut self, name: TextureParameterName, value: u32) {
        let Self { gl, target, .. } = self;
        gl.tex_parameteri(*target, name, value as i32);
    }

    pub(super) fn set_data(&mut self, data: &TextureData<T>) {
        let Self { gl, target, .. } = self;

        gl.tex_image2_d(
            *target,
            0,
            InternalFormat::R32f,
            data.get_w() as i32,
            data.get_h() as i32,
            0,
            PixelFormat::Red,
            PixelType::Float,
            data.ptr().cast(),
        );
    }

    fn unit(id: i32) -> TextureUnit {
        match id {
            0 => TextureUnit::Texture0,
            1 => TextureUnit::Texture1,
            2 => TextureUnit::Texture2,
            3 => TextureUnit::Texture3,
            _ => panic!("Too many texture binded"),
        }
    }
}

impl<T: Pixel> Drop for Handle<T> {
    fn drop(&mut self) {
        let Self { gl, id, .. } = self;
        gl.delete_textures(1, id);
    }
}
