use super::Builder;
use crate::{
    gl::{enums::ClearBufferMask, Gl, GlContext},
    sdl::{self, gl::Attr, Sdl, Window},
};

pub type Event = sdl::Event;

pub struct Context {
    sdl: Sdl,
    gl: GlContext,
    window: Window,
}

impl Context {
    pub(super) fn new(builder: Builder) -> Self {
        let Builder {
            title,
            init_flag,

            win_pos: (x, y, w, h),
            win_flag,

            gl_major,
            gl_minor,
            gl_profile,
        } = builder;

        let sdl = Sdl::new(init_flag);

        sdl.gl_set_attr(Attr::MajorVersion, gl_major);
        sdl.gl_set_attr(Attr::MinorVersion, gl_minor);
        sdl.gl_set_attr(Attr::ProfileMask, gl_profile);

        let window = sdl.create_window(title, x, y, w, h, win_flag);
        let gl = GlContext::new(&window);

        Self { sdl, gl, window }
    }

    pub fn delay(&self, ms: u32) {
        self.sdl.delay(ms);
    }

    pub fn clear(&self) {
        let gl = self.gl();
        gl.clear_color(0., 0., 0., 1.);
        gl.clear(ClearBufferMask::ColorBufferBit);
    }

    pub fn swap(&self) {
        self.window.swap()
    }

    pub fn gl(&self) -> &Gl {
        self.gl.gl()
    }

    pub fn poll(&self) -> Option<Event> {
        self.window.poll()
    }

    pub fn tick(&self) -> u64 {
        self.window.tick()
    }
}
