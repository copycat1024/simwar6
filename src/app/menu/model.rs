use super::{event::Event, view::View};
use crate::launcher::APP_LIST;
use soyo::{
    gfx::{self, Key},
    mvc::{self, Flow},
};

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
            Self::Event::StartApp => Some(self.id + 1),
            Self::Event::MenuNext => {
                if self.id < APP_LIST.len() {
                    self.id += 1;
                    flow.draw = true;
                }
                None
            }
            Self::Event::MenuPrev => {
                if self.id > 0 {
                    self.id -= 1;
                    flow.draw = true;
                }
                None
            }
        }
    }

    fn update(&self, view: &mut Self::View) {
        view.set_menu(self.app_list());
        view.set_item(self.item());

        view.write_top("Launcher");
    }
}

impl Model {
    pub fn app_list(&self) -> Vec<&str> {
        APP_LIST[1..].to_vec()
    }

    pub fn item(&self) -> usize {
        self.id
    }
}
