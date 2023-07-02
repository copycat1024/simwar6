use super::{event::Event, view::View};
use soyo::{
    mvc::{self, Flow},
    gfx::{self, Key},
};

#[derive(Default)]
pub struct Model {}

impl<T: 'static> mvc::Model<T> for Model {
    type Event = Event;
    type View = View;
    type Trigger = ();

    fn new(_: &T) -> (Self, Self::View) {
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

    fn reduce(&mut self, event: Self::Event, flow: &mut Flow) -> Vec<Self::Trigger> {
        match event {
            Self::Event::Exit => flow.stop = true,
        };
        Vec::new()
    }

    fn trigger(&self, _view: &mut Self::View, _trigger: Self::Trigger) {}

    fn update(&self, _view: &mut Self::View) {}
}
