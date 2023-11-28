use crate::{
    gfx::Slot,
    util::AlignX,
    view::{Attribute, Render, Symbol},
};

pub struct Label {
    pub text: String,
    pub align: AlignX,
    pub len: usize,
}

impl Render for Label {
    type Frag = Slot;

    fn render(&self, attr: &Attribute) -> Vec<Slot> {
        let offset = self.align.offset(attr.frame.w, self.len as i32);

        self.text
            .chars()
            .enumerate()
            .skip(offset.child as usize)
            .take(offset.len as usize)
            .map(|(i, c)| Symbol::new(offset.parent + i as i32, 0, c).to_slot(attr))
            .collect()
    }
}

impl Default for Label {
    fn default() -> Self {
        Self {
            text: String::new(),
            align: AlignX::Center,
            len: 0,
        }
    }
}
