use std::{cell::Cell, ops::BitOrAssign};

pub struct Latch {
    flag: Cell<bool>,
}

impl Latch {
    pub fn set(&mut self) {
        self.flag.set(true);
    }

    pub fn clear(&mut self) {
        self.flag.set(false);
    }

    pub fn get(&self) -> bool {
        let flag = self.flag.get();
        self.flag.set(false);
        flag
    }

    pub fn peek(&mut self) -> bool {
        self.flag.get()
    }

    fn follow(&mut self, input: bool) {
        let flag = self.flag.get();
        self.flag.set(flag || input);
    }
}

impl Default for Latch {
    fn default() -> Self {
        Self {
            flag: Cell::new(false),
        }
    }
}

impl BitOrAssign<&Self> for Latch {
    fn bitor_assign(&mut self, rhs: &Self) {
        self.follow(rhs.get())
    }
}

impl BitOrAssign<bool> for Latch {
    fn bitor_assign(&mut self, rhs: bool) {
        self.follow(rhs)
    }
}
