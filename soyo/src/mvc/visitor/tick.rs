use crate::view::{Render, Renderer, Visitor};

pub struct TickVisitor {
    delta: u64,
    pub draw: bool,
}

impl TickVisitor {
    pub fn new(delta: u64) -> Self {
        Self { delta, draw: false }
    }
}

impl Visitor for TickVisitor {
    fn precompose<C: crate::view::Compose>(&mut self, host: &mut crate::view::Composer<C>) {
        self.draw |= host.tick(self.delta);
    }

    fn render<R: Render>(&mut self, host: &mut Renderer<R>) {
        self.draw |= host.tick(self.delta);
    }
}
