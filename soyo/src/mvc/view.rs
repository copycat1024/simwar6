use super::visitor::{DrawVisitor, RedrawVisitor, TickVisitor};
use crate::{
    gfx::{Backend, Event},
    util::Frame,
    widget::{Compose, Composer, Host, Widget},
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

    pub fn handle_event(&mut self, event: Event) {
        match event {
            Event::Resize { w, h } => {
                self.resize(w, h);
            }
            Event::Update { delta } => {
                let delta = delta.as_millis() as u64;
                self.tick(delta);
            }
            _ => {}
        }
    }

    pub fn resize(&mut self, w: i32, h: i32) {
        self.screen = Frame::screen(w, h);
    }

    fn tick(&mut self, delta: u64) {
        let mut visitor = TickVisitor::new(delta);
        self.root.accept_visitor(&mut visitor);
    }

    pub fn draw<B>(&mut self, ctx: &mut B)
    where
        B: Backend<Frag = T::Frag>,
    {
        self.root.compose(self.screen);

        let mut visitor = RedrawVisitor::default();
        self.root.accept_visitor(&mut visitor);

        if visitor.redraw {
            // emit draw commands
            let mut visitor = DrawVisitor::new(ctx);
            self.root.accept_visitor(&mut visitor);

            // execute draw commands
            ctx.draw();
        }
    }

    pub fn node(&self) -> &Composer<T> {
        &self.root
    }

    pub fn node_mut(&mut self) -> &mut Composer<T> {
        &mut self.root
    }

    pub fn handle(&mut self) -> <T as Widget>::Handle<'_> {
        self.root.handle()
    }
}
