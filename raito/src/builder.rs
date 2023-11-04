use super::Context;
use crate::sdl::{gl, init, window};

pub struct Builder<'a> {
    pub(super) title: &'a str,

    pub(super) init_flag: u32,

    pub(super) win_pos: (i32, i32, i32, i32),
    pub(super) win_flag: u32,

    pub(super) gl_major: i32,
    pub(super) gl_minor: i32,
    pub(super) gl_profile: i32,
}

impl<'a> Builder<'a> {
    pub fn new(title: &'a str) -> Self {
        Self {
            title,

            init_flag: init::VIDEO,

            win_pos: (window::POS_CENTERED, window::POS_CENTERED, 800, 640),
            win_flag: window::SHOWN | window::OPENGL,

            gl_major: 4,
            gl_minor: 6,
            gl_profile: gl::PROFILE_CORE,
        }
    }

    pub fn build(self) -> Context {
        Context::new(self)
    }
}
