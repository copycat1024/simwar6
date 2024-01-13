use crate::{
    gfx::Fragment,
    util::Frame,
    view::{Common, Host, Visitor},
};

pub trait Compose: 'static {
    type Frag: Fragment;
    fn propagate<V: Visitor<Self::Frag>>(&mut self, v: &mut V);
    fn compose(&mut self, _: &mut Frame) {}
}

pub struct Composer<T>
where
    T: Compose,
{
    pub widget: T,
    pub common: Common,
}

impl<T> Composer<T>
where
    T: Compose,
{
    pub fn new(widget: T) -> Self {
        Self {
            widget,
            common: Common::default(),
        }
    }

    pub fn compose(&mut self, frame: Frame) -> Frame {
        self.common.frame = frame;
        self.widget.compose(&mut self.common.frame);
        self.common.frame
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
