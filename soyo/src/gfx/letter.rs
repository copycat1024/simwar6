use crate::gfx::Color;

#[derive(Clone)]
pub struct Letter {
    pub fg: Color,
    pub bg: Color,
    pub c: char,
}
