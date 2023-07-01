#[derive(Default)]
pub struct Timer {
    pub t0: u64,
    t: u64,
}

impl Timer {
    pub fn set(&mut self) {
        self.t = self.t0;
    }

    pub fn tick(&mut self, delta: u64) -> bool {
        if self.t == 0 {
            false
        } else if self.t > delta {
            self.t -= delta;
            false
        } else {
            self.t = 0;
            true
        }
    }
}
