use crate::{
    gfx::Fragment,
    view::{Render, Renderer, Visitor},
};

pub struct TickVisitor {
    delta: u64,
    pub draw: bool,
}

impl TickVisitor {
    pub fn new(delta: u64) -> Self {
        Self { delta, draw: false }
    }
}

impl<F> Visitor<F> for TickVisitor
where
    F: Fragment,
{
    fn render<R: Render>(&mut self, host: &mut Renderer<R>) {
        let Renderer { widget, .. } = host;
        self.draw |= widget.tick(self.delta);
    }
}
