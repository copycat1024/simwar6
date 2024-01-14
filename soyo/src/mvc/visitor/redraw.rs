use crate::{
    gfx::Fragment,
    widget::{Render, Renderer, Visitor},
};

pub struct RedrawVisitor {
    pub redraw: bool,
}

#[allow(clippy::derivable_impls)]
impl Default for RedrawVisitor {
    fn default() -> Self {
        Self { redraw: false }
    }
}

impl<F> Visitor<F> for RedrawVisitor
where
    F: Fragment,
{
    fn render<R: Render>(&mut self, host: &mut Renderer<R>) {
        self.redraw |= host.need_redraw();
    }
}
