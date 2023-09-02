use super::{Render, Zone};
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

        let mut zone = Zone::new(self.attr);
        zone.collect(self.widget.render(rect, frame.z_value()));

        ctx.render(zone);
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
