use crate::widget::Utable;
use soyo::{
    view::{Compose, Frame, Host, Renderer, Visitor},
    widget::Label,
};

#[derive(Default)]
pub struct View {
    top: Renderer<Label>,
    table: Renderer<Utable>,
}

impl Compose for View {
    fn register(&mut self) {}

    fn propagate<V: Visitor>(&mut self, v: &mut V) {
        self.top.accept_visitor(v);
        self.table.accept_visitor(v);
    }

    fn layout(&mut self, frame: &mut Frame) {
        self.top.layout(frame.set_h(1).rise_z());

        let (tw, th) = self.table.widget.get_wh();
        self.table.layout(frame.center(tw, th).rise_z());
    }
}

impl View {
    pub fn set_cell(&mut self, cell: u8) {
        write!(self.top.widget, "Cell {}", cell);
        self.table.widget.set_cell(cell);
    }
}
