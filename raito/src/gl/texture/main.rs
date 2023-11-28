use crate::TextureData;

use super::{Handle, Pass, Pixel};

pub struct Texture<T: Pixel> {
    handle: Handle<T>,
}

impl<T: Pixel> Texture<T> {
    pub(super) fn new(handle: Handle<T>) -> Self {
        Self { handle }
    }

    pub fn bind(&mut self, unit: i32) {
        self.handle.bind_unit(unit);
    }

    #[allow(dead_code)]
    pub fn set_data(&mut self, data: &TextureData<T>) {
        let mut pass = Pass::new(&mut self.handle);
        pass.set_data(data);
    }
}
