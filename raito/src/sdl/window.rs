use std::{
    ffi::CString,
    os::raw::{c_char, c_int, c_void},
};

use super::event::{Event, SdlEvent};

pub const OPENGL: u32 = 2;
pub const SHOWN: u32 = 4;

pub const POS_CENTERED: i32 = 0x2FFF0000;

#[repr(C)]
struct SdlWindow([u8; 0]);

pub struct Window {
    window: *mut SdlWindow,
    gl: *const c_void,
}

extern "C" {
    fn SDL_CreateWindow(
        title: *const c_char,
        x: c_int,
        y: c_int,
        w: c_int,
        h: c_int,
        flags: u32,
    ) -> *mut SdlWindow;
    fn SDL_DestroyWindow(window: *mut SdlWindow);

    fn SDL_GL_CreateContext(window: *mut SdlWindow) -> *const c_void;
    fn SDL_GL_DeleteContext(ctx: *const c_void);

    fn SDL_GL_SwapWindow(window: *mut SdlWindow);
    fn SDL_GL_GetProcAddress(name: *const c_char) -> *mut c_void;
    fn SDL_PollEvent(event: *mut SdlEvent) -> ::std::os::raw::c_int;
    fn SDL_GetTicks64() -> u64;
}

impl Window {
    pub(super) fn new(title: &str, x: i32, y: i32, w: i32, h: i32, flag: u32) -> Self {
        let title = CString::new(title).unwrap();
        unsafe {
            let window = SDL_CreateWindow(title.as_ptr(), x, y, w, h, flag);
            let gl = SDL_GL_CreateContext(window);
            Self { window, gl }
        }
    }

    pub fn swap(&self) {
        unsafe { SDL_GL_SwapWindow(self.window) }
    }

    pub unsafe fn load(&self, name: *const i8) -> *mut c_void {
        unsafe { SDL_GL_GetProcAddress(name) }
    }

    pub fn poll(&self) -> Option<Event> {
        let mut sdl_event = SdlEvent::default();
        let code = unsafe { SDL_PollEvent(&mut sdl_event) };

        if code == 0 {
            None
        } else {
            sdl_event.try_into().ok()
        }
    }

    pub fn tick(&self) -> u64 {
        unsafe { SDL_GetTicks64() }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            SDL_GL_DeleteContext(self.gl);
            SDL_DestroyWindow(self.window);
        }
    }
}
