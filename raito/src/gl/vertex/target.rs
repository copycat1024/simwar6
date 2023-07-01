use crate::gl::enums::BufferTargetARB;

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum Target {
    Array,
    CopyRead,
    CopyWrite,
    ElementArray,
    Texture,
    Uniform,
}

impl From<Target> for BufferTargetARB {
    fn from(value: Target) -> Self {
        match value {
            Target::Array => Self::ArrayBuffer,
            Target::CopyRead => Self::CopyReadBuffer,
            Target::CopyWrite => Self::CopyWriteBuffer,
            Target::ElementArray => Self::ElementArrayBuffer,
            Target::Texture => Self::TextureBuffer,
            Target::Uniform => Self::UniformBuffer,
        }
    }
}
