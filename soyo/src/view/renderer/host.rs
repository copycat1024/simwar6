use super::{Render, Zone};
use crate::{
    gfx::Context,
    view::{Attribute, Frame, Host, Visitor},
};

pub struct Renderer<T: Render> {
    pub widget: T,
    pub attr: Attribute,
}

impl<T: Render> Renderer<T> {
    pub fn new(widget: T) -> Self {
        Self {
            widget,
            attr: Attribute::default(),
        }
    }

    pub fn layout(&mut self, frame: Frame) -> Frame {
        self.attr.frame = (self.attr.layout_fn)(frame);
        self.widget.layout(&mut self.attr.frame);
        self.attr.frame
    }

    pub fn render(&self, ctx: &mut Context) {
        let frame = self.attr.frame;
        let rect = frame.rect();

        let mut zone = Zone::new(self.attr);
        zone.collect(self.widget.render(rect));

        ctx.render(zone);
    }

    pub fn tick(&mut self, delta: u64) -> bool {
        self.widget.tick(delta)
    }
}

impl<T: Render + Default> Default for Renderer<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T: Render> Host for Renderer<T> {
    fn accept_visitor<V: Visitor>(&mut self, v: &mut V) {
        v.render(self);
    }
}
