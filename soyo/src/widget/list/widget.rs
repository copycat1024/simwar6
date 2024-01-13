use super::Handle;
use crate::{
    gfx::Color,
    raito::Slot,
    util::Frame,
    view::{Compose, Host, Renderer, Visitor, Widget},
    widget::Label,
};

#[derive(Default)]
pub struct ListView {
    pub(super) item: usize,
    pub(super) list: Vec<String>,
    children: Vec<Renderer<Label>>,
}

impl Widget for ListView {
    type Handle<'a> = Handle<'a>;
}

impl Compose for ListView {
    type Frag = Slot;

    fn compose(&mut self, frame: &mut Frame) {
        let Self {
            item,
            list,
            children,
        } = self;

        if list.len() != children.len() {
            self.children
                .resize_with(list.len(), Renderer::<Label>::default);
        }

        for (i, (text, label)) in list.iter().zip(&mut self.children).enumerate() {
            label.set_frame(frame.set_y(frame.y + i as i32).set_h(1));
            let mut label = label.handle();
            label.set_fg(Color::WHITE);
            if i == *item {
                label.set_bg(Color::BLUE);
            } else {
                label.set_bg(Color::BLACK);
            }
            label.set_text(text.as_ref());
        }
    }

    fn propagate<V: Visitor<Self::Frag>>(&mut self, v: &mut V) {
        for label in self.children.iter_mut() {
            label.accept_visitor(v);
        }
    }
}
