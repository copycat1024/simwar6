use super::{event::Event, view::View};
use crate::launcher::APP_LIST;
use soyo::{
    mvc::{self, Flow},
    gfx::{self, Key},
};

#[derive(Default)]
pub struct Model {
    id: usize,
}

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

    fn reduce(&mut self, event: Self::Event, flow: &mut Flow) -> Vec<Self::Trigger> {
        match event {
            Self::Event::Exit => {
                flow.exit(usize::MAX);
            }
            Self::Event::StartApp => {
                flow.exit(self.id + 1);
            }
            Self::Event::MenuNext => {
                if self.id < APP_LIST.len() - 2 {
                    self.id += 1;
                    flow.draw = true;
                }
            }
            Self::Event::MenuPrev => {
                if self.id > 0 {
                    self.id -= 1;
                    flow.draw = true;
                }
            }
        };
        Vec::new()
    }

    fn trigger(&self, _view: &mut Self::View, _trigger: Self::Trigger) {}

    fn update(&self, view: &mut Self::View) {
        view.set_menu(self.app_list());
        view.set_item(self.item());

        view.write_top("Launcher");
    }
}

impl Model {
    pub fn app_list(&self) -> Vec<&str> {
        APP_LIST[1..].iter().map(|i| i.0).collect()
    }

    pub fn item(&self) -> usize {
        self.id
    }
}
