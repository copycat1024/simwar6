use crate::{
    gfx::Fragment,
    widget::{Compose, Composer, Render, Renderer},
};

pub trait Visitor<F: Fragment> {
    fn before_render<C>(&mut self, _host: &mut Composer<C>)
    where
        C: Compose<Frag = F>,
    {
    }

    fn render<R>(&mut self, _host: &mut Renderer<R>)
    where
        R: Render<Frag = F>,
    {
    }

    fn after_render<C>(&mut self, _host: &mut Composer<C>)
    where
        C: Compose<Frag = F>,
    {
    }
}

pub trait Host<F: Fragment> {
    fn accept_visitor<V>(&mut self, v: &mut V)
    where
        V: Visitor<F>;
}
