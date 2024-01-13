use super::Widget;
use crate::{
    gfx::Fragment,
    util::Frame,
    view::{Common, Host, Visitor},
};

pub trait Compose: Widget {
    type Frag: Fragment;

    fn compose(&mut self, frame: &mut Frame);
    fn propagate<V: Visitor<Self::Frag>>(&mut self, v: &mut V);
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

    pub fn handle(&mut self) -> T::Handle<'_> {
        let Self { widget, common } = self;
        widget.handle(common)
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
