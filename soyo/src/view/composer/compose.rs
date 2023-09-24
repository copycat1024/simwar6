use crate::view::{Frame, Visitor};

pub trait Compose: 'static {
    fn register(&mut self);

    fn layout(&mut self, _: &mut Frame) {}

    fn tick(&mut self, _: u64) -> bool {
        false
    }

    fn propagate<V: Visitor>(&mut self, v: &mut V);
}
