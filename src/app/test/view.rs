use soyo::{
    view::{Compose, Frame, Host, Renderer, Visitor},
    widget::Label,
};

pub struct View {
    title: Renderer<Label>,
}

impl Compose for View {
    fn propagate<V: Visitor>(&mut self, v: &mut V) {
        self.title.accept_visitor(v);
    }

    fn layout(&mut self, frame: &mut Frame) {
        self.title.layout(frame.set_h(1));
    }
}

impl Default for View {
    fn default() -> Self {
        let mut title = Renderer::<Label>::default();
        write!(title.widget, "Test");
        Self { title }
    }
}
