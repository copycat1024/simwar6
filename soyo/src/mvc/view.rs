use super::{
    visitor::{DrawVisitor, TickVisitor},
    Flow,
};
use crate::{
    gfx::{Context, Event},
    view::{Compose, Composer, Frame, Host},
};

pub struct View<T>
where
    T: Compose,
{
    root: Composer<T>,
    screen: Frame,
}

impl<T> View<T>
where
    T: Compose,
{
    pub fn new(node: T) -> Self {
        Self {
            root: Composer::new(node),
            screen: Frame::screen(0, 0),
        }
    }

    pub fn handle_event(&mut self, event: Event, flow: &mut Flow) {
        match event {
            Event::Resize { w, h } => {
                self.resize(w, h, flow);
            }
            Event::Update { delta } => {
                let delta = delta.as_millis() as u64;
                self.tick(delta, flow);
            }
            _ => {}
        }
    }

    pub fn resize(&mut self, w: i32, h: i32, flow: &mut Flow) {
        flow.draw.set();
        self.screen = Frame::screen(w, h);
    }

    fn tick(&mut self, delta: u64, flow: &mut Flow) {
        let mut visitor = TickVisitor::new(delta);
        self.root.accept_visitor(&mut visitor);
        flow.draw |= visitor.draw;
    }

    pub fn compose(&mut self) {
        self.root.layout(self.screen);
    }

    pub fn draw(&mut self, ctx: &mut Context<T::Frag>) {
        let mut visitor = DrawVisitor::new(ctx);
        self.root.accept_visitor(&mut visitor);
        ctx.draw();
    }

    pub fn node(&self) -> &Composer<T> {
        &self.root
    }

    pub fn node_mut(&mut self) -> &mut Composer<T> {
        &mut self.root
    }
}
