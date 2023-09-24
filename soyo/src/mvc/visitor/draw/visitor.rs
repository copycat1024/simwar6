use super::Zone;
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
    fn render<R: Render>(&mut self, host: &mut Renderer<R>) {
        let Self { ctx } = self;
        let Renderer { widget, attr } = host;

        let frame = attr.frame;
        let rect = frame.rect();

        let mut zone = Zone::new(*attr);
        zone.collect(widget.render(rect));

        ctx.render(zone);
    }
}
