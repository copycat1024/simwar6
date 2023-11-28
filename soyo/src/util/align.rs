use std::cmp::min;

pub enum AlignX {
    Left,
    Center,
    Right,
}

impl AlignX {
    pub fn offset(&self, parent: i32, child: i32) -> AlignOffset {
        let coeff = match self {
            Self::Left => 0,
            Self::Center => 1,
            Self::Right => 2,
        };

        AlignOffset::new(coeff, parent, child)
    }
}

pub struct AlignOffset {
    pub parent: i32,
    pub child: i32,
    pub len: i32,
}

impl AlignOffset {
    fn new(coeff: i32, parent: i32, child: i32) -> Self {
        let len = min(parent, child);

        Self {
            parent: (parent - len) * coeff / 2,
            child: (child - len) * coeff / 2,
            len,
        }
    }
}
