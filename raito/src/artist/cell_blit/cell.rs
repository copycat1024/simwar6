use crate::gl::{Vertex, VertexAttr};

pub struct Cell {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub id: f32,
}

impl Cell {
    pub fn new(x: f32, y: f32, z: f32, id: u8) -> Self {
        Self {
            x,
            y,
            z,
            id: id as f32,
        }
    }
}

impl Vertex for Cell {
    fn attr() -> Vec<VertexAttr> {
        vec![VertexAttr::Vec4]
    }
}
