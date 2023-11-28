use crate::{
    gfx::Backend,
    view::{Render, Renderer, Visitor},
};

pub struct DrawVisitor<'a, B>
where
    B: Backend,
{
    ctx: &'a mut B,
}

impl<'a, B> DrawVisitor<'a, B>
where
    B: Backend,
{
    pub fn new(ctx: &'a mut B) -> Self {
        Self { ctx }
    }
}

impl<'a, B> Visitor<B::Frag> for DrawVisitor<'a, B>
where
    B: Backend,
{
    fn render<R>(&mut self, host: &mut Renderer<R>)
    where
        R: Render<Frag = B::Frag>,
    {
        host.render(self.ctx);
    }
}
