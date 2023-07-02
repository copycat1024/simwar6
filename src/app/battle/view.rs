use super::{Battlefield, Trigger};
use somme::Game;
use soyo::{
    view::{Compose, Composer, Frame, NodeList, Renderer},
    widget::Label,
};

#[derive(Default)]
pub struct View {
    field: Composer<Battlefield>,
    topbar: Renderer<Label>,
}

impl Compose for View {
    fn register(&mut self, children: &mut NodeList) {
        children.register_composer(&self.field);
        children.register_renderer(&self.topbar);
    }

    fn layout(&mut self, frame: &mut Frame) {
        self.topbar.layout(frame.set_h(1));
        self.field.layout(*frame);
    }
}

impl View {
    pub fn update(&mut self, game: &Game) {
        self.topbar.set(|w| write!(w, "Title"));
        self.field.set(|w| w.update_units(game.view().ecs()));
    }

    pub fn trigger(&mut self, trigger: Trigger) {
        self.field.set(|w| w.update_trigger(&trigger));
    }
}
