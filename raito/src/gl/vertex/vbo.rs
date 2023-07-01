use super::{Target, Usage};
use crate::gl::Gl;
use std::mem::size_of;

pub struct Vbo<T> {
    id: u32,
    gl: Gl,
    data: Vec<T>,
}

impl<T> Vbo<T> {
    pub(super) fn new(gl: &Gl) -> Self {
        Self {
            id: gl.new_vbo(),
            gl: gl.clone(),
            data: Vec::new(),
        }
    }

    pub fn data<V>(&mut self, data: V)
    where
        V: IntoIterator<Item = T>,
    {
        let target = Target::Array.into();
        let usage = Usage::StaticDraw;
        let data: Vec<T> = data.into_iter().collect();

        let size = (data.len() * size_of::<T>()) as isize;
        let ptr = data.as_ptr();

        self.gl.bind_buffer(target, self.id);
        self.gl.buffer_data(target, size, ptr.cast(), usage);

        self.data = data;
    }
}

impl<T> Drop for Vbo<T> {
    fn drop(&mut self) {
        self.gl.delete_buffers(1, &self.id);
    }
}
