use soyo::{
    view::{Compose, Frame, NodeList, Renderer},
    widget::Label,
};

#[derive(Default)]
pub struct View {
    bullet: Renderer<Label>,
}

impl Compose for View {
    fn register(&mut self, children: &mut NodeList) {
        children.register_renderer(&self.bullet);
        self.bullet.set(|w| write!(w, "Test"));
    }

    fn layout(&mut self, frame: &mut Frame) {
        self.bullet.layout(frame.set_h(1));
    }
}
