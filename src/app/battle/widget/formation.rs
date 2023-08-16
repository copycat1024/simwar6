use crate::util::math::Scale;
use rand_xoshiro::Xoshiro256PlusPlus;
use soyo::{
    gfx::{Letter, Rect},
    view::{Frame, Render},
};

pub enum Orient {
    Left,
    Right,
}

pub enum Status {
    Idle,
    Strike,
}

pub struct Formation {
    symbol: char,
    orient: Orient,
    status: Status,
    units: Vec<i32>,
    rand: Xoshiro256PlusPlus,
    scale: Scale,
}

impl Formation {
    pub fn set_status(&mut self, status: Status) {
        self.status = status;
    }

    pub fn set_orient(&mut self, orient: Orient) {
        self.orient = orient;
    }

    pub fn set_symbol(&mut self, symbol: char) {
        self.symbol = symbol;
    }

    pub fn update(&mut self, value: i32, max: i32) {
        self.scale.set_value_and_max(value, max);
    }

    fn is_line(&self, rect: Rect) -> bool {
        match self.status {
            Status::Idle => rect.x % 2 == 0,
            Status::Strike => rect.x % 2 == 1,
        }
    }

    fn is_margin(&self, rect: Rect) -> bool {
        rect.x < 0 || rect.x / 2 >= rect.w / 2
    }

    fn transform_quad(&self, rect: &mut Rect) {
        rect.x = match self.orient {
            Orient::Right => rect.x - 1,
            Orient::Left => rect.w - rect.x - 2,
        };
        rect.w -= 1;
    }

    fn get_id(&self, mut rect: Rect) -> Option<i32> {
        self.transform_quad(&mut rect);

        if self.is_margin(rect) || !self.is_line(rect) {
            None
        } else {
            let x = rect.x / 2;
            Some(x * rect.h + rect.y)
        }
    }

    fn is_populated(&self, id: i32) -> bool {
        let i = std::cmp::max(id, 0) as usize;
        self.units[i] < self.scale.value()
    }

    fn get_char(&self, is_unit: bool) -> char {
        if is_unit {
            self.symbol
        } else {
            ' '
        }
    }

    fn populate(&mut self, frame: Frame) {
        use crate::util::rand::shuffle;
        use std::cmp::max;

        let count = max(frame.w / 2, 0) * max(frame.h, 0);
        if self.units.len() as i32 != count {
            self.scale.set_scale(count);
            self.units = (0..count).collect();
            shuffle(&mut self.rand, &mut self.units);
        }
    }
}

impl Default for Formation {
    fn default() -> Self {
        use rand::SeedableRng;

        Self {
            symbol: '\u{f054}',
            orient: Orient::Right,
            status: Status::Idle,
            units: Vec::new(),
            rand: Xoshiro256PlusPlus::seed_from_u64(0),
            scale: Scale::default(),
        }
    }
}

impl Render for Formation {
    fn render(&self, rect: Rect, letter: &mut Letter) {
        let populated = if let Some(id) = self.get_id(rect) {
            self.is_populated(id)
        } else {
            false
        };
        letter.c = self.get_char(populated);
    }

    fn layout(&mut self, frame: &mut Frame) {
        self.populate(*frame);
    }
}
