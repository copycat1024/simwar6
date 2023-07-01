use crate::widget::Menu;
use soyo::{
    tui::Color,
    view::{Compose, Frame, NodeList, Renderer},
    widget::Label,
};

#[derive(Default)]
pub struct View {
    top: Renderer<Label>,
    menu: Renderer<Menu>,
}

impl Compose for View {
    fn register(&mut self, children: &mut NodeList) {
        self.top.attr(|a| a.bg = Color::RED);
        children.register_renderer(&self.top);

        self.menu.attr(|a| a.bg = Color::RED);
        children.register_renderer(&self.menu);
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
        self.menu.set(move |w| w.set_list(iter));
    }

    pub fn set_item(&mut self, item: usize) {
        self.menu.set(|w| w.set_item(item));
    }

    pub fn write_top(&mut self, text: &str) {
        self.top.set(|w| write!(w, "{}", text));
    }
}
