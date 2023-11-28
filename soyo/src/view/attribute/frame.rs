use crate::gfx::Rect;

#[derive(Clone, Copy, Default)]
pub struct Frame {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub w: i32,
    pub h: i32,
}

impl Frame {
    pub fn screen(w: i32, h: i32) -> Self {
        Self {
            x: 0,
            y: 0,
            z: 0,
            w,
            h,
        }
    }

    pub fn rise_z(self) -> Self {
        self.set_z(self.z + 1)
    }

    pub fn center(self, w: i32, h: i32) -> Self {
        Self {
            x: self.x + (self.w - w) / 2,
            y: self.y + (self.h - h) / 2,
            w,
            h,
            z: self.z,
        }
    }
}

impl Frame {
    pub fn set_x(self, x: i32) -> Self {
        Self {
            x,
            y: self.y,
            w: self.w,
            h: self.h,
            z: self.z,
        }
    }

    pub fn set_y(self, y: i32) -> Self {
        Self {
            x: self.x,
            y,
            w: self.w,
            h: self.h,
            z: self.z,
        }
    }

    pub fn set_w(self, w: i32) -> Self {
        Self {
            x: self.x,
            y: self.y,
            w,
            h: self.h,
            z: self.z,
        }
    }

    pub fn set_h(self, h: i32) -> Self {
        Self {
            x: self.x,
            y: self.y,
            w: self.w,
            h,
            z: self.z,
        }
    }

    pub fn set_z(self, z: i32) -> Self {
        Self {
            x: self.x,
            y: self.y,
            w: self.w,
            h: self.h,
            z,
        }
    }

    pub fn margin(self, top: i32, bot: i32, left: i32, right: i32) -> Self {
        Self {
            x: self.x + left,
            y: self.y + top,
            w: self.w - left - right,
            h: self.h - top - bot,
            z: self.z,
        }
    }

    pub fn rect(&self) -> Rect {
        Rect {
            x: self.x,
            y: self.y,
            w: self.w,
            h: self.h,
        }
    }

    pub fn z_value(&self) -> i32 {
        self.z
    }
}

impl std::fmt::Debug for Frame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Frame")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("z", &self.z)
            .field("w", &self.w)
            .field("h", &self.h)
            .finish()
    }
}
