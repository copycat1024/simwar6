use std::cell::Cell;

pub struct Latch {
    flag: Cell<bool>,
}

impl Latch {
    pub fn set(&mut self) {
        self.flag.set(true);
    }

    pub fn follow(&mut self, input: bool) {
        let flag = self.flag.get();
        self.flag.set(flag || input);
    }

    pub fn get(&self) -> bool {
        let flag = self.flag.get();
        self.flag.set(false);
        flag
    }

    pub fn peek(&mut self) -> bool {
        self.flag.get()
    }
}

impl Default for Latch {
    fn default() -> Self {
        Self {
            flag: Cell::new(false),
        }
    }
}
