use crate::view::{Attribute, Frame, Host, Visitor};

pub trait Compose: 'static {
    fn propagate<V: Visitor>(&mut self, v: &mut V);
    fn layout(&mut self, _: &mut Frame) {}
}

pub struct Composer<T: Compose> {
    pub widget: T,
    pub attr: Attribute,
}

impl<T: Compose> Composer<T> {
    pub fn new(widget: T) -> Self {
        Self {
            widget,
            attr: Attribute::default(),
        }
    }

    pub fn layout(&mut self, frame: Frame) -> Frame {
        self.attr.frame = frame;
        self.widget.layout(&mut self.attr.frame);
        self.attr.frame
    }
}

impl<T: Compose + Default> Default for Composer<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T: Compose> Host for Composer<T> {
    fn accept_visitor<V: Visitor>(&mut self, v: &mut V) {
        v.precompose(self);
        self.widget.propagate(v);
        v.postcompose(self);
    }
}
