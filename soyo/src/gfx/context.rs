use crate::{
    gfx::{backend::Backend, Color, Event, FrameBuffer, Letter, Quad},
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
    frame: FrameBuffer,
    config: Config,
}

impl Context {
    pub fn new<B: Backend>(backend: B) -> Self {
        Self {
            backend: Box::new(backend),
            frame: FrameBuffer::default(),
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

    pub fn render<F>(&mut self, rect: Quad, z: i32, renderer: F)
    where
        F: Fn(Quad, &mut Letter),
    {
        self.frame.render(rect, z, renderer);
    }

    pub fn draw(&mut self) -> Result {
        let Self { frame, backend, .. } = self;
        frame.draw(backend)
    }

    pub fn clear(&mut self) -> Result {
        let Self { frame, backend, .. } = self;
        frame.clear(backend, self.config.clear_bg)
    }

    pub fn size(&self) -> (i32, i32) {
        self.backend.size()
    }
}
