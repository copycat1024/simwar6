use super::event::Event;
use soyo::{mvc, util::Latch};

pub struct Model {
    pub cell: u8,
    pub draw: Latch,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            cell: 0x25,
            draw: Latch::default(),
        }
    }
}

impl mvc::Model for Model {
    type Event = Event;
    type Input = ();
    type Output = usize;

    fn new(_: &Self::Input) -> Self {
        Model::default()
    }

    fn reduce(&mut self, event: Self::Event) -> Option<usize> {
        match event {
            Self::Event::Exit => Some(0),
            Self::Event::Prev => {
                if self.cell > 0 {
                    self.cell -= 1;
                    self.draw.set();
                }
                None
            }
            Self::Event::Next => {
                if self.cell < 255 {
                    self.cell += 1;
                    self.draw.set();
                }
                None
            }
        }
    }
}
