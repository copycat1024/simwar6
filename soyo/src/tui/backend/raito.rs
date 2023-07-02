use super::render::Text;
use crate::{
    tui::{Backend, Color, Event, Key},
    util::Result,
};
use raito::{
    artist::{cell_blit::Cell, CellBlit},
    Builder, Context,
};
use std::time::{Duration, Instant};

pub struct Raito {
    ctx: Context,
    artist: CellBlit,
    last_update: Instant,
    x: i32,
    y: i32,
    first: bool,
}

impl Raito {
    pub fn new() -> Self {
        let ctx = Builder::new("Test").build();
        let text = Text::new(include_bytes!("ChivoMono-Light.otf"), 16.);

        let artist = CellBlit::new(&ctx, |pass| {
            for i in 0x20..0x7E_u32 {
                let c = char::from_u32(i).unwrap();
                let offset = i as usize * 16;
                text.render(c, |x, y, c| pass.set(x, y + offset, c));
            }
        });

        Self {
            ctx,
            artist,
            last_update: Instant::now(),
            x: 0,
            y: 0,
            first: true,
        }
    }
}

impl Backend for Raito {
    fn event(&mut self, _event_period: Duration, update_period: Duration) -> Result<Option<Event>> {
        let Self { ctx, first, .. } = self;

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
        } else if *first {
            *first = false;
            Some(Event::Resize { w: 100, h: 40 })
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

    fn print(&mut self, txt: &str) -> Result {
        let vertices = txt
            .chars()
            .enumerate()
            .filter(|(_, c)| (*c as u32) > 0x1F && (*c as u32) < 0x7F)
            .map(|(i, c)| Cell::new(self.x as f32 + i as f32, self.y as f32, 1., c as u8))
            .collect();

        self.artist.draw(vertices);

        Ok(())
    }

    fn gotoxy(&mut self, x: i32, y: i32) -> Result {
        self.x = x;
        self.y = y;
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

    fn clear(&mut self) -> Result {
        self.ctx.clear();
        Ok(())
    }

    fn flush(&mut self) -> Result {
        self.ctx.swap();
        Ok(())
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
