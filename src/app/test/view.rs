use soyo::{
    view::{Compose, Frame, Host, Renderer, Visitor},
    widget::Label,
};

#[derive(Default)]
pub struct View {
    bullet: Renderer<Label>,
}

impl Compose for View {
    fn register(&mut self) {
        write!(self.bullet.widget, "Test");
    }

    fn propagate<V: Visitor>(&mut self, v: &mut V) {
        self.bullet.accept_visitor(v);
    }

    fn layout(&mut self, frame: &mut Frame) {
        self.bullet.layout(frame.set_h(1));
    }
}
