use super::Label;
use crate::{util::AlignX, view::Renderer};
use std::fmt::{Arguments, Write};

pub trait ILabel {
    fn from_str<S>(text: &S) -> Self
    where
        S: AsRef<str> + ?Sized;
    fn write_fmt(&mut self, fmt: Arguments<'_>);
    fn set_align(&mut self, ha: AlignX);
}

impl ILabel for Renderer<Label> {
    fn from_str<S>(text: &S) -> Self
    where
        S: AsRef<str> + ?Sized,
    {
        let text = text.as_ref().to_owned();
        Self::new(Label {
            len: text.len(),
            text,
            ..Label::default()
        })
    }

    fn write_fmt(&mut self, fmt: Arguments<'_>) {
        self.widget.text.clear();
        write!(self.widget.text, "{}", fmt).unwrap();
        self.widget.len = self.widget.text.chars().count();
    }

    fn set_align(&mut self, align: AlignX) {
        self.widget.align = align
    }
}
