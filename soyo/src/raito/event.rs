use crate::gfx::{Event, Key};
use raito::Context;
use std::time::{Duration, Instant};

pub struct EventQueue {
    period: Duration,
    last_tick: Instant,
}

impl EventQueue {
    pub fn new(period: Duration) -> Self {
        Self {
            period,
            last_tick: Instant::now(),
        }
    }

    pub fn event(&mut self, ctx: &Context) -> Option<Event> {
        if let Some(event) = ctx.poll() {
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
            let delta = now.duration_since(self.last_tick);
            if delta > self.period {
                self.last_tick = now;
                Some(Event::Update { delta })
            } else {
                None
            }
        }
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
