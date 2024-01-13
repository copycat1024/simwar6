pub use super::Menu;
pub use soyo::util::Latch;

#[derive(Default)]
pub struct MenuCtrl {
    list: Vec<String>,
    item: usize,
}

impl MenuCtrl {
    pub fn update_widget(&self, widget: &mut Menu) {
        widget.set_list(&self.list);
        widget.set_item(self.item);
    }

    pub fn item_up(&mut self) {
        if self.item > 0 {
            self.item -= 1;
        }
    }

    pub fn item_down(&mut self) {
        if self.item < self.list.len() - 1 {
            self.item += 1;
        }
    }

    pub fn set_list<'a, T, S>(&mut self, list: T)
    where
        T: IntoIterator<Item = &'a S>,
        S: AsRef<str> + 'a,
    {
        let list: Vec<String> = list.into_iter().map(|s| s.as_ref().to_owned()).collect();
        if self.list != list {
            self.list = list;
        }
    }

    pub fn item(&self) -> usize {
        self.item
    }
}
