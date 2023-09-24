use crate::widget::Menu;
use soyo::{
    gfx::Color,
    view::{Compose, Composer, Frame, Host, Renderer, Visitor},
    widget::Label,
};

#[derive(Default)]
pub struct View {
    top: Renderer<Label>,
    menu: Composer<Menu>,
}

impl Compose for View {
    fn register(&mut self) {
        self.top.attr.bg = Color::RED;
        self.menu.attr.bg = Color::RED;
    }

    fn propagate<V: Visitor>(&mut self, v: &mut V) {
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

impl View {
    pub fn set_menu<'a, T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = &'a str>,
    {
        self.menu.widget.set_list(iter);
    }

    pub fn set_item(&mut self, item: usize) {
        self.menu.widget.set_item(item);
    }

    pub fn write_top(&mut self, text: &str) {
        write!(self.top.widget, "{}", text);
    }
}
