use soyo::{
    gfx::{Color, Letter, Rect},
    util::FlexVec,
    view::Render,
};

pub struct Menu {
    item: i32,
    list: FlexVec<FlexVec<char>>,
}

impl Menu {
    fn align(&self, i: i32, pos: Rect) -> i32 {
        let w1 = self.list[i].len();
        let w2 = pos.w;
        (w2 - w1) / 2
    }

    pub fn set_list<'a, T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = &'a str>,
    {
        self.list = FlexVec::from_iter(
            iter.into_iter().map(FlexVec::<char>::text),
            FlexVec::new(' '),
        );
    }

    pub fn set_item(&mut self, item: usize) {
        self.item = item as i32;
    }
}

impl Default for Menu {
    fn default() -> Self {
        Self {
            item: 0,
            list: FlexVec::new(FlexVec::new(' ')),
        }
    }
}

impl Render for Menu {
    fn render_rel(&self, rect: Rect, letter: &mut Letter) {
        let text = &self.list[rect.y];

        letter.c = text[rect.x - self.align(rect.y, rect)];
        if rect.y == self.item {
            letter.bg = Color::BLUE
        };
    }
}
