use super::{Symbol, Zone};
use crate::{
    gfx::{Context, Rect},
    view::{Attribute, Frame, Host, Visitor},
};

pub trait Render: 'static {
    fn render(&self, rect: Rect) -> Vec<Symbol> {
        rect.iter(false)
            .filter_map(|(x, y)| {
                let mut sym = Symbol::new(rect.x + x, rect.y + y, '\0');
                let rect = Rect::xywh(x, y, rect.w, rect.h);
                self.render_rel(rect, &mut sym);
                (sym.c != '\0').then_some(sym)
            })
            .collect()
    }

    fn render_rel(&self, _rect: Rect, _sym: &mut Symbol) {}

    fn layout(&mut self, _: &mut Frame) {}

    fn tick(&mut self, _: u64) -> bool {
        false
    }
}

pub struct Renderer<T: Render> {
    pub widget: T,
    pub attr: Attribute,
}

impl<T: Render> Renderer<T> {
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

    pub fn render(&self, ctx: &mut Context) {
        let frame = self.attr.frame;
        let rect = frame.rect();

        let mut zone = Zone::new(self.attr);
        zone.collect(self.widget.render(rect));

        ctx.render(zone);
    }
}

impl<T: Render + Default> Default for Renderer<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T: Render> Host for Renderer<T> {
    fn accept_visitor<V: Visitor>(&mut self, v: &mut V) {
        v.render(self);
    }
}
