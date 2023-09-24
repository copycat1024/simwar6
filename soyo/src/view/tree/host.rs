use crate::view::tree::Visitor;

pub trait Host {
    fn accept_visitor<V: Visitor>(&mut self, v: &mut V);
}
