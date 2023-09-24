use crate::view::{Compose, Composer, Frame, Render, Renderer, Visitor};

pub struct LayoutVisitor {
    stack: Vec<Frame>,
}

impl LayoutVisitor {
    pub fn new(screen: Frame) -> Self {
        Self {
            stack: vec![screen],
        }
    }
}

impl Visitor for LayoutVisitor {
    fn precompose<C: Compose>(&mut self, host: &mut Composer<C>) {
        let frame = *self.stack.last().expect("Empty stack in LayoutVisitor");
        let frame = host.layout(frame);
        self.stack.push(frame);
    }

    fn render<R: Render>(&mut self, host: &mut Renderer<R>) {
        let frame = *self.stack.last().expect("Empty stack in LayoutVisitor");
        host.layout(frame);
    }

    fn postcompose<C: Compose>(&mut self, _host: &mut Composer<C>) {
        self.stack.pop();
    }
}
