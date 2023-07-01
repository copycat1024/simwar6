use crate::gl::consts;

#[repr(u32)]
#[allow(dead_code)]

pub enum FilterMode {
    Nearest = consts::GL_NEAREST,
    Linear = consts::GL_LINEAR,
}
