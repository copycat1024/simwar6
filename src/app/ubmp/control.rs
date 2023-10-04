use super::{Event, Model, View};
use soyo::{
    gfx::{self, Key},
    mvc::{self, Flow},
};

#[derive(Default)]
pub struct Control {
    cell: u8,
}

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
            } else if key == Key::LEFT {
                Some(Event::Prev)
            } else if key == Key::RIGHT {
                Some(Event::Next)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn cache(&mut self, model: &Self::Model, flow: &mut Flow) {
        self.cell = model.cell;
        flow.draw |= &model.draw;
    }

    fn update(&self, view: &mut Self::View) {
        view.set_cell(self.cell);
    }
}
