use std::mem::size_of;

#[allow(dead_code)]
pub enum VertexAttr {
    Float,
    Vec2,
    Vec3,
    Vec4,
}

impl VertexAttr {
    pub fn count(&self) -> usize {
        match self {
            Self::Float => 1,
            Self::Vec2 => 2,
            Self::Vec3 => 3,
            Self::Vec4 => 4,
        }
    }

    pub fn size(&self) -> usize {
        match self {
            Self::Float | Self::Vec2 | Self::Vec3 | Self::Vec4 => self.count() * size_of::<f32>(),
        }
    }
}
