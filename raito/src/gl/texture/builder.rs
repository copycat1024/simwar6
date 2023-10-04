use super::{Pass, Pixel, Texture, TextureData};
use crate::gl::{enums::TextureTarget, Gl};

pub struct Builder {
    pub(super) id: u32,
    pub(super) gl: Gl,
    pub(super) target: TextureTarget,
}

impl Builder {
    pub fn new(gl: &Gl, target: TextureTarget) -> Self {
        Self {
            id: gl.new_texture(),
            gl: gl.clone(),
            target,
        }
    }

    pub fn rectangle(gl: &Gl) -> Self {
        Self::new(gl, TextureTarget::TextureRectangle)
    }

    pub fn config<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut Pass),
    {
        {
            let mut pass = Pass::new(&self.gl, &mut self.target, self.id);
            f(&mut pass);
        }
        self
    }

    pub fn build<T: Pixel>(mut self, data: &TextureData<T>) -> Texture {
        self = self.config(|pass| pass.set_data(data));
        Texture::new(self)
    }
}
