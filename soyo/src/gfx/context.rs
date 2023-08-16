use crate::{
    gfx::{backend::Backend, Color, Event, Slot},
    util::Result,
};
use std::time::Duration;

#[derive(Clone, Copy)]
struct Config {
    event_period: Duration,
    update_period: Duration,
    clear_bg: Color,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            event_period: Duration::from_millis(5),
            update_period: Duration::from_millis(50),
            clear_bg: Color::BLACK,
        }
    }
}

pub struct Context {
    // external components
    backend: Box<dyn Backend>,

    // internal components
    buffer: Vec<Slot>,
    config: Config,
}

impl Context {
    pub fn new<B: Backend>(backend: B) -> Self {
        Self {
            backend: Box::new(backend),
            buffer: Vec::new(),
            config: Config::default(),
        }
    }

    pub fn event(&mut self) -> Result<Option<Event>> {
        let Config {
            event_period,
            update_period,
            ..
        } = self.config;
        self.backend.event(event_period, update_period)
    }

    pub fn render<I>(&mut self, slots: I)
    where
        I: IntoIterator<Item = Slot>,
    {
        let iter = slots.into_iter().filter(|slot| slot.letter.c != '\0');
        self.buffer.extend(iter);
    }

    pub fn draw(&mut self) -> Result {
        let Self {
            buffer, backend, ..
        } = self;

        for slot in buffer.iter() {
            backend.bg(slot.letter.bg)?;
            backend.fg(slot.letter.fg)?;
            backend.print(&[slot.clone()])?;
        }

        backend.flush()
    }

    pub fn clear(&mut self) -> Result {
        let Self {
            buffer,
            backend,
            config,
            ..
        } = self;

        buffer.clear();
        backend.clear(config.clear_bg)?;
        backend.flush()
    }

    pub fn size(&self) -> (i32, i32) {
        self.backend.size()
    }
}
