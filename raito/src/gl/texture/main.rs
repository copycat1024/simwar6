use super::Builder;
use crate::gl::{
    enums::{TextureTarget, TextureUnit},
    Gl,
};

pub struct Texture {
    id: u32,
    gl: Gl,
    target: TextureTarget,
}

impl Texture {
    pub(super) fn new(builder: Builder) -> Self {
        let Builder { id, gl, target, .. } = builder;

        Self { id, gl, target }
    }

    pub fn bind(&mut self, unit: TextureUnit) {
        self.gl.active_texture(unit);
        self.gl.bind_texture(self.target, self.id);
    }

    pub fn unit(id: i32) -> TextureUnit {
        match id {
            0 => TextureUnit::Texture0,
            1 => TextureUnit::Texture1,
            2 => TextureUnit::Texture2,
            3 => TextureUnit::Texture3,
            _ => panic!("Too many texture binded"),
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {}
}
