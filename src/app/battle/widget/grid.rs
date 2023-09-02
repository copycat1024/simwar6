use crate::usym::hbox;
use soyo::{
    gfx::Rect,
    view::{Frame, Render, Renderer, Symbol},
};

pub struct Grid {
    w: i32,
    h: i32,
    sw: i32, // cell w
    sh: i32, // cell h
}

impl Grid {
    pub fn new(w: i32, h: i32, sw: i32, sh: i32) -> Self {
        Self { w, h, sw, sh }
    }

    pub fn get_wh(&self) -> (i32, i32) {
        let Self { w, h, sw, sh } = self;
        (w * (sw + 1) + 1, h * (sh + 1) + 1)
    }

    pub fn get_cell(&self, x: i32, y: i32, frame: Frame) -> Frame {
        let x = x * (self.sw + 1) + 1;
        let y = y * (self.sh + 1) + 1;
        Frame {
            x: frame.x + x,
            y: frame.y + y,
            w: self.sw,
            h: self.sh,
            z: frame.z + 1,
        }
    }

    fn render_intersect(&self, rect: Rect, symbol: &mut Symbol) {
        if rect.x == 0 {
            if rect.y == 0 {
                symbol.c = hbox::CTL;
            } else if rect.y == rect.h - 1 {
                symbol.c = hbox::CBL;
            } else {
                symbol.c = hbox::IVL;
            }
        } else if rect.x == rect.w - 1 {
            if rect.y == 0 {
                symbol.c = hbox::CTR;
            } else if rect.y == rect.h - 1 {
                symbol.c = hbox::CBR;
            } else {
                symbol.c = hbox::IVR;
            }
        } else if rect.y == 0 {
            symbol.c = hbox::IHT;
        } else if rect.y == rect.h - 1 {
            symbol.c = hbox::IHB;
        } else {
            symbol.c = hbox::CRX;
        }
    }

    fn render_horizon_line(&self, symbol: &mut Symbol) {
        symbol.c = hbox::LNH;
    }

    fn render_vertical_line(&self, symbol: &mut Symbol) {
        symbol.c = hbox::LNV;
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            w: 7,
            h: 3,
            sw: 20,
            sh: 12,
        }
    }
}

impl Render for Grid {
    fn render_rel(&self, rect: Rect, symbol: &mut Symbol) {
        let v = rect.x % (self.sw + 1) == 0;
        let h = rect.y % (self.sh + 1) == 0;

        if h && v {
            self.render_intersect(rect, symbol);
        } else if h {
            self.render_horizon_line(symbol);
        } else if v {
            self.render_vertical_line(symbol);
        }
    }
}

pub trait GridRenderer {
    fn get_cell(&self, x: i32, y: i32) -> Frame;
}

impl GridRenderer for Renderer<Grid> {
    fn get_cell(&self, x: i32, y: i32) -> Frame {
        let host = self.ptr.borrow();
        let widget = &host.widget;
        let attr = host.attr;
        widget.get_cell(x, y, attr.frame)
    }
}
