use super::{Event, Model, View};
use crate::widget::MenuCtrl;
use soyo::{
    gfx::{self, Key},
    mvc::{self, Flow},
};

pub const APP_LIST: [&str; 2] = ["Test app", "Unicode plane 0"];

#[derive(Default)]
pub struct Control {
    menu: MenuCtrl,
}

impl mvc::Control for Control {
    type Model = Model;
    type View = View;

    fn new(_: &<Self::Model as mvc::Model>::Input) -> (Self, Self::Model, Self::View) {
        (Self::default(), Model::default(), View::default())
    }

    fn handle(&mut self, event: gfx::Event, _view: &Self::View) {
        if let gfx::Event::Key { key } = event {
            if key == Key::UP {
                self.menu.item_up();
            } else if key == Key::DOWN {
                self.menu.item_down();
            }
        }
    }

    fn dispatch(
        &mut self,
        event: gfx::Event,
        _view: &Self::View,
    ) -> Option<<Self::Model as mvc::Model>::Event> {
        if let gfx::Event::Key { key } = event {
            if key == Key::ESC {
                Some(Event::Exit)
            } else if key == Key::ENTER {
                Some(Event::StartApp(self.menu.item()))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn cache(&mut self, model: &Self::Model, flow: &mut Flow) {
        self.menu.set_list(APP_LIST.iter());

        flow.draw |= self.menu.hot();
        flow.draw |= model.id.is_some();
    }

    fn update(&self, view: &mut Self::View) {
        self.menu.update_widget(&mut view.menu.widget);

        view.write_top("Launcher");
    }
}
