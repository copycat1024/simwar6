use super::Menu;
use soyo::widget::{Common, HandleOf};

pub struct Handle<'a> {
    widget: &'a mut Menu,
    _common: &'a mut Common,
}

impl<'a> HandleOf<'a> for Handle<'a> {
    type Widget = Menu;

    fn new(widget: &'a mut Self::Widget, _common: &'a mut Common) -> Self {
        Self { widget, _common }
    }
}

impl<'a> Handle<'a> {
    pub fn item_up(&mut self) {
        self.widget.list.handle().item_up();
    }

    pub fn item_down(&mut self) {
        self.widget.list.handle().item_down();
    }

    pub fn item(&mut self) -> usize {
        self.widget.list.handle().item()
    }
}
