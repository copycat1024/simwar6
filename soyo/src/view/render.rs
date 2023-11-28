use crate::{
    gfx::{Backend, Fragment},
    view::{Attribute, Frame, Host, Visitor},
};

pub trait Render: 'static {
    type Frag: Fragment;

    fn render(&self, attr: &Attribute) -> Vec<Self::Frag>;

    fn layout(&mut self, _: &mut Frame) {}

    fn tick(&mut self, _: u64) -> bool {
        false
    }
}

pub struct Renderer<T>
where
    T: Render,
{
    pub widget: T,
    pub attr: Attribute,
}

impl<T> Renderer<T>
where
    T: Render,
{
    pub fn new(widget: T) -> Self {
        Self {
            widget,
            attr: Attribute::default(),
        }
    }

    pub fn layout(&mut self, frame: Frame) -> Frame {
        self.attr.frame = (self.attr.layout_fn)(frame);
        self.widget.layout(&mut self.attr.frame);
        self.attr.frame
    }

    pub fn render<B>(&self, ctx: &mut B)
    where
        B: Backend<Frag = T::Frag>,
    {
        let Self { widget, attr, .. } = self;
        let items = widget.render(attr);
        ctx.push(items);
    }
}

impl<T> Default for Renderer<T>
where
    T: Render + Default,
{
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T> Host<T::Frag> for Renderer<T>
where
    T: Render,
{
    fn accept_visitor<V: Visitor<T::Frag>>(&mut self, v: &mut V) {
        v.render(self);
    }
}
