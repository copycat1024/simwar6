use crate::{
    gfx::{Letter, Rect, Slot},
    view::Frame,
};

pub trait Render: 'static {
    fn render(&self, rect: Rect, z: i32) -> Vec<Slot> {
        rect.iter(false)
            .filter_map(|(x, y)| {
                let mut slot = Slot::new(rect.x + x, rect.y + y, z);
                let rect = Rect::xywh(x, y, rect.w, rect.h);
                self.render_rel(rect, &mut slot.letter);
                (slot.letter.c != '\0').then_some(slot)
            })
            .collect()
    }

    fn render_rel(&self, _rect: Rect, _letter: &mut Letter) {}

    fn layout(&mut self, _: &mut Frame) {}

    fn tick(&mut self, _: u64) -> bool {
        false
    }
}
