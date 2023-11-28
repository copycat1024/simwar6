use crate::widget::Menu;
use soyo::{
    gfx::{Color, Slot},
    view::{Compose, Composer, Frame, Host, Renderer, Visitor},
    widget::{ILabel, Label},
};

pub struct View {
    top: Renderer<Label>,
    pub menu: Composer<Menu>,
}

impl View {
    pub fn write_top(&mut self, text: &str) {
        write!(self.top, "{}", text);
    }
}

impl Compose for View {
    type Frag = Slot;

    fn propagate<V: Visitor<Self::Frag>>(&mut self, v: &mut V) {
        self.top.accept_visitor(v);
        self.menu.accept_visitor(v);
    }

    fn layout(&mut self, frame: &mut Frame) {
        self.top.layout(frame.set_h(1).rise_z());

        self.menu.layout(
            frame
                .set_x(frame.w / 3)
                .set_y(5)
                .set_w(frame.w / 3)
                .set_h(frame.h - 11)
                .rise_z(),
        );
    }
}

impl Default for View {
    fn default() -> Self {
        let mut top = Renderer::default();
        top.attr.bg = Color::RED;

        let mut menu = Composer::default();
        menu.attr.bg = Color::RED;

        Self { top, menu }
    }
}
