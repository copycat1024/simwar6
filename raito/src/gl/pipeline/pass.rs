use super::Program;
use crate::gl::{enums::PrimitiveType, Gl, Texture, Vao, Vertex};

pub struct ProgramPass<'a> {
    program: &'a mut Program,
    gl: Gl,
    texture: i32,
}

impl<'a> ProgramPass<'a> {
    pub(super) fn new(program: &'a mut Program, gl: Gl) -> Self {
        program.bind();
        Self {
            program,
            gl,
            texture: 0,
        }
    }

    pub fn draw<V: Vertex>(&mut self, vao: &mut Vao<V>, size: usize) {
        let _pass = vao.pass();
        let size = size as i32;
        self.gl.draw_arrays(PrimitiveType::Points, 0, size);
    }

    pub fn bind_texture(&mut self, name: &str, texture: &mut Texture) {
        texture.bind(self.texture);
        self.set_1i(name, self.texture);
        self.texture += 1;
    }

    pub fn set_1i(&mut self, name: &str, x: i32) {
        let id = self.get_uniform_id(name);
        self.gl.uniform1i(id, x);
    }

    pub fn set_2f(&mut self, name: &str, x: f32, y: f32) {
        let id = self.get_uniform_id(name);
        self.gl.uniform2f(id, x, y);
    }

    pub fn set_3f(&mut self, name: &str, x: f32, y: f32, z: f32) {
        let id = self.get_uniform_id(name);
        self.gl.uniform3f(id, x, y, z);
    }

    fn get_uniform_id(&self, name: &str) -> i32 {
        let id = self.program.id;
        self.gl.get_uniform_id(id, name)
    }
}

impl<'a> Drop for ProgramPass<'a> {
    fn drop(&mut self) {
        self.program.unbind()
    }
}
