use super::{Pass, Texture};
use crate::gl::{
    enums::{TextureTarget, TextureUnit},
    Gl,
};

pub struct Builder<T: Default + Clone> {
    pub(super) id: u32,
    pub(super) gl: Gl,
    pub(super) data: Vec<T>,
    pub(super) target: TextureTarget,
    pub(super) w: usize,
    pub(super) h: usize,
}

impl<T: Default + Clone> Builder<T> {
    pub fn new(gl: &Gl, target: TextureTarget, w: usize, h: usize) -> Self {
        let id = gl.new_texture();
        let data = vec![T::default(); w * h];

        Self {
            id,
            gl: gl.clone(),
            data,
            target,
            w,
            h,
        }
    }

    pub fn rectangle(gl: &Gl, w: usize, h: usize) -> Self {
        Self::new(gl, TextureTarget::TextureRectangle, w, h)
    }

    pub fn config<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut Pass<T>),
    {
        {
            self.gl.active_texture(TextureUnit::Texture0);
            self.gl.bind_texture(self.target, self.id);

            let mut pass = Pass::new(&mut self);
            f(&mut pass);
        }
        self
    }

    pub fn build(mut self) -> Texture {
        self = self.config(|pass| pass.flush_to_gpu());
        Texture::new(self)
    }
}
