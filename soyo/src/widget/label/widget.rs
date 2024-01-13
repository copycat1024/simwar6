use super::handle::Handle;
use crate::{
    gfx::Color,
    raito::Slot,
    util::AlignX,
    view::{Common, Render, Renderer, Widget},
};

pub struct Label {
    pub text: String,
    pub align: AlignX,
    pub len: usize,
    pub fg: Color,
    pub bg: Color,
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
            .map(|(i, c)| Slot {
                x: offset.parent + i as i32,
                y: 0,
                z: 0,
                c,
                fg: self.fg,
                bg: self.bg,
            })
            .collect()
    }
}

impl Default for Label {
    fn default() -> Self {
        Self {
            text: String::new(),
            align: AlignX::Center,
            len: 0,
            fg: Color::WHITE,
            bg: Color::BLACK,
        }
    }
}
