use super::Compose;
use crate::view::{Attribute, Frame, Host, Visitor};

pub struct Composer<T: Compose> {
    pub widget: T,
    pub attr: Attribute,
}

impl<T: Compose> Composer<T> {
    pub fn new(mut widget: T) -> Self {
        widget.register();

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

    pub fn tick(&mut self, delta: u64) -> bool {
        self.widget.tick(delta)
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
