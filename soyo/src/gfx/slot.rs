use crate::gfx::{Color, Letter};

#[derive(Clone)]
pub struct Slot {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub letter: Letter,
}

impl Slot {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self {
            x,
            y,
            z,
            letter: Letter {
                fg: Color::WHITE.into(),
                bg: Color::BLACK.into(),
                c: '\0'.into(),
            },
        }
    }
}
