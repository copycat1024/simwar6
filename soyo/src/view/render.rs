use crate::{
    gfx::{Backend, Fragment},
    util::Frame,
    view::{Common, Host, Visitor, Widget},
};

pub trait Render: Widget {
    type Frag: Fragment;

    fn render(&self, common: &Common) -> Vec<Self::Frag>;

    fn layout(&mut self, _: &mut Frame) {}

    fn tick(&mut self, _: u64) -> bool {
        false
    }
}

pub struct Renderer<T>
where
    T: Render,
{
    widget: T,
    common: Common,
}

impl<T> Renderer<T>
where
    T: Render,
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

    pub fn layout(&mut self, frame: Frame) -> Frame {
        self.common.frame = (self.common.layout_fn)(frame);
        self.widget.layout(&mut self.common.frame);
        self.common.frame
    }

    pub fn render<B>(&self, ctx: &mut B)
    where
        B: Backend<Frag = T::Frag>,
    {
        let Self { widget, common, .. } = self;
        let items = widget.render(common);
        ctx.push(items);
    }

    pub fn need_redraw(&self) -> bool {
        if self.common.redraw.peek() {
            println!("peek");
        }
        self.common.redraw.get()
    }

    pub fn tick(&mut self, delta: u64) -> bool {
        self.widget.tick(delta)
    }

    pub fn set_frame(&mut self, frame: Frame) {
        self.common.frame = frame;
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
