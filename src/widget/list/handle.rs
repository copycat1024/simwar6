use super::ListView;
use soyo::view::{Common, HandleOf};

pub struct Handle<'a> {
    widget: &'a mut ListView,
    _common: &'a mut Common,
}

impl<'a> HandleOf<'a> for Handle<'a> {
    type Widget = ListView;

    fn new(widget: &'a mut Self::Widget, _common: &'a mut Common) -> Self {
        Self { widget, _common }
    }
}

impl<'a> Handle<'a> {
    pub fn set_list<'b, T, S>(&mut self, list: T)
    where
        T: IntoIterator<Item = &'b S>,
        S: AsRef<str> + 'b,
    {
        self.widget.list = list.into_iter().map(|s| s.as_ref().to_owned()).collect();
    }

    pub fn item_up(&mut self) {
        let item = self.item();
        if item > 0 {
            self.set_item(item - 1)
        }
    }

    pub fn item_down(&mut self) {
        let item = self.item();
        if item < self.len() - 1 {
            self.set_item(item + 1);
        }
    }

    pub fn set_item(&mut self, item: usize) {
        self.widget.item = item;
    }

    pub fn item(&self) -> usize {
        self.widget.item
    }

    pub fn len(&self) -> usize {
        self.widget.list.len()
    }
}
