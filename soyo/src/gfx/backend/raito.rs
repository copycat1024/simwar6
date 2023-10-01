use crate::{
    gfx::{Backend, Color, Event, Key, Slot},
    util::Result,
};
use raito::{
    artist::{
        cell_blit::{Cell, TEXTURE_SIZE},
        CellBlit,
    },
    Builder, Context, TextureData,
};
use std::time::{Duration, Instant};

pub struct Raito {
    ctx: Context,
    artist: CellBlit,
    last_update: Instant,
}

impl Raito {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let ctx = Builder::new("Test").build();

        let data_u8 = include_bytes!("tex0.bin");
        let data = TextureData::from_u8(data_u8, TEXTURE_SIZE.0, TEXTURE_SIZE.1)
            .expect("Wrong texture size");

        let artist = CellBlit::new(&ctx, &data);

        Self {
            ctx,
            artist,
            last_update: Instant::now(),
        }
    }
}

impl Backend for Raito {
    fn event(&mut self, _event_period: Duration, update_period: Duration) -> Result<Option<Event>> {
        let Self { ctx, .. } = self;

        let event = if let Some(event) = ctx.poll() {
            match event {
                raito::Event::Quit { .. } => Some(Event::Key { key: Key::ESC }),
                raito::Event::Key {
                    down: true,
                    keycode,
                    ..
                } => map_key(keycode).map(|key| Event::Key { key }),
                _ => None,
            }
        } else {
            let now = Instant::now();
            let delta = now.duration_since(self.last_update);
            if delta > update_period {
                self.last_update = now;
                Some(Event::Update { delta })
            } else {
                None
            }
        };

        Ok(event)
    }

    fn print(&mut self, slots: &[Slot]) -> Result {
        let vertices = slots
            .iter()
            .map(|slot| {
                let c = slot.c;
                Cell::new(
                    slot.x as f32,
                    slot.y as f32,
                    slot.z as f32,
                    if (c as u32) > 0x1F && (c as u32) < 0x7F {
                        slot.c
                    } else {
                        '?'
                    } as u8,
                )
            })
            .collect();

        self.artist.draw(vertices);

        Ok(())
    }

    fn fg(&mut self, color: Color) -> Result {
        let (r, g, b) = color.rgb();
        self.artist.fg(r, g, b);
        Ok(())
    }

    fn bg(&mut self, color: Color) -> Result {
        let (r, g, b) = color.rgb();
        self.artist.bg(r, g, b);
        Ok(())
    }

    fn clear(&mut self, _color: Color) -> Result {
        self.ctx.clear();
        Ok(())
    }

    fn flush(&mut self) -> Result {
        self.ctx.swap();
        Ok(())
    }

    fn size(&self) -> (i32, i32) {
        (100, 40)
    }
}

fn map_key(code: i32) -> Option<Key> {
    match code {
        0x1b => Some(Key::ESC),
        0x0d => Some(Key::ENTER),

        0x40000052 => Some(Key::UP),
        0x40000051 => Some(Key::DOWN),
        0x40000050 => Some(Key::LEFT),
        0x4000004f => Some(Key::RIGHT),

        code => {
            println!("Unknown key {code:08x}");
            None
        }
    }
}
