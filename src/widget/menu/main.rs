use soyo::{
    gfx::Color,
    raito::Slot,
    util::Frame,
    view::{Compose, Host, Renderer, Visitor},
    widget::Label,
};

#[derive(Default)]
pub struct Menu {
    item: usize,
    items: Vec<String>,
    labels: Vec<Renderer<Label>>,
}

impl Menu {
    pub fn set_list<'a, S>(&mut self, input: &[S])
    where
        S: AsRef<str> + 'a,
    {
        self.items = input.iter().map(|s| s.as_ref().to_owned()).collect();
    }

    pub fn set_item(&mut self, item: usize) {
        self.item = item;
    }
}

impl Compose for Menu {
    type Frag = Slot;

    fn compose(&mut self, frame: &mut Frame) {
        let Self {
            items,
            labels,
            item,
        } = self;

        if items.len() != labels.len() {
            self.labels
                .resize_with(items.len(), Renderer::<Label>::default);
        }

        for (i, (text, label)) in items.iter().zip(&mut self.labels).enumerate() {
            label.set_frame(frame.set_y(frame.y + i as i32).set_h(1));
            let mut label = label.handle();
            label.set_fg(Color::WHITE);
            if i == *item {
                label.set_bg(Color::RED);
            } else {
                label.set_bg(Color::BLUE);
            }
            label.set_text(text.as_ref());
        }
    }

    fn propagate<V: Visitor<Self::Frag>>(&mut self, v: &mut V) {
        for label in self.labels.iter_mut() {
            label.accept_visitor(v);
        }
    }
}
