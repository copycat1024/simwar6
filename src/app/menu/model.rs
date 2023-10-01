use super::{event::Event, view::View};
use crate::app::{test, ubmp};
use soyo::{
    gfx::{self, Context, Key},
    mvc::{self, App, Flow},
    util::Result,
};

pub const APP_LIST: [&str; 2] = ["Test app", "Unicode plane 0"];

#[derive(Default)]
pub struct Model {
    id: usize,
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
            } else if key == Key::ENTER {
                Some(Event::StartApp)
            } else if key == Key::UP {
                Some(Event::MenuPrev)
            } else if key == Key::DOWN {
                Some(Event::MenuNext)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn reduce(&mut self, event: Self::Event, flow: &mut Flow) -> Option<usize> {
        match event {
            Self::Event::Exit => Some(usize::MAX),
            Self::Event::StartApp => {
                if self.id < APP_LIST.len() {
                    flow.spawn.set()
                }
                None
            }
            Self::Event::MenuNext => {
                if self.id < APP_LIST.len() - 1 {
                    self.id += 1;
                    flow.draw.set();
                }
                None
            }
            Self::Event::MenuPrev => {
                if self.id > 0 {
                    self.id -= 1;
                    flow.draw.set();
                }
                None
            }
        }
    }

    fn spawn(&self, ctx: &mut Context) -> Result {
        match self.id {
            0 => {
                App::default().run::<(), usize, test::Model>(&mut (), ctx)?;
            }
            1 => {
                App::default().run::<(), usize, ubmp::Model>(&mut (), ctx)?;
            }
            _ => {}
        }
        Ok(())
    }

    fn update(&self, view: &mut Self::View) {
        view.set_menu(APP_LIST);
        view.set_item(self.id);

        view.write_top("Launcher");
    }
}
