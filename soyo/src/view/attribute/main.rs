use super::Frame;
use crate::gfx::Color;

#[derive(Clone, Copy)]
pub struct Attribute {
    pub frame: Frame,
    pub fg: Color,
    pub bg: Color,
    pub fill: bool,
}

impl Default for Attribute {
    fn default() -> Self {
        Self {
            frame: Frame::screen(0, 0),
            fg: Color::WHITE,
            bg: Color::BLACK,
            fill: true,
        }
    }
}

impl Attribute {
    pub fn from_size(w: i32, h: i32) -> Self {
        Self {
            frame: Frame::screen(w, h),
            fg: Color::WHITE,
            bg: Color::BLACK,
            fill: true,
        }
    }
}
