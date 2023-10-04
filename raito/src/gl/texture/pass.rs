use super::{FilterMode, Pixel, TextureData, WrapMode};
use crate::gl::{
    enums::{InternalFormat, PixelFormat, PixelType, TextureParameterName, TextureTarget},
    Gl,
};

pub struct Pass<'a> {
    gl: &'a Gl,
    target: &'a mut TextureTarget,
}

impl<'a> Pass<'a> {
    pub(super) fn new(gl: &'a Gl, target: &'a mut TextureTarget, id: u32) -> Self {
        gl.bind_texture(*target, id);
        Self { gl, target }
    }

    pub fn set_parameter(&mut self, name: TextureParameterName, value: u32) {
        let Self { gl, target } = self;
        gl.tex_parameteri(**target, name, value as i32);
    }

    pub fn set_wrap_x(&mut self, mode: WrapMode) {
        self.set_parameter(TextureParameterName::TextureWrapS, mode as u32)
    }

    pub fn set_wrap_y(&mut self, mode: WrapMode) {
        self.set_parameter(TextureParameterName::TextureWrapT, mode as u32)
    }

    pub fn set_filter_mag(&mut self, mode: FilterMode) {
        self.set_parameter(TextureParameterName::TextureMagFilter, mode as u32)
    }

    pub fn set_filter_min(&mut self, mode: FilterMode) {
        self.set_parameter(TextureParameterName::TextureMinFilter, mode as u32)
    }

    pub(super) fn set_data<T: Pixel>(&mut self, data: &TextureData<T>) {
        let Self { gl, target } = self;

        gl.tex_image2_d(
            **target,
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
}

impl<'a> Drop for Pass<'a> {
    fn drop(&mut self) {}
}
