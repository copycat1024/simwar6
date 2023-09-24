use super::Symbol;
use crate::{gfx::Rect, view::Frame};

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
