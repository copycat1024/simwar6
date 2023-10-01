use soyo::{
    gfx::Color,
    view::{Compose, Frame, Host, Renderer, Visitor},
    widget::Label,
};

#[derive(Default)]
pub struct Menu {
    item: usize,
    list: Vec<Renderer<Label>>,
}

impl Menu {
    pub fn set_list<'a, T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = &'a str>,
    {
        self.list = iter
            .into_iter()
            .map(|t| {
                let label = Label::new(t);
                let mut label = Renderer::new(label);
                label.attr.fg = Color::WHITE;
                label.attr.bg = Color::BLACK;
                label
            })
            .collect();
        self.item = 0;
        self.list[0].attr.bg = Color::BLUE;
    }

    pub fn set_item(&mut self, item: usize) {
        self.list[self.item].attr.bg = Color::BLACK;
        self.item = item;
        self.list[self.item].attr.bg = Color::BLUE;
    }
}

impl Compose for Menu {
    fn layout(&mut self, frame: &mut Frame) {
        for (i, label) in self.list.iter_mut().enumerate() {
            label.attr.frame = frame.set_y(frame.y + i as i32).set_h(1);
        }
    }

    fn propagate<V: Visitor>(&mut self, v: &mut V) {
        for label in self.list.iter_mut() {
            label.accept_visitor(v);
        }
    }
}
