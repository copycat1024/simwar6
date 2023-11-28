use super::{EventQueue, Gfx, Slot};
use crate::gfx::{self, Event};
use raito::{Builder, Context};
use std::time::Duration;

pub struct Backend {
    ctx: Context,
    event_queue: EventQueue,
    gfx: Gfx,
}

impl Backend {
    #[allow(clippy::new_without_default)]
    pub fn new(period: Duration) -> Self {
        let ctx = Builder::new("Test").build();
        let gfx = Gfx::new(&ctx);

        Self {
            ctx,
            gfx,
            event_queue: EventQueue::new(period),
        }
    }
}

impl gfx::Backend for Backend {
    type Frag = Slot;
    fn event(&mut self) -> Option<Event> {
        let Self {
            event_queue, ctx, ..
        } = self;
        event_queue.event(ctx)
    }

    fn push<I>(&mut self, slots: I)
    where
        I: IntoIterator<Item = Slot>,
    {
        let slots: Vec<Slot> = slots.into_iter().collect();
        self.gfx.push(&slots);
    }

    fn draw(&mut self) {
        let Self { gfx, ctx, .. } = self;
        gfx.draw(ctx);
    }

    fn size(&self) -> (i32, i32) {
        (100, 40)
    }
}
