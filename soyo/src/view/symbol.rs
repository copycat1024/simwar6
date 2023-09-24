use crate::{
    gfx::{Color, Slot},
    view::Attribute,
};

pub struct Symbol {
    pub x: i32,
    pub y: i32,
    pub z: Option<i32>,
    pub c: char,
    pub fg: Option<Color>,
    pub bg: Option<Color>,
}

impl Symbol {
    pub fn new(x: i32, y: i32, c: char) -> Self {
        Self {
            x,
            y,
            z: None,
            c,
            fg: None,
            bg: None,
        }
    }

    pub fn set_z(self, z: i32) -> Self {
        let Self {
            x, y, c, fg, bg, ..
        } = self;
        Self {
            x,
            y,
            z: Some(z),
            c,
            bg,
            fg,
        }
    }

    pub fn set_fg(self, fg: Color) -> Self {
        let Self { x, y, z, c, bg, .. } = self;
        Self {
            x,
            y,
            z,
            c,
            fg: Some(fg),
            bg,
        }
    }

    pub fn set_bg(self, bg: Color) -> Self {
        let Self { x, y, z, c, fg, .. } = self;
        Self {
            x,
            y,
            z,
            c,
            fg,
            bg: Some(bg),
        }
    }

    pub fn to_slot(&self, attr: &Attribute) -> Slot {
        let Self { x, y, z, c, bg, fg } = self;
        Slot {
            x: x + attr.frame.x,
            y: y + attr.frame.y,
            z: z.unwrap_or(attr.frame.z_value()),
            c: *c,
            bg: bg.unwrap_or(attr.bg),
            fg: fg.unwrap_or(attr.fg),
        }
    }
}
