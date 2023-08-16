use crate::{
    gfx::{Letter, Quad, Slot},
    view::Frame,
};

pub trait Render: 'static {
    fn render(&self, quad: Quad, z: i32) -> Vec<Slot> {
        quad.iter(false)
            .filter_map(|(x, y)| {
                let mut slot = Slot::new(quad.x + x, quad.y + y, z);
                let quad = Quad::xywh(x, y, quad.w, quad.h);
                self.render_rel(quad, &mut slot.letter);
                (slot.letter.c != '\0').then_some(slot)
            })
            .collect()
    }

    fn render_rel(&self, _quad: Quad, _letter: &mut Letter) {}

    fn layout(&mut self, _: &mut Frame) {}

    fn tick(&mut self, _: u64) -> bool {
        false
    }
}
