use crate::gl::consts;

#[repr(u32)]

pub enum FilterMode {
    Nearest = consts::GL_NEAREST,
    Linear = consts::GL_LINEAR,
}

#[repr(u32)]
pub enum WrapMode {
    ClampToEdge = consts::GL_CLAMP_TO_EDGE,
    ClampToBorder = consts::GL_CLAMP_TO_BORDER,
    MirroredRepeat = consts::GL_MIRRORED_REPEAT,
    Repeat = consts::GL_REPEAT,
}
