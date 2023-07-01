use crate::gl::consts;

#[repr(u32)]
#[allow(dead_code)]
pub enum WrapMode {
    ClampToEdge = consts::GL_CLAMP_TO_EDGE,
    ClampToBorder = consts::GL_CLAMP_TO_BORDER,
    MirroredRepeat = consts::GL_MIRRORED_REPEAT,
    Repeat = consts::GL_REPEAT,
}
