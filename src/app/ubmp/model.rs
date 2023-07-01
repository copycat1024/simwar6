use super::{event::Event, view::View};
use soyo::{
    mvc::{self, Flow},
    tui::{self, Key},
};

pub struct Model {
    cell: u8,
}

impl Default for Model {
    fn default() -> Self {
        Self { cell: 0x25 }
    }
}

impl Model {
    pub fn cell(&self) -> u8 {
        self.cell
    }
}

impl<T: 'static> mvc::Model<T> for Model {
    type Event = Event;
    type View = View;
    type Trigger = ();

    fn new(_: &T) -> (Self, Self::View) {
        (Model::default(), View::default())
    }

    fn dispatch(&self, event: tui::Event, _view: &Self::View) -> Option<Self::Event> {
        if let tui::Event::Key { key } = event {
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

    fn reduce(&mut self, event: Self::Event, flow: &mut Flow) -> Vec<Self::Trigger> {
        match event {
            Self::Event::Exit => flow.stop = true,
            Self::Event::Prev => {
                if self.cell > 0 {
                    self.cell -= 1;
                    flow.clear = true;
                    flow.draw = true;
                }
            }
            Self::Event::Next => {
                if self.cell < 255 {
                    self.cell += 1;
                    flow.clear = true;
                    flow.draw = true;
                }
            }
        };
        Vec::new()
    }

    fn trigger(&self, _view: &mut Self::View, _trigger: Self::Trigger) {}

    fn update(&self, view: &mut Self::View) {
        view.set_cell(self.cell());
    }
}
