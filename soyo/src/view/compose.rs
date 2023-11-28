use crate::{
    gfx::Fragment,
    view::{Attribute, Frame, Host, Visitor},
};

pub trait Compose: 'static {
    type Frag: Fragment;
    fn propagate<V: Visitor<Self::Frag>>(&mut self, v: &mut V);
    fn layout(&mut self, _: &mut Frame) {}
}

pub struct Composer<T>
where
    T: Compose,
{
    pub widget: T,
    pub attr: Attribute,
}

impl<T> Composer<T>
where
    T: Compose,
{
    pub fn new(widget: T) -> Self {
        Self {
            widget,
            attr: Attribute::default(),
        }
    }

    pub fn layout(&mut self, frame: Frame) -> Frame {
        self.attr.frame = frame;
        self.widget.layout(&mut self.attr.frame);
        self.attr.frame
    }
}

impl<T> Default for Composer<T>
where
    T: Compose + Default,
{
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T> Host<T::Frag> for Composer<T>
where
    T: Compose,
{
    fn accept_visitor<V: Visitor<T::Frag>>(&mut self, v: &mut V) {
        v.before_render(self);
        self.widget.propagate(v);
        v.after_render(self);
    }
}
