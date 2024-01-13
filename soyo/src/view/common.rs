use crate::{
    gfx::Color,
    util::{Frame, Latch},
};

pub struct Common {
    pub frame: Frame,
    pub fg: Color,
    pub bg: Color,
    pub fill: bool,
    pub layout_fn: fn(Frame) -> Frame,
    pub redraw: Latch,
}

impl Default for Common {
    fn default() -> Self {
        Self {
            frame: Frame::screen(0, 0),
            fg: Color::WHITE,
            bg: Color::BLACK,
            fill: true,
            layout_fn: |f| f,
            redraw: Latch::new(true),
        }
    }
}

impl Common {
    pub fn from_size(w: i32, h: i32) -> Self {
        Self {
            frame: Frame::screen(w, h),
            fg: Color::WHITE,
            bg: Color::BLACK,
            fill: true,
            layout_fn: |f| f,
            redraw: Latch::new(true),
        }
    }

    pub fn set_redraw(&mut self) {
        self.redraw.set()
    }
}
