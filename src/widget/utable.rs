use soyo::{
    gfx::{Color, Rect},
    view::{Render, Symbol},
};
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

    fn render_row_title(&self, symbols: &mut Vec<Symbol>) {
        let items = (0..16_i32).flat_map(|i| {
            (0..6_i32).map(move |j| {
                let x = j;
                let y = i + 1;
                let c = match j {
                    1 => 'x',
                    2 => char::from_digit(self.cell as u32 / 16, 16).unwrap(),
                    3 => char::from_digit(self.cell as u32 % 16, 16).unwrap(),
                    4 => char::from_digit(i as u32, 16).unwrap(),
                    _ => '0',
                };
                Symbol::new(x, y, c)
            })
        });
        symbols.extend(items);
    }

    fn render_col_title(&self, symbols: &mut Vec<Symbol>) {
        let items = (0..16_u32).map(|i| {
            let c = char::from_digit(i, 16).unwrap_or('\0');
            let x = i as i32 * 2 + 7;
            Symbol::new(x, 0, c)
        });
        symbols.extend(items);
    }

    fn render_item(&self, symbols: &mut Vec<Symbol>) {
        let items = (0..256_u32).map(|i| {
            let code = self.get_code(i);
            let (c, fg) = map_basic(code);
            let x = (i % 16) as i32 * 2 + 7;
            let y = (i / 16) as i32 + 1;
            Symbol::new(x, y, c).set_fg(fg)
        });
        symbols.extend(items);
    }

    fn get_code(&self, i: u32) -> u32 {
        let cell = self.cell as u32;
        cell * 0x100 + i
    }
}

impl Render for Utable {
    fn render(&self, _rect: Rect) -> Vec<Symbol> {
        let mut symbols = Vec::new();
        self.render_col_title(&mut symbols);
        self.render_row_title(&mut symbols);
        self.render_item(&mut symbols);
        symbols
    }
}

fn map_basic(code: u32) -> (char, Color) {
    if let Some(c) = char::from_u32(code) {
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
