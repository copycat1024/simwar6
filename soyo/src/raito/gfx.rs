use super::Slot;
use crate::gfx::Color;
use raito::{
    artist::{
        cell_blit::{Cell, TEXTURE_SIZE},
        CellBlit,
    },
    Context, TextureData,
};
use std::collections::HashMap;

pub struct Gfx {
    artist: CellBlit,
    cache: HashMap<(Color, Color), Vec<Cell>>,
}

impl Gfx {
    pub fn new(ctx: &Context) -> Self {
        let data_u8 = include_bytes!("tex0.bin");
        let data = TextureData::from_u8(data_u8, TEXTURE_SIZE.0, TEXTURE_SIZE.1)
            .expect("Wrong texture size");

        let artist = CellBlit::new(ctx, &data);

        Self {
            artist,
            cache: HashMap::new(),
        }
    }

    pub fn push(&mut self, slots: &[Slot]) {
        for slot in slots {
            let vert = self.cache.entry((slot.fg, slot.bg)).or_default();
            vert.push(slot2cell(slot));
        }
    }

    pub fn draw(&mut self, ctx: &Context) {
        ctx.clear();

        let cache = std::mem::take(&mut self.cache);
        for ((fg, bg), vert) in cache.into_iter() {
            let (r, g, b) = fg.rgb();
            self.artist.fg(r, g, b);

            let (r, g, b) = bg.rgb();
            self.artist.bg(r, g, b);

            self.artist.draw(vert);
        }

        ctx.swap();
    }
}

fn slot2cell(slot: &Slot) -> Cell {
    Cell::new(
        slot.x as f32,
        slot.y as f32,
        slot.z as f32,
        if (slot.c as u32) > 0x1F && (slot.c as u32) < 0x7F {
            slot.c
        } else {
            '?'
        } as u8,
    )
}
