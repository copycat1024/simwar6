use crate::widget::Utable;
use soyo::{
    view::{Compose, Frame, NodeList, Renderer},
    widget::Label,
};

#[derive(Default)]
pub struct View {
    top: Renderer<Label>,
    table: Renderer<Utable>,
}

impl Compose for View {
    fn register(&mut self, children: &mut NodeList) {
        children.register_renderer(&self.top);
        children.register_renderer(&self.table);
    }

    fn layout(&mut self, frame: &mut Frame) {
        self.top.layout(frame.set_h(1).rise_z());

        let (tw, th) = self.table.get(|view| view.get_wh());
        self.table.layout(frame.center(tw, th).rise_z());
    }
}

impl View {
    pub fn set_cell(&mut self, cell: u8) {
        self.top.set(|w| write!(w, "Cell {}", cell));
        self.table.set(|w| w.set_cell(cell));
    }
}
