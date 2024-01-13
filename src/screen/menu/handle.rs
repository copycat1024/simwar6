use super::Menu;
use soyo::view::{Common, HandleOf};

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

    pub fn set_list<'b, T, S>(&mut self, list: T)
    where
        T: IntoIterator<Item = &'b S>,
        S: AsRef<str> + 'b,
    {
        self.widget.list.handle().set_list(list)
    }

    pub fn item(&mut self) -> usize {
        self.widget.list.handle().item()
    }

    pub fn write_top(&mut self, text: &str) {
        let mut top = self.widget.top.handle();
        write!(top, "{}", text);
    }
}
