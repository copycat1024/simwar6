use crate::{
    gfx::{Backend, Color, Letter, Rect, Slot},
    util::Result,
};

#[derive(Default)]
pub struct FrameBuffer {
    buffer: Vec<Slot>,
}

impl FrameBuffer {
    pub fn render<F>(&mut self, rect: Rect, z: i32, renderer: F)
    where
        F: Fn(Rect, &mut Letter),
    {
        let iter = rect.iter(false).filter_map(|(x, y)| {
            let mut slot = Slot::new(rect.x + x, rect.y + y, z);
            let rect = Rect::xywh(x, y, rect.w, rect.h);
            renderer(rect, &mut slot.letter);

            (slot.letter.c != '\0').then_some(slot)
        });
        self.buffer.extend(iter);
    }

    pub fn draw(&self, backend: &mut Box<dyn Backend>) -> Result {
        for slot in self.buffer.iter() {
            backend.bg(slot.letter.bg)?;
            backend.fg(slot.letter.fg)?;
            backend.print(&[slot.clone()])?;
        }
        backend.flush()?;

        Ok(())
    }

    pub fn clear(&mut self, backend: &mut Box<dyn Backend>, _c: Color) -> Result {
        self.buffer.clear();
        backend.clear()?;
        backend.flush()?;

        Ok(())
    }
}
