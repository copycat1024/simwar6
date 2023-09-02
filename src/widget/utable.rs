use soyo::{
    gfx::{Color, Rect},
    view::{Render, Symbol},
};
use std::char::{from_digit, from_u32};
use unic_ucd_category::GeneralCategory;
use unicode_bidi::bidi_class;

#[derive(Default)]
pub struct Utable {
    cell: u8,
}

impl Utable {
    pub fn set_cell(&mut self, cell: u8) {
        self.cell = cell;
    }

    pub fn get_wh(&self) -> (i32, i32) {
        (6 + 32, 17)
    }

    fn render_row_title(&self, rect: Rect, letter: &mut Symbol) {
        let row = (rect.y - 1) as u32;
        let cell = self.cell as u32;
        letter.c = if rect.y > 0 {
            match rect.x {
                0 => 'U',
                1 => '+',
                2 => from_digit(cell / 16, 16).unwrap_or('\0'),
                3 => from_digit(cell % 16, 16).unwrap_or('\0'),
                4 => from_digit(row % 16, 16).unwrap_or('\0'),
                5 => '0',
                _ => ' ',
            }
        } else {
            ' '
        }
    }

    fn render_col_title(&self, rect: Rect, letter: &mut Symbol) {
        let col = (rect.x - 6) as u32;
        letter.c = if col % 2 == 0 {
            ' '
        } else {
            from_digit(col / 2, 16).unwrap_or('\0')
        }
    }

    fn render_item(&self, rect: Rect, letter: &mut Symbol) {
        letter.c = if rect.x % 2 == 0 {
            ' '
        } else {
            let row = rect.y - 1;
            let col = (rect.x - 6) / 2;
            let code = self.get_code(row as u32, col as u32);

            let (c, fg) = Self::map_basic(code);
            letter.fg = Some(fg);
            c
        }
    }

    fn get_code(&self, row: u32, col: u32) -> u32 {
        let cell = self.cell as u32;
        cell * 0x100 + row * 0x10 + col
    }

    fn map_basic(code: u32) -> (char, Color) {
        if let Some(c) = from_u32(code) {
            use GeneralCategory::*;
            match GeneralCategory::of(c) {
                NonspacingMark | SpacingMark | EnclosingMark => ('M', Color::GREEN),
                SpaceSeparator | LineSeparator | ParagraphSeparator => ('S', Color::GREEN),
                Control => ('C', Color::RED),
                Format => ('F', Color::RED),
                Surrogate => ('S', Color::RED),
                PrivateUse => (c, Color::YELLOW),
                Unassigned => ('U', Color::GRAY),
                _ => {
                    use unicode_bidi::BidiClass::*;
                    match bidi_class(c) {
                        L | EN | ES | ET | CS | ON => {
                            use unicode_width::UnicodeWidthChar;
                            let w = c.width().unwrap_or(0);
                            (
                                c,
                                match w {
                                    1 => Color::WHITE,
                                    2 => Color::BLUE,
                                    3 => Color::YELLOW,
                                    4 => Color::GREEN,
                                    _ => Color::RED,
                                },
                            )
                        }
                        R => ('R', Color::NAVY),
                        AL | AN => (c, Color::OLIVE),
                        B | S | WS => ('N', Color::NAVY),
                        LRE | LRO | RLE | RLO | PDF | LRI | RLI | FSI | PDI => ('X', Color::NAVY),
                        _ => ('E', Color::NAVY),
                    }
                }
            }
        } else {
            ('U', Color::RED)
        }
    }
}

impl Render for Utable {
    fn render_rel(&self, rect: Rect, letter: &mut Symbol) {
        if rect.x < 6 {
            self.render_row_title(rect, letter)
        } else if rect.y < 1 {
            self.render_col_title(rect, letter)
        } else {
            self.render_item(rect, letter)
        }
    }
}
