use crate::gfx::{Color, Fragment};

#[derive(Clone)]
pub struct Slot {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub fg: Color,
    pub bg: Color,
    pub c: char,
}

impl Fragment for Slot {}
