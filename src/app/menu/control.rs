use super::{Event, Model};
use crate::screen::Menu;
use soyo::{
    gfx::{self, Key},
    mvc,
    raito::Slot,
    view::Widget,
};

pub const APP_LIST: [&str; 2] = ["Test app", "Unicode plane 0"];

#[derive(Default)]
pub struct Control {}

impl mvc::Control for Control {
    type Frag = Slot;
    type Model = Model;
    type View = Menu;

    fn new(_: &<Self::Model as mvc::Model>::Input) -> (Self::Model, Self::View) {
        (Model::default(), Menu::default())
    }

    fn dispatch(
        event: gfx::Event,
        mut view: <Self::View as Widget>::Handle<'_>,
    ) -> Option<<Self::Model as mvc::Model>::Event> {
        if let gfx::Event::Key { key } = event {
            if key == Key::UP {
                view.item_up();
                None
            } else if key == Key::DOWN {
                view.item_down();
                None
            } else if key == Key::ESC {
                Some(Event::Exit)
            } else if key == Key::ENTER {
                Some(Event::StartApp(view.item()))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn update(mut view: <Self::View as Widget>::Handle<'_>) {
        view.set_list(APP_LIST.iter());
        view.write_top("Launcher");
    }
}
