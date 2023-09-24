use crate::view::{Compose, Composer, Render, Renderer};

pub trait Visitor {
    fn render<R: Render>(&mut self, _host: &mut Renderer<R>) {}
    fn precompose<C: Compose>(&mut self, _host: &mut Composer<C>) {}
    fn postcompose<C: Compose>(&mut self, _host: &mut Composer<C>) {}
}

pub trait Host {
    fn accept_visitor<V: Visitor>(&mut self, v: &mut V);
}
