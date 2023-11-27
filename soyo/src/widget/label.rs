use crate::{
    gfx::{Rect, Slot},
    util::HAlign,
    view::{Attribute, Render, Symbol},
};
use std::{
    cmp::min,
    fmt::{Arguments, Write},
};

pub struct Label {
    text: String,
    ha: HAlign,
    len: usize,
}

impl Label {
    pub fn new<S>(text: &S) -> Self
    where
        S: AsRef<str> + ?Sized,
    {
        let mut label = Self::default();
        write!(label, "{}", text.as_ref().to_owned());
        label
    }

    fn align(&self, pos: Rect) -> (i32, usize, usize) {
        let w = pos.w;
        let wi = self.len as i32;

        let len = min(w, wi);

        let coeff = match self.ha {
            HAlign::Center => 1,
            HAlign::Left => 0,
            HAlign::Right => 2,
        };
        let x = (w - len) * coeff / 2;
        let xi = (wi - len) * coeff / 2;

        (x, xi as usize, len as usize)
    }

    pub fn write_fmt(&mut self, fmt: Arguments<'_>) {
        self.text.clear();
        write!(self.text, "{}", fmt).unwrap();
        self.len = self.text.chars().count();
    }

    pub fn set_align(&mut self, ha: HAlign) {
        self.ha = ha
    }
}

impl Render for Label {
    type Frag = Slot;

    fn render(&self, attr: &Attribute) -> Vec<Slot> {
        let (x, xi, len) = self.align(attr.frame.rect());

        self.text
            .chars()
            .enumerate()
            .skip(xi)
            .take(len)
            .map(|(i, c)| Symbol::new(x + i as i32, 0, c).to_slot(attr))
            .collect()
    }
}

impl Default for Label {
    fn default() -> Self {
        Self {
            text: String::new(),
            ha: HAlign::Center,
            len: 0,
        }
    }
}
