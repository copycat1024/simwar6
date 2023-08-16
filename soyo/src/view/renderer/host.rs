use super::Render;
use crate::{
    gfx::Context,
    view::{Attribute, Frame, Host},
};

pub struct RenderHost<T>
where
    T: Render,
{
    pub widget: T,
    pub attr: Attribute,
}

impl<T: Render> RenderHost<T> {
    pub fn new(widget: T) -> Self {
        Self {
            widget,
            attr: Attribute::default(),
        }
    }
}

impl<T: Render> Host for RenderHost<T> {
    fn render(&self, ctx: &mut Context) {
        use crate::gfx::{Quad, Slot};
        let frame = self.attr.frame;
        let rect = frame.quad();

        let iter = rect.iter(false).filter_map(|(x, y)| {
            let mut slot = Slot::new(rect.x + x, rect.y + y, frame.z_value());
            let quad = Quad::xywh(x, y, rect.w, rect.h);
            slot.letter.fg = self.attr.fg;
            slot.letter.bg = self.attr.bg;
            self.widget.render(quad, &mut slot.letter);
            (slot.letter.c != '\0').then_some(slot)
        });

        ctx.render(iter);
    }

    fn layout(&mut self, frame: Frame) -> Frame {
        self.attr.frame = frame;
        self.widget.layout(&mut self.attr.frame);
        self.attr.frame
    }

    fn tick(&mut self, delta: u64) -> bool {
        self.widget.tick(delta)
    }
}
