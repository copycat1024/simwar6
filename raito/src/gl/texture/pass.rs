use super::{Builder, FilterMode, WrapMode};
use crate::gl::enums::{InternalFormat, PixelFormat, PixelType, TextureParameterName};

pub struct Pass<'a, T: Default + Clone> {
    builder: &'a mut Builder<T>,
}

impl<'a, T: Default + Clone> Pass<'a, T> {
    pub(super) fn new(builder: &'a mut Builder<T>) -> Self {
        Self { builder }
    }

    pub fn set_parameter(&mut self, name: TextureParameterName, value: u32) {
        let Builder { gl, target, .. } = self.builder;
        gl.tex_parameteri(*target, name, value as i32);
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

    pub fn set(&mut self, x: usize, y: usize, item: T) {
        let Builder { w, h, data, .. } = self.builder;

        if x < *w && y < *h {
            data[x + y * *w] = item;
        }
    }

    pub(super) fn flush_to_gpu(&mut self) {
        let Builder {
            gl,
            target,
            w,
            h,
            data,
            ..
        } = self.builder;

        gl.tex_image2_d(
            *target,
            0,
            InternalFormat::R32f,
            *w as i32,
            *h as i32,
            0,
            PixelFormat::Red,
            PixelType::Float,
            data.as_ptr().cast(),
        );
    }
}

impl<'a, T> Drop for Pass<'a, T>
where
    T: Default + Clone,
{
    fn drop(&mut self) {}
}
