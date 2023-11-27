use crate::{
    gfx::{Context, Fragment},
    view::{Render, Renderer, Visitor},
};

pub struct DrawVisitor<'a, F>
where
    F: Fragment,
{
    ctx: &'a mut Context<F>,
}

impl<'a, F> DrawVisitor<'a, F>
where
    F: Fragment,
{
    pub fn new(ctx: &'a mut Context<F>) -> Self {
        Self { ctx }
    }
}

impl<'a, F> Visitor<F> for DrawVisitor<'a, F>
where
    F: Fragment,
{
    fn render<R>(&mut self, host: &mut Renderer<R>)
    where
        R: Render<Frag = F>,
    {
        host.render(self.ctx);
    }
}
