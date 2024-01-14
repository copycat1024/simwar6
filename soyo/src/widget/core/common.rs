use crate::util::{Frame, Latch};

pub struct Common {
    pub frame: Frame,
    pub fill: bool,
    pub layout_fn: fn(Frame) -> Frame,
    pub redraw: Latch,
}

impl Default for Common {
    fn default() -> Self {
        Self {
            frame: Frame::screen(0, 0),
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
            fill: true,
            layout_fn: |f| f,
            redraw: Latch::new(true),
        }
    }

    pub fn set_redraw(&mut self) {
        self.redraw.set()
    }

    pub fn cmp_and_set<T>(&mut self, dst: &mut T, src: T)
    where
        T: PartialEq,
    {
        cmp_and_set(&mut self.redraw, dst, src);
    }
}

fn cmp_and_set<T>(redraw: &mut Latch, dst: &mut T, src: T)
where
    T: PartialEq,
{
    if *dst != src {
        redraw.set();
        *dst = src;
    }
}
