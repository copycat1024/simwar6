use super::{handle::Handle, Pass, Pixel, Texture, TextureData};
use crate::gl::{enums::TextureTarget, Gl};

pub struct Builder<T: Pixel> {
    handle: Handle<T>,
}

impl<T: Pixel> Builder<T> {
    pub fn new(gl: &Gl, target: TextureTarget) -> Self {
        Self {
            handle: Handle::new(gl, target),
        }
    }

    pub fn rectangle(gl: &Gl) -> Self {
        Self::new(gl, TextureTarget::TextureRectangle)
    }

    pub fn config<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut Pass<T>),
    {
        {
            let mut pass = Pass::new(&mut self.handle);
            f(&mut pass);
        }
        self
    }

    pub fn build(mut self, data: &TextureData<T>) -> Texture<T> {
        self = self.config(|pass| pass.set_data(data));
        Texture::new(self.handle)
    }
}
