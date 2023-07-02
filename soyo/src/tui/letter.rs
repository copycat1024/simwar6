use crate::tui::Color;

#[derive(Clone)]
pub struct Letter {
    pub fg: Color,
    pub bg: Color,
    pub c: char,
}

impl Letter {
    pub fn hot(&self) -> bool {
        self.c != '\0'
    }
}
