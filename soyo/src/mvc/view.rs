use super::{
    visitor::{DrawVisitor, TickVisitor},
    Flow,
};
use crate::{
    gfx::Context,
    util::Result,
    view::{Compose, Composer, Frame, Host},
};

pub struct View<T: Compose> {
    root_ref: Composer<T>,
    screen: Frame,
}

impl<T: 'static + Compose> View<T> {
    pub fn new(node: T) -> Self {
        Self {
            root_ref: Composer::new(node),
            screen: Frame::screen(0, 0),
        }
    }

    pub fn resize(&mut self, w: i32, h: i32, flow: &mut Flow) -> Result {
        flow.draw.set();
        self.screen = Frame::screen(w, h);
        Ok(())
    }

    pub fn tick(&mut self, delta: u64, flow: &mut Flow) {
        let mut visitor = TickVisitor::new(delta);
        self.root_ref.accept_visitor(&mut visitor);
        flow.draw.follow(visitor.draw);
    }

    pub fn compose(&mut self) {
        self.root_ref.layout(self.screen);
    }

    pub fn draw(&mut self, ctx: &mut Context) -> Result {
        let mut visitor = DrawVisitor::new(ctx);
        self.root_ref.accept_visitor(&mut visitor);
        ctx.draw()?;

        Ok(())
    }

    pub fn node(&self) -> &Composer<T> {
        &self.root_ref
    }

    pub fn node_mut(&mut self) -> &mut Composer<T> {
        &mut self.root_ref
    }
}
