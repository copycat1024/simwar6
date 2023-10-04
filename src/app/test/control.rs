use super::{Event, Model, View};
use soyo::{
    gfx::{self, Key},
    mvc::{self, Flow},
};

#[derive(Default)]
pub struct Control {}

impl mvc::Control for Control {
    type Model = Model;
    type View = View;

    fn new(_: &<Self::Model as mvc::Model>::Input) -> (Self, Self::Model, Self::View) {
        (Self::default(), Model::default(), View::default())
    }

    fn handle(&mut self, _event: gfx::Event, _view: &Self::View) {}

    fn dispatch(
        &mut self,
        event: gfx::Event,
        _view: &Self::View,
    ) -> Option<<Self::Model as mvc::Model>::Event> {
        if let gfx::Event::Key { key } = event {
            if key == Key::ESC {
                Some(Event::Exit)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn cache(&mut self, _model: &Self::Model, _flow: &mut Flow) {}
    fn update(&self, _view: &mut Self::View) {}
}
