use super::{event::Event, view::View};
use soyo::{
    gfx::{self, Key},
    mvc::{self, Flow},
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

    fn reduce(&mut self, event: Self::Event, flow: &mut Flow) -> Option<usize> {
        match event {
            Self::Event::Exit => Some(0),
            Self::Event::Prev => {
                if self.cell > 0 {
                    self.cell -= 1;
                    flow.draw.set();
                }
                None
            }
            Self::Event::Next => {
                if self.cell < 255 {
                    self.cell += 1;
                    flow.draw.set();
                }
                None
            }
        }
    }

    fn update(&self, view: &mut Self::View) {
        view.set_cell(self.cell());
    }
}
