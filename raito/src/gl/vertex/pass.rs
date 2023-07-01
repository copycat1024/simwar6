use super::{Vao, Vbo, Vertex};
use crate::gl::{enums::VertexAttribPointerType, Gl};
use std::{ffi::c_void, mem::size_of};

pub struct VaoPass<'a, V: Vertex> {
    vao: &'a mut Vao<V>,
    gl: Gl,
}

impl<'a, V: Vertex> VaoPass<'a, V> {
    pub(super) fn new(vao: &'a mut Vao<V>, gl: Gl) -> Self {
        vao.bind();

        Self { vao, gl }
    }

    pub fn vertex(&mut self) -> &mut Vbo<V> {
        &mut self.vao.array
    }

    pub fn set_attr(&mut self, id: u32, count: usize, offset: usize) {
        let count = count as i32;
        let type_ = VertexAttribPointerType::Float;
        let stride = size_of::<V>() as i32;
        let offset = offset as *const c_void;

        self.gl
            .vertex_attrib_pointer(id, count, type_, false, stride, offset);
        self.gl.enable_vertex_attrib_array(id);
    }
}

impl<'a, V: Vertex> Drop for VaoPass<'a, V> {
    fn drop(&mut self) {
        self.vao.unbind()
    }
}
