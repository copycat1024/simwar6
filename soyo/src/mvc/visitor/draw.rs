use crate::{
    gfx::Context,
    view::{Render, Renderer, Visitor},
};

pub struct DrawVisitor<'a> {
    ctx: &'a mut Context,
}
impl<'a> DrawVisitor<'a> {
    pub fn new(ctx: &'a mut Context) -> Self {
        Self { ctx }
    }
}

impl<'a> Visitor for DrawVisitor<'a> {
    fn render<R: Render>(&mut self, widget: &mut Renderer<R>) {
        widget.render(self.ctx);
    }
}
