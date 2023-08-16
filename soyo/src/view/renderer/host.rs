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
        let frame = self.attr.frame;
        let rect = frame.rect();

        // use crate::gfx::{Rect, Slot};
        // let iter = rect.iter(false).filter_map(|(x, y)| {
        //     let mut slot = Slot::new(rect.x + x, rect.y + y, frame.z_value());
        //     let rect = Rect::xywh(x, y, rect.w, rect.h);
        //     slot.letter.fg = self.attr.fg;
        //     slot.letter.bg = self.attr.bg;
        //     self.widget.render(rect, &mut slot.letter);
        //     (slot.letter.c != '\0').then_some(slot)
        // });

        ctx.render(self.widget.render(rect, frame.z_value()));
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
