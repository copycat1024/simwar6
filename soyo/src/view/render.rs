use super::Symbol;
use crate::{
    gfx::Rect,
    view::{Attribute, Frame, Host, Visitor},
};

pub trait Render: 'static {
    fn render(&self, rect: Rect) -> Vec<Symbol>;

    fn layout(&mut self, _: &mut Frame) {}

    fn tick(&mut self, _: u64) -> bool {
        false
    }
}

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
