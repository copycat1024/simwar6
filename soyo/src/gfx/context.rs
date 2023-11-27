use crate::gfx::{Backend, Color, Event, Fragment};
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

pub struct Context<F: Fragment> {
    // external components
    backend: Box<dyn Backend<F>>,

    // internal components
    config: Config,
}

impl<F: Fragment> Context<F> {
    pub fn new<B: Backend<F>>(backend: B) -> Self {
        Self {
            backend: Box::new(backend),
            config: Config::default(),
        }
    }

    pub fn event(&mut self) -> Option<Event> {
        let Config {
            event_period,
            update_period,
            ..
        } = self.config;
        self.backend.event(event_period, update_period)
    }

    pub fn render<I>(&mut self, slots: I)
    where
        I: IntoIterator<Item = F>,
    {
        let slots: Vec<F> = slots.into_iter().collect();
        self.backend.push(&slots);
    }

    pub fn draw(&mut self) {
        self.backend.draw(self.config.clear_bg);
    }

    pub fn size(&self) -> (i32, i32) {
        self.backend.size()
    }
}
