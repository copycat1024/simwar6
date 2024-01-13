use super::Label;
use crate::{
    gfx::Color,
    util::AlignX,
    view::{Common, HandleOf},
};
use std::fmt::Arguments;

pub struct Handle<'a> {
    widget: &'a mut Label,
    common: &'a mut Common,
}

impl<'a> HandleOf<'a> for Handle<'a> {
    type Widget = Label;

    fn new(widget: &'a mut Self::Widget, common: &'a mut Common) -> Self {
        Self { widget, common }
    }
}

impl<'a> Handle<'a> {
    pub fn write_fmt(&mut self, fmt: Arguments<'_>) {
        let text = format!("{}", fmt);
        self.set_text(&text);
    }

    pub fn set_text(&mut self, text: &str) {
        if text != self.widget.text {
            self.widget.text = text.to_owned();
            self.widget.len = self.widget.text.chars().count();
        }
    }

    pub fn set_align(&mut self, align: AlignX) {
        self.widget.align = align;
    }

    pub fn set_fg(&mut self, color: Color) {
        let Self { common, widget } = self;
        common.cmp_and_set(&mut widget.fg, color);
    }

    pub fn set_bg(&mut self, color: Color) {
        let Self { common, widget } = self;
        common.cmp_and_set(&mut widget.bg, color);
    }

    pub fn text(&self) -> &str {
        &self.widget.text
    }
}
