use super::{gl, Window};
use std::os::raw::c_int;

pub struct Sdl {}

extern "C" {
    fn SDL_Init(flags: u32) -> c_int;
    fn SDL_Quit() -> c_int;

    fn SDL_Delay(ms: u32);

    fn SDL_GL_SetAttribute(attr: c_int, value: c_int) -> c_int;
}

impl Sdl {
    pub fn new(flag: u32) -> Self {
        unsafe { SDL_Init(flag) };
        Self {}
    }

    pub fn delay(&self, ms: u32) {
        unsafe { SDL_Delay(ms) };
    }

    pub fn create_window(&self, title: &str, x: i32, y: i32, w: i32, h: i32, flag: u32) -> Window {
        Window::new(title, x, y, w, h, flag)
    }

    pub fn gl_set_attr(&self, attr: gl::Attr, value: i32) {
        unsafe { SDL_GL_SetAttribute(attr as i32, value) };
    }
}

impl Drop for Sdl {
    fn drop(&mut self) {
        unsafe { SDL_Quit() };
    }
}
