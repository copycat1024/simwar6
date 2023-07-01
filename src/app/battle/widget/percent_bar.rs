use crate::util::math::Scale;
use soyo::{
    tui::{Letter, Quad},
    view::{Frame, Render},
};

const PARTS: [char; 5] = [' ', '\u{258e}', '\u{258c}', '\u{258a}', '\u{2588}'];
const PART_COUNT: i32 = (PARTS.len() as i32) - 1;

#[derive(Default)]
pub struct PercentBar {
    block: i32, // block count
    part: i32,  // part count
    scale: Scale,
}

impl PercentBar {
    pub fn update(&mut self, value: i32, max: i32) {
        self.scale.set_value_and_max(value, max);
    }

    fn calc(&mut self) {
        use std::cmp::max;

        let value = max(self.scale.value(), 0);

        self.block = value / PART_COUNT;
        self.part = value % PART_COUNT;
    }
}

impl Render for PercentBar {
    fn render(&self, quad: Quad, letter: &mut Letter) {
        use std::cmp::Ordering;

        *letter.c = match quad.x.cmp(&self.block) {
            Ordering::Greater => PARTS[0],
            Ordering::Less => PARTS[PARTS.len() - 1],
            Ordering::Equal => PARTS[self.part as usize],
        };
    }

    fn layout(&mut self, frame: &mut Frame) {
        self.scale.set_scale(frame.w * PART_COUNT);
        self.calc();
    }
}
