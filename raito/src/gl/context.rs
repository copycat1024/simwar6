use super::{Gl, GlCmd};
use crate::sdl::Window;

pub struct GlContext {
    gl: Gl,
}

impl GlContext {
    pub fn new(window: &Window) -> Self {
        let cmd = GlCmd::load(|name| unsafe { window.load(name) });
        Self { gl: Gl::new(cmd) }
    }

    pub fn gl(&self) -> &Gl {
        &self.gl
    }
}
