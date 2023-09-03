use crate::{
    gfx::{Color, Slot},
    view::Attribute,
};

pub struct Symbol {
    pub x: i32,
    pub y: i32,
    pub z: Option<i32>,
    pub c: char,
    pub bg: Option<Color>,
    pub fg: Option<Color>,
}

impl Symbol {
    pub fn new(x: i32, y: i32, c: char) -> Self {
        Self {
            x,
            y,
            z: None,
            c,
            bg: None,
            fg: None,
        }
    }

    pub fn to_slot(self, attr: &Attribute) -> Slot {
        let Self { x, y, z, c, bg, fg } = self;
        Slot {
            x,
            y,
            z: z.unwrap_or(attr.frame.z_value()),
            c,
            bg: bg.unwrap_or(attr.bg),
            fg: fg.unwrap_or(attr.fg),
        }
    }
}
