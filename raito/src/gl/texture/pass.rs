use super::{FilterMode, Handle, Pixel, TextureData, WrapMode};
use crate::gl::enums::TextureParameterName;

pub struct Pass<'a, T: Pixel> {
    handle: &'a mut Handle<T>,
}

impl<'a, T: Pixel> Pass<'a, T> {
    pub(super) fn new(handle: &'a mut Handle<T>) -> Self {
        handle.bind();
        Self { handle }
    }

    pub fn set_wrap_x(&mut self, mode: WrapMode) {
        self.handle
            .set_parameter(TextureParameterName::TextureWrapS, mode as u32)
    }

    pub fn set_wrap_y(&mut self, mode: WrapMode) {
        self.handle
            .set_parameter(TextureParameterName::TextureWrapT, mode as u32)
    }

    pub fn set_filter_mag(&mut self, mode: FilterMode) {
        self.handle
            .set_parameter(TextureParameterName::TextureMagFilter, mode as u32)
    }

    pub fn set_filter_min(&mut self, mode: FilterMode) {
        self.handle
            .set_parameter(TextureParameterName::TextureMinFilter, mode as u32)
    }

    pub(super) fn set_data(&mut self, data: &TextureData<T>) {
        self.handle.set_data(data);
    }
}

impl<'a, T: Pixel> Drop for Pass<'a, T> {
    fn drop(&mut self) {}
}
