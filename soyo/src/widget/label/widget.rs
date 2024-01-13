use super::handle::Handle;
use crate::{
    raito::Slot,
    util::AlignX,
    view::{Common, Render, Renderer, Symbol, Widget},
};

pub struct Label {
    pub text: String,
    pub align: AlignX,
    pub len: usize,
}

impl Label {
    pub fn new<S>(text: &S) -> Renderer<Self>
    where
        S: AsRef<str> + ?Sized,
    {
        let text = text.as_ref().to_owned();
        Renderer::new(Self {
            len: text.len(),
            text,
            ..Label::default()
        })
    }
}

impl Widget for Label {
    type Handle<'a> = Handle<'a>;
}

impl Render for Label {
    type Frag = Slot;

    fn render(&self, attr: &Common) -> Vec<Slot> {
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
