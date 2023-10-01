use super::{event::Event, view::View};
use soyo::{
    gfx::{self, Key},
    mvc::{self, Flow},
};

#[derive(Default)]
pub struct Model {}

impl<I> mvc::Model<I, usize> for Model {
    type Event = Event;
    type View = View;

    fn new(_: &I) -> (Self, Self::View) {
        (Model::default(), View::default())
    }

    fn dispatch(&self, event: gfx::Event, _view: &Self::View) -> Option<Self::Event> {
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

    fn reduce(&mut self, event: Self::Event, _flow: &mut Flow) -> Option<usize> {
        match event {
            Self::Event::Exit => Some(0),
        }
    }

    fn update(&self, _view: &mut Self::View) {}
}
