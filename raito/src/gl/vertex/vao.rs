use super::{VaoPass, Vbo, Vertex};
use crate::gl::Gl;

pub struct Vao<V: Vertex> {
    id: u32,
    gl: Gl,

    pub(super) array: Vbo<V>,
}

impl<V: Vertex> Vao<V> {
    pub fn new(gl: &Gl) -> Self {
        let id = gl.new_vao();
        let array = Vbo::new(gl);

        Self {
            id,
            array,
            gl: gl.clone(),
        }
    }

    pub(super) fn bind(&self) {
        self.gl.bind_vertex_array(self.id);
    }

    pub(super) fn unbind(&self) {
        self.gl.bind_vertex_array(0);
    }

    pub(in crate::gl) fn pass(&mut self) -> VaoPass<V> {
        let gl = self.gl.clone();
        VaoPass::new(self, gl)
    }

    pub fn config<F>(&mut self, f: F)
    where
        F: FnOnce(&mut VaoPass<V>),
    {
        let gl = self.gl.clone();
        let mut pass = VaoPass::new(self, gl);
        let mut offset: usize = 0;

        f(&mut pass);

        for (i, attr) in V::attr().into_iter().enumerate() {
            pass.set_attr(i as u32, attr.count(), offset);
            offset += attr.size();
        }
    }
}

impl<V: Vertex> Drop for Vao<V> {
    fn drop(&mut self) {
        self.gl.delete_vertex_arrays(1, &self.id);
    }
}
