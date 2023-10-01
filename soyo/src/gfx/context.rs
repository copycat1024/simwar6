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
    config: Config,
}

impl Context {
    pub fn new<B: Backend>(backend: B) -> Self {
        Self {
            backend: Box::new(backend),
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
        let slots: Vec<Slot> = slots.into_iter().filter(|slot| slot.c != '\0').collect();
        self.backend.push(&slots);
    }

    pub fn draw(&mut self) -> Result {
        self.backend.draw(self.config.clear_bg)
    }

    pub fn size(&self) -> (i32, i32) {
        self.backend.size()
    }
}
